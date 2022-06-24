use bitflags::bitflags;
use std::fmt;

// bitflags
bitflags! {
    #[derive(Default)]
    pub struct Status: u8 {
        const C = 0b00000001; // carry flag
        const Z = 0b00000010; // zero flag
        const I = 0b00000100; // interrupt disable
        const D = 0b00001000; // decimal mode
        const B = 0b00010000; // break command
        const U = 0b00100000; // break command
        const V = 0b01000000; // overflow flag
        const N = 0b10000000; // negative flag
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // cast as u8 to show as 1 or 0, instead of the actual bitflag value
        write!(f, "(c: {}, z: {}, i: {}, d: {}, b: {}, u: {}, v: {}, n: {})\n",
            (self.bits & Status::C.bits == Status::C.bits) as u8,
            (self.bits & Status::Z.bits == Status::Z.bits) as u8,
            (self.bits & Status::I.bits == Status::I.bits) as u8,
            (self.bits & Status::D.bits == Status::D.bits) as u8,
            (self.bits & Status::B.bits == Status::B.bits) as u8,
            (self.bits & Status::U.bits == Status::U.bits) as u8,
            (self.bits & Status::V.bits == Status::V.bits) as u8,
            (self.bits & Status::N.bits == Status::N.bits) as u8)
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