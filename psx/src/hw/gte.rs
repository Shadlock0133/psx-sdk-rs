//! Geometry Transformation Engine Coprocessor
//!
//! This module provides access to GTE, or cop2, registers and instructions.

use crate::hw::Register;

define_cop! {
    /// The 16-bit VX0 and VY0 vectors
    VXY0<u32>; COP: 2; R: 0,
    /// The 16-bit VZ0 vector
    VZ0<i16>;  COP: 2; R: 1,
    /// The 16-bit VX1 and VY1 vectors
    VXY1<u32>; COP: 2; R: 2,
    /// The 16-bit VZ1 vector
    VZ1<i16>;  COP: 2; R: 3,
    /// The 16-bit VX2 and VY2 vectors
    VXY2<u32>; COP: 2; R: 4,
    /// The 16-bit VZ2 vector
    VZ2<i16>;  COP: 2; R: 5,
    /// RGB value
    RGB<u32>; COP: 2; R: 6,
    /// Ordering table average Z value
    OTZ<u16>;  COP: 2; R: 7,

    /// Intermediate value 0
    IR0<u32>; COP: 2; R: 8,
    /// Intermediate value 1
    IR1<u32>; COP: 2; R: 9,
    /// Intermediate value 2
    IR2<u32>; COP: 2; R: 10,
    /// Intermediate value 3
    IR3<u32>; COP: 2; R: 11,

    /// Screen XY coord FIFO 0
    SXY0<u32>; COP: 2; R: 12,
    /// Screen XY coord FIFO 1
    SXY1<u32>; COP: 2; R: 13,
    /// Screen XY coord FIFO 2
    SXY2<u32>; COP: 2; R: 14,
    /// Screen XY coord FIFO P
    SXYP<u32>; COP: 2; R: 15,

    /// Screen Z FIFO 0
    SZ0<u16>; COP: 2; R: 16,
    /// Screen Z FIFO 1
    SZ1<u16>; COP: 2; R: 17,
    /// Screen Z FIFO 2
    SZ2<u16>; COP: 2; R: 18,
    /// Screen Z FIFO 3
    SZ3<u16>; COP: 2; R: 19,

    /// Characteristic color FIFO 0
    RGB0<u32>; COP: 2; R: 20,
    /// Characteristic color FIFO 1
    RGB1<u32>; COP: 2; R: 21,
    /// Characteristic color FIFO 2
    RGB2<u32>; COP: 2; R: 22,

    /// Unused / Prohibited
    RES1<u32>; COP: 2; R: 23,

    /// Scalar math accumulator
    MAC0<i32>; COP: 2; R: 24,

    /// The first component of the vector math accumulator
    MAC1<i32>; COP: 2; R: 25,
    /// The second component of the vector math accumulator
    MAC2<i32>; COP: 2; R: 26,
    /// The third component of the vector math accumulator
    MAC3<i32>; COP: 2; R: 27,

    /// IRGB
    IRGB<u16>;  COP: 2; R: 28,
    /// ORGB
    ORGB<u16>;  COP: 2; R: 29,

    /// Leading zeros count source
    LZCS<u32>; COP: 2; R: 30,
    /// Leading zeros count result
    LZCR<u32>; COP: 2; R: 31,

    /// Rotation matrix entries RT11 and RT12
    RT11_12<u32>; COP: 2; R: 32; c,

    /// Rotation matrix entries RT13 and RT21
    RT13_21<u32>; COP: 2; R: 33; c,
    /// Rotation matrix entries RT22 and RT23
    RT22_23<u32>; COP: 2; R: 34; c,
    /// Rotation matrix entries RT31 and RT32
    RT31_32<u32>; COP: 2; R: 35; c,
    /// Rotation matrix entry RT33
    RT33<i16>;    COP: 2; R: 36; c,

    /// Translation vector X
    TRX<i32>; COP: 2; R: 37; c,
    /// Translation vector Y
    TRY<i32>; COP: 2; R: 38; c,
    /// Translation vector Z
    TRZ<i32>; COP: 2; R: 39; c,

    /// Light matrix entries L11 and L12
    L11_12<u32>; COP: 2; R: 40; c,
    /// Light matrix entries L13 and L21
    L13_21<u32>; COP: 2; R: 41; c,
    /// Light matrix entries L22 and L23
    L22_23<u32>; COP: 2; R: 42; c,
    /// Light matrix entries L31 and L32
    L31_32<u32>; COP: 2; R: 43; c,
    /// Light matrix entry L33
    L33<i16>;    COP: 2; R: 44; c,

    /// Background color red component
    RBK<u32>; COP: 2; R: 45; c,
    /// Background color green component
    GBK<u32>; COP: 2; R: 46; c,
    /// Background color blue component
    BBK<u32>; COP: 2; R: 47; c,

    /// Light color matrix entries LR11 and LR12
    LR11_12<u32>; COP: 2; R: 48; c,
    /// Light color matrix entries LR13 and LR21
    LR13_21<u32>; COP: 2; R: 49; c,
    /// Light color matrix entries LR22 and LR23
    LR22_23<u32>; COP: 2; R: 50; c,
    /// Light color matrix entries LR31 and LR32
    LR31_32<u32>; COP: 2; R: 51; c,
    /// Light color matrix entry LR33
    LR33<i16>;    COP: 2; R: 52; c,

    /// Far color red component
    RFC<u32>; COP: 2; R: 53; c,
    /// Far color green component
    GFC<u32>; COP: 2; R: 54; c,
    /// Far color blue component
    BFC<u32>; COP: 2; R: 55; c,

    /// Screen Offset and Distance X
    OFX<u32>; COP: 2; R: 56; c,
    /// Screen Offset and Distance Y
    OFY<u32>; COP: 2; R: 57; c,
    /// Projection plane distance
    H<u16>;   COP: 2; R: 58; c,

    /// Depth queuing parameter A. (coefficient)
    DQA<i16>; COP: 2; R: 59; c,
    /// Depth queuing parameter B. (offset)
    DQB<u32>; COP: 2; R: 60; c,

    /// Z3 average scale factor (normally 1/3)
    ZSF3<u32>; COP: 2; R: 61; c,
    /// Z4 average scale factor (normally 1/4)
    ZSF4<u32>; COP: 2; R: 62; c,

    /// Error code status
    FLAG<u32>; COP: 2; R: 63; c,
}

