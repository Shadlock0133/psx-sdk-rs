//! Geometry Transformation Engine Coprocessor
//!
//! This module provides access to GTE, or cop2, registers and instructions.

use core::{cell::SyncUnsafeCell, marker::PhantomData};

use crate::{hw::{Register, cop0},
            math::{fx16, fx32}};

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
/// MVMVA command opcode
pub const MVMVA_CMD: u32 = (0x04 << 20) | 0x12;
/// SQR command opcode
pub const SQR_CMD: u32 = (0x0a << 20) | 0x28;
/// OP command opcode
pub const OP_CMD: u32 = (0x17 << 20) | 0x0c;
/// NCS command opcode
pub const NCS_CMD: u32 = (0x0c << 20) | SF | 0x1e;
/// NCT command opcode
pub const NCT_CMD: u32 = (0x0d << 20) | SF | 0x20;
/// NCCS command opcode
pub const NCCS_CMD: u32 = (0x10 << 20) | SF | 0x1b;
/// NCCT command opcode
pub const NCCT_CMD: u32 = (0x11 << 20) | SF | 0x3f;
/// NCDS command opcode
pub const NCDS_CMD: u32 = (0x0e << 20) | SF | 0x13;
/// NCDT command opcode
pub const NCDT_CMD: u32 = (0x0f << 20) | SF | 0x16;
/// CC command opcode
pub const CC_CMD: u32 = (0x13 << 20) | SF | 0x1c;
/// CDP command opcode
pub const CDP_CMD: u32 = (0x12 << 20) | SF | 0x14;
/// DCPL command opcode
pub const DCPL_CMD: u32 = (0x06 << 20) | SF | 0x29;
/// DPCS command opcode
pub const DPCS_CMD: u32 = (0x07 << 20) | SF | 0x10;
/// DPCT command opcode
pub const DPCT_CMD: u32 = (0x08 << 20) | SF | 0x2a;
/// INTPL command opcode
pub const INTPL_CMD: u32 = (0x09 << 20) | SF | 0x11;
/// GPF command opcode
pub const GPF_CMD: u32 = (0x19 << 20) | 0x3d;
/// GPL command opcode
pub const GPL_CMD: u32 = (0x1a << 20) | 0x3e;

/// WIP
/// Type-safe interface to GTE
pub struct Gte(());

impl Gte {
    /// Take access to the GTE, allowing for only one instance
    pub fn take() -> Option<Self> {
        static GTE_TAKEN: SyncUnsafeCell<bool> = SyncUnsafeCell::new(false);

        let ptr = GTE_TAKEN.get();
        let value = unsafe { ptr.read() };
        unsafe { ptr.write(true) };
        (!value).then(|| {
            cop0::Status::new().enable_gte().store();
            Self(())
        })
    }

    /// Loads values into V{X,Y,Z}{0,1,2} registers
    pub fn load_v012(&self, v: [[i16; 3]; 3]) {
        let [(vxy0, vz0), (vxy1, vz1), (vxy2, vz2)] = v.map(|row| {
            let [x, y, z] = row.map(|n| n.cast_unsigned());
            (u32::from(x) | (u32::from(y) << 16), z.cast_signed())
        });

        VXY0::skip_load().assign(vxy0).store();
        VZ0::skip_load().assign(vz0).store();
        VXY1::skip_load().assign(vxy1).store();
        VZ1::skip_load().assign(vz1).store();
        VXY2::skip_load().assign(vxy2).store();
        VZ2::skip_load().assign(vz2).store();
    }

    /// Loads values into V{X,Y,Z}0 registers
    pub fn load_v0(&self, v: [i16; 3]) {
        let [x, y, z] = v.map(|n| n.cast_unsigned());
        let (vxy, vz) = (u32::from(x) | (u32::from(y) << 16), z.cast_signed());

        VXY0::skip_load().assign(vxy).store();
        VZ0::skip_load().assign(vz).store();
    }

