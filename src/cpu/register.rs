use bitflags::bitflags;
use std::fmt;

// bitflags
bitflags! {
    #[derive(Default)]
    pub struct Status: u8 {
        const C = 0b0000001; // carry flag
        const Z = 0b0000010; // zero flag
        const I = 0b0000100; // interrupt disable
        const D = 0b0001000; // decimal mode
        const B = 0b0010000; // break command
        const V = 0b0100000; // overflow flag
        const N = 0b1000000; // negative flag
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(c: {}, z: {}, i: {}, d: {}, b: {}, v: {}, n: {})\n",
            self.bits & Status::C.bits,
            self.bits & Status::Z.bits,
            self.bits & Status::I.bits,
            self.bits & Status::D.bits,
            self.bits & Status::B.bits,
            self.bits & Status::V.bits,
            self.bits & Status::N.bits)
    }
}

// registers
#[derive(Default)]
pub struct Registers {
    pub a: u8, // accumulator
    pub x: u8,
    pub y: u8,
    pub sp: u8, // stack pointer, points to current stack address in memory
    pub pc: u16, // program counter, points to next instruction address in memory
    pub status: Status // status flags
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\t\ta: {}\n\t\tx: {}\n\t\ty: {}\n\t\tsp: {}\n\t\tpc: {}\n\t\tstatus: {}"
        , self.a, self.x, self.y, self.sp, self.pc, self.status)
    }
}