/// Call cop2 instruction
/// # Safety
/// GTE must be enabled
/// Read GTE docs on how to use it safely
#[inline(always)]
pub unsafe fn cop2<const CMD: u32>() {
    const { assert!(CMD & 0xfe000000 == 0) };
    unsafe {
        core::arch::asm!(
            ".long (0b0100101 << 25) | {} # cop2",
            const CMD,
            options(nostack),
        );
    }
}

const SF: u32 = 1 << 19;
const LM: u32 = 1 << 10;
const MVMVA_LIGHT: u32 = 1 << 17;
const MVMVA_COLOR: u32 = 2 << 17;
const MVMVA_V1: u32 = 1 << 15;
const MVMVA_V2: u32 = 2 << 15;
const MVMVA_BK: u32 = 1 << 13;

/// RTPS command opcode
pub const RTPS_CMD: u32 = (0x01 << 20) | SF | 0x01;
/// RTPT command opcode
pub const RTPT_CMD: u32 = (0x02 << 20) | SF | 0x30;
/// NCLIP command opcode
pub const NCLIP_CMD: u32 = (0x14 << 20) | 0x06;
/// AVSZ3 command opcode
pub const AVSZ3_CMD: u32 = (0x15 << 20) | SF | 0x2d;
/// AVSZ4 command opcode
pub const AVSZ4_CMD: u32 = (0x16 << 20) | SF | 0x2e;
