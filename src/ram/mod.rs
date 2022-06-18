use std::fmt;

/*
    Memory map of the 6502 is as follows, see https://www.nesdev.org/wiki/CPU_memory_map for more info:
    $0000-$07FF	$0800	2KB internal RAM
    $0800-$0FFF	$0800	Mirrors of $0000-$07FF
    $1000-$17FF	$0800
    $1800-$1FFF	$0800
    $2000-$2007	$0008	NES PPU registers
    $2008-$3FFF	$1FF8	Mirrors of $2000-2007 (repeats every 8 bytes)
    $4000-$4017	$0018	NES APU and I/O registers
    $4018-$401F	$0008	APU and I/O functionality that is normally disabled. See CPU Test Mode.
    $4020-$FFFF	$BFE0	Cartridge space: PRG ROM, PRG RAM, and mapper registers

*/

pub struct RAM {
    pub memory: [u8; 0xffff]
}

// see http://www.emulator101.com/6502-addressing-modes.html for how these work
pub enum AddressingMode {
    ZeroPage { address: u8},
    ZeroPageX { address: u8, x: u8 },
    Absolute { address: u16 },
    AbsoluteX { address: u16, x: u8 },
    AbsoluteY { address: u16, y: u8 },
    IndirectX { address: u8, x: u8 }, // Indexed Indirect
    IndirectY { address: u8, y: u8 } // Indirect Indexed
}

// set default ram here for debug purposes
impl Default for RAM {
    fn default() -> RAM {
        RAM {
            memory: [0; 0xffff],
        }
    }
}

impl fmt::Display for RAM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl RAM {
    // returns a slice of the memory array
    pub fn read(&self, address: usize , number_of_bytes: usize) -> &[u8] {
        let ending_address = address + (number_of_bytes);
        &self.memory[address..ending_address]
    }

    // read byte array to int (little endian)
    pub fn read_u8(&self, address: usize ) -> u8 {
        u8::from_le_bytes(self.read(address, 1).try_into().expect("tried to parse u8 with incorrect length slice"))
    }

    pub fn read_u16(&self, address: usize ) -> u16 {
        u16::from_le_bytes(self.read(address, 2).try_into().expect("tried to parse u16 with incorrect length slice"))
    }

    // return value at address as well as a bool indicating if a page cross happened
    pub fn read_with_addressing_mode(&self, addressing_mode: AddressingMode) -> (u8, bool) {
        let value: u8;
        let page_cross: bool;

        match addressing_mode {
            AddressingMode::ZeroPage { address } => {
                value = self.read_u8(address.into());
                page_cross = false;
            },
            AddressingMode::ZeroPageX { address, x } => {
                value = self.read_u8(((address as u16 + x as u16) & 0xFF).into());
                page_cross = false;
            },
            AddressingMode::Absolute { address } => {
                value = self.read_u8(address.into());
                page_cross = false;
            },
            AddressingMode::AbsoluteX { address, x } => {
                value = self.read_u8((address + x as u16).into());
                page_cross = address & 0xFF + x as u16 > 0xFF;
            },
            AddressingMode::AbsoluteY { address, y } => {
                value = self.read_u8((address + y as u16).into());
                page_cross = address & 0xFF + y as u16 > 0xFF;
            },
            AddressingMode::IndirectX { address, x } => {
                let calculated_address: u16 = address as u16 + x as u16;
                let indexed_value = self.read_u16(calculated_address.into());
                value = self.read_u8(indexed_value.into());
                page_cross = false;
            },
            AddressingMode::IndirectY { address, y } => {
                let indexed_value = self.read_u16(address.into());
                let calculated_address: u16 = indexed_value + y as u16;
                value = self.read_u8(indexed_value.into());
                page_cross = calculated_address > 0xFF;
            },
        }

        return (value, page_cross);
    }
}