    /// Loads values into input registers of RTP{S,T} instructions,
    /// excluding V registers
    pub fn load_rtpx_params(&self, input: RtpxParams) {
        let [ofx, ofy] = input.of.map(|x| x.to_bits().cast_unsigned());

        let rot = input
            .rot
            .map(|row| row.map(|x| u32::from(x.to_bits().cast_unsigned())));
        let rot11_12 = rot[0][0] | rot[0][1] << 16;
        let rt13_21 = rot[0][2] | rot[1][0] << 16;
        let rt22_23 = rot[1][1] | rot[1][2] << 16;
        let rt31_32 = rot[2][0] | rot[2][1] << 16;
        let rot33 = input.rot[2][2].to_bits();

        RT11_12::skip_load().assign(rot11_12).store();
        RT13_21::skip_load().assign(rt13_21).store();
        RT22_23::skip_load().assign(rt22_23).store();
        RT31_32::skip_load().assign(rt31_32).store();
        RT33::skip_load().assign(rot33).store();
        TRX::skip_load().assign(input.tr[0]).store();
        TRY::skip_load().assign(input.tr[1]).store();
        TRZ::skip_load().assign(input.tr[2]).store();
        OFX::skip_load().assign(ofx).store();
        OFY::skip_load().assign(ofy).store();
        H::skip_load().assign(input.h).store();
        DQA::skip_load().assign(input.dqa.to_bits()).store();
        DQB::skip_load()
            .assign(input.dqb.to_bits().cast_unsigned())
            .store();
    }

    /// Call RTPT instruction
    pub fn rtpt<'a>(&'a self) -> Rtpt<'a> {
        unsafe {
            cop2::<{ RTPT_CMD }>();
        }
        Rtpt(PhantomData)
    }

    /// Call RTPS instruction
    pub fn rtps<'a>(&'a self) -> Rtps<'a> {
        unsafe {
            cop2::<{ RTPS_CMD }>();
        }
        Rtps(PhantomData)
    }
}

/// Parameters for RTP{S,T} instructions
pub struct RtpxParams {
    /// Rotation matrix
    pub rot: [[fx16<12>; 3]; 3],
    /// Translation vector
    pub tr: [i32; 3],
    /// Offset added to projected 2d vertices
    pub of: [fx32<16>; 2],
    /// Plane clipping value
    pub h: u16,
    /// Depth quotient(?)
    pub dqa: fx16<8>,
    /// Depth quotient(?)
    pub dqb: fx32<24>,
}

/// Running RTPT instruction
pub struct Rtpt<'a>(PhantomData<&'a Gte>);
/// Output of the RTPT instruction
pub struct RtptOutput {
    /// 3 vertices
    pub sxy: [[i16; 2]; 3],
}

impl Rtpt<'_> {
    /// Read output registers, blocking if GTE os busy
    pub fn get_output(self) -> RtptOutput {
        let sxy0 = SXY0::new().to_bits();
        let sxy1 = SXY1::new().to_bits();
        let sxy2 = SXY2::new().to_bits();

        let sxy = [sxy0, sxy1, sxy2].map(|xy| {
            let x = u16::try_from(xy & 0xffff).unwrap().cast_signed();
            let y = u16::try_from(xy >> 16).unwrap().cast_signed();
            [x, y]
        });

        RtptOutput { sxy }
    }
}

/// Running RTPS instruction
pub struct Rtps<'a>(PhantomData<&'a Gte>);
/// Output of the RTPS instruction
pub struct RtpsOutput {
    /// Vertex
    pub sxy: [i16; 2],
}

impl Rtps<'_> {
    /// Read output registers, blocking if GTE os busy
    pub fn get_output(self) -> RtpsOutput {
        let sxy2 = SXY2::new().to_bits();

        let [sxy] = [sxy2].map(|xy| {
            let x = u16::try_from(xy & 0xffff).unwrap().cast_signed();
            let y = u16::try_from(xy >> 16).unwrap().cast_signed();
            [x, y]
        });

        RtpsOutput { sxy }
    }
}
