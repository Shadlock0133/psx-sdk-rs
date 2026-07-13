//! Gamepad polling operations
use crate::sys::kernel;
use core::cell::SyncUnsafeCell;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::mem::size_of;
use core::ptr::read_volatile;

// TODO: this is off by a factor of two, but I should double check whether the
// buffers need to be aligned before fixing this. Buffer size in 4-byte words
const BUFFER_SIZE: usize = 68 / size_of::<u32>();

/// `Gamepad` is a reference to a gamepad buffer managed by the BIOS.
///
/// Since the buffer is managed by the BIOS, it must have a stable address while
/// the BIOS can modify it. To ensure this, `Gamepad` references a buffer
/// instead of storing its own. Since the BIOS may only modify the buffer for
/// the lifetime of the `Gamepad`, its destructor must run when it's dropped.
/// Calling `core::mem::forget` on this type or skipping its destructors when
/// it's dropped will lead to **undefined behavior**.
pub struct Gamepad<'a> {
    buf1: *mut u16,
    buf2: *mut u16,
    _buf: PhantomData<&'a mut [u32; BUFFER_SIZE]>,
}

/// This boolean is used to ensure that there's only one Gamepad at a time.
static GAMEPAD_TAKEN: SyncUnsafeCell<bool> = SyncUnsafeCell::new(false);

/// The reading from polling the gamepad buttons
pub struct Buttons {
    value: u16,
}
/// Iterator of pressed buttons
pub struct ButtonsIter {
    value: u16,
    iter_idx: u16,
}
/// The reading from polling a gamepad joystick
pub struct JoyStick(u16);

/// Gamepad buttons
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Button {
    Select = 0,
    L3,
    R3,
    Start,
    Up,
    Right,
    Down,
    Left,
    L2,
    R2,
    L1,
    R1,
    Triangle,
    Circle,
    Cross,
    Square,
}

impl Button {
    fn from_bit(n: u16) -> Option<Self> {
        match n {
            0 => Some(Button::Select),
            1 => Some(Button::L3),
            2 => Some(Button::R3),
            3 => Some(Button::Start),
            4 => Some(Button::Up),
            5 => Some(Button::Right),
            6 => Some(Button::Down),
            7 => Some(Button::Left),
            8 => Some(Button::L2),
            9 => Some(Button::R2),
            10 => Some(Button::L1),
            11 => Some(Button::R1),
            12 => Some(Button::Triangle),
            13 => Some(Button::Circle),
            14 => Some(Button::Cross),
            15 => Some(Button::Square),
            _ => None,
        }
    }
    
    fn to_u16(self) -> u16 {
        match self {
            Button::Select => 0,
            Button::L3 => 1,
            Button::R3 => 2,
            Button::Start => 3,
            Button::Up => 4,
            Button::Right => 5,
            Button::Down => 6,
            Button::Left => 7,
            Button::L2 => 8,
            Button::R2 => 9,
            Button::L1 => 10,
            Button::R1 => 11,
            Button::Triangle => 12,
            Button::Circle => 13,
            Button::Cross => 14,
            Button::Square => 15,
        }
    }
}

#[allow(missing_docs)]
pub mod buttons {
    use super::Button;

    pub const SELECT: Button = Button::Select;
    pub const L3: Button = Button::L3;
    pub const R3: Button = Button::R3;
    pub const START: Button = Button::Start;
    pub const UP: Button = Button::Up;
    pub const RIGHT: Button = Button::Right;
    pub const DOWN: Button = Button::Down;
    pub const LEFT: Button = Button::Left;
    pub const L2: Button = Button::L2;
    pub const R2: Button = Button::R2;
    pub const L1: Button = Button::L1;
    pub const R1: Button = Button::R1;
    pub const TRIANGLE: Button = Button::Triangle;
    pub const CIRCLE: Button = Button::Circle;
    pub const CROSS: Button = Button::Cross;
    pub const SQUARE: Button = Button::Square;
}

impl<'a> Gamepad<'a> {
    /// Creates a new gamepad from a reference to a static buffer.
    ///
    /// Note that the buffer is embedded in the executable. To use a temporary
    /// buffer (e.g. to minimize executable size) use `Gamepad::new_with_buffer`
    /// directly.
    ///
    /// # PANICS
    ///
    /// If this function is called while there's already a gamepad that hasn't
    /// been dropped, then this function will panic.
    #[expect(
        clippy::new_without_default,
        reason = "new implementation not currently safe, Default would encourage its use"
    )]
    pub fn new() -> Self {
        static BUFFER1: SyncUnsafeCell<MaybeUninit<[u32; BUFFER_SIZE]>> =
            SyncUnsafeCell::new(MaybeUninit::uninit());
        static BUFFER2: SyncUnsafeCell<MaybeUninit<[u32; BUFFER_SIZE]>> =
            SyncUnsafeCell::new(MaybeUninit::uninit());
        // SAFETY: If this function's caller drops `Self` without calling its
        // destructors the BIOS will still manage these buffers. However, they have a
        // static lifetime and cannot be access from anywhere else so it's effectively
        // just leaking the buffers.
        unsafe { Self::new_with_buffer_ptr(BUFFER1.get(), BUFFER2.get()) }
    }

    /// Creates a new gamepad from a reference to a buffer.
    ///
    /// `Gamepad::new_with_buffer` can be called by passing in a mutable
    /// reference to a `MaybeUninit::uninit()` and the buffer's size will be
    /// inferred.
    ///
    /// # PANICS
    ///
    /// If this function is called while there's already a gamepad that hasn't
    /// been dropped, then this function will panic.
    ///
    /// # SAFETY
    ///
    /// Since the buffer passed to the BIOS may be dynamic, the BIOS must stop
    /// modifying them when the Gamepad is dropped. This is done by calling
    /// `stop_pad` in `Gamepad`'s destructor. Dropping `Gamepad` without running
    /// its destructor or manuall calling `stop_pad` on the managed buffers will
    /// lead to *undefined behavior*.
    pub unsafe fn new_with_buffer(
        buf1: &'a mut MaybeUninit<[u32; BUFFER_SIZE]>,
        buf2: &'a mut MaybeUninit<[u32; BUFFER_SIZE]>,
    ) -> Self {
        // SAFETY: Provided buffers have 'a lifetime.
        unsafe { Self::new_with_buffer_ptr(buf1, buf2) }
    }

    /// Creates a new gamepad from a pointer to a buffer.
    ///
    /// # PANICS
    ///
    /// If this function is called while there's already a gamepad that hasn't
    /// been dropped, then this function will panic.
    ///
    /// # SAFETY
    ///
    /// The provided buffers must be valid for the entire lifetime 'a.
    /// Since the buffer passed to the BIOS may be dynamic, the BIOS must stop
    /// modifying them when the Gamepad is dropped. This is done by calling
    /// `stop_pad` in `Gamepad`'s destructor. Dropping `Gamepad` without running
    /// its destructor or manuall calling `stop_pad` on the managed buffers will
    /// lead to *undefined behavior*.
    unsafe fn new_with_buffer_ptr(
        buf1: *mut MaybeUninit<[u32; BUFFER_SIZE]>, buf2: *mut MaybeUninit<[u32; BUFFER_SIZE]>,
    ) -> Self {
        if unsafe { *GAMEPAD_TAKEN.get() } {
            panic!(
                "Gamepad's constructor was called while there's already an existing Gamepad handle."
            );
        } else {
            unsafe {
                *GAMEPAD_TAKEN.get() = true;
            }
        }

        let buf1 = buf1.cast::<u16>();
        let buf2 = buf2.cast::<u16>();
        unsafe {
            kernel::psx_init_pad(buf1.cast(), 0x22, buf2.cast(), 0x22);
            // Set the status byte to not ok since init_pad zerofills the buffer
            buf1.cast::<u8>().write_volatile(0xFF);
            // Set all of player 1's buttons to not pressed
            buf1.add(1).write_volatile(0xFFFF);
            // Center player 1's joystick values
            buf1.cast::<u32>().add(1).write_volatile(0x8080_8080);
            kernel::psx_start_pad();
            kernel::psx_change_clear_pad(1);
        }
        Self {
            buf1,
            buf2,
            _buf: PhantomData,
        }
    }

    /// Poll player 1's buttons. This emits a 16-byte volatile read which cannot
    /// be elided.
    pub fn poll_p1(&mut self) -> Buttons {
        Buttons {
            value: unsafe { read_volatile(self.buf1.add(1)) },
        }
    }

    /// Poll player 1's right joystick. This emits a 16-byte volatile read which
    /// cannot be elided.
    pub fn poll_rstick_p1(&mut self) -> JoyStick {
        unsafe { JoyStick(read_volatile(self.buf1.add(2))) }
    }

    /// Poll player 1's left joystick. This emits a 16-byte volatile read which
    /// cannot be elided.
    pub fn poll_lstick_p1(&mut self) -> JoyStick {
        unsafe { JoyStick(read_volatile(self.buf1.add(3))) }
    }

    /// Poll player 2's buttons. This emits a 16-byte volatile read which cannot
    /// be elided.
    pub fn poll_p2(&mut self) -> Buttons {
        Buttons {
            value: unsafe { read_volatile(self.buf2.add(1)) },
        }
    }

    /// Poll player 2's right joystick. This emits a 16-byte volatile read which
    /// cannot be elided.
    pub fn poll_rstick_p2(&mut self) -> JoyStick {
        unsafe { JoyStick(read_volatile(self.buf2.add(2))) }
    }

    /// Poll player 2's left joystick. This emits a 16-byte volatile read which
    /// cannot be elided.
    pub fn poll_lstick_p2(&mut self) -> JoyStick {
        unsafe { JoyStick(read_volatile(self.buf2.add(3))) }
    }
}

impl<'a> Drop for Gamepad<'a> {
    fn drop(&mut self) {
        unsafe {
            kernel::psx_stop_pad();
            *GAMEPAD_TAKEN.get() = false;
        }
    }
}

impl Iterator for ButtonsIter {
    type Item = Button;
    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_idx == 16 {
            return None
        }
        let this_bit = self.iter_idx;
        let bit_value = (self.value >> this_bit) & 1;
        self.iter_idx += 1;
        if bit_value == 0 {
            Button::from_bit(this_bit)
        } else {
            self.next()
        }
    }
}

impl Buttons {
    /// Gives iterator of pressed buttons
    pub fn iter(&self) -> ButtonsIter {
        ButtonsIter { value: self.value, iter_idx: 0 }
    }

    /// Check if `button` was pressed when the `Gamepad` was polled.
    pub fn pressed(&self, button: Button) -> bool {
        let bit = (self.value >> button.to_u16()) & 1;
        bit == 0
    }

    /// Check if `button` was released when the `Gamepad` was polled.
    pub fn released(&self, button: Button) -> bool {
        let bit = (self.value >> button.to_u16()) & 1;
        bit != 0
    }
}

impl JoyStick {
    /// Check the joystick's horizontal offset when the `Gamepad` was polled.
    pub fn horizontal(&self) -> i8 {
        self.0.to_le_bytes()[0].wrapping_sub(0x80).cast_signed()
    }

    /// Check the joystick's vertical offset when the `Gamepad` was polled.
    pub fn vertical(&self) -> i8 {
        self.0.to_le_bytes()[1].wrapping_sub(0x80).cast_signed()
    }
}
