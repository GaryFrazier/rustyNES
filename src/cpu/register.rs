use bitflags::bitflags;

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
