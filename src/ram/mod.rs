// see http://www.emulator101.com/6502-addressing-modes.html for how these work
pub enum AddressingMode {
    ZeroPage { address: u8},
    ZeroPageX { address: u8, x: u8 },
    ZeroPageY { address: u8, y: u8 },
    Absolute { address: u16 },
    AbsoluteX { address: u16, x: u8 },
    AbsoluteY { address: u16, y: u8 },
    IndirectX { address: u8, x: u8 }, // Indexed Indirect
    IndirectY { address: u8, y: u8 } // Indirect Indexed
}

// returns a slice of the memory array
pub fn read(memory: &mut [u8], address: usize , number_of_bytes: usize) -> &[u8] {
    let ending_address = address + (number_of_bytes);
    &memory[address..ending_address]
}

// read byte array to int (little endian)
pub fn read_u8(memory: &mut [u8], address: usize ) -> u8 {
    u8::from_le_bytes(read(memory, address, 1).try_into().expect("tried to parse u8 with incorrect length slice"))
}

pub fn read_u16(memory: &mut [u8], address: usize ) -> u16 {
    u16::from_le_bytes(read(memory, address, 2).try_into().expect("tried to parse u16 with incorrect length slice"))
}

// return value at address as well as a bool indicating if a page cross happened
pub fn read_with_addressing_mode(memory: &mut [u8], addressing_mode: AddressingMode) -> (u8, bool) {
    let value: u8;
    let page_cross: bool;

    match addressing_mode {
        AddressingMode::ZeroPage { address } => {
            value = read_u8(memory, address.into());
            page_cross = false;
        },
        AddressingMode::ZeroPageX { address, x } => {
            value = read_u8(memory, ((address as u16 + x as u16) & 0xFF).into());
            page_cross = false;
        },
        AddressingMode::ZeroPageY { address, y } => {
            value = read_u8(memory, ((address as u16 + y as u16) & 0xFF).into());
            page_cross = false;
        },
        AddressingMode::Absolute { address } => {
            value = read_u8(memory, address.into());
            page_cross = false;
        },
        AddressingMode::AbsoluteX { address, x } => {
            value = read_u8(memory, (address + x as u16).into());
            page_cross = address & 0xFF + x as u16 > 0xFF;
        },
        AddressingMode::AbsoluteY { address, y } => {
            value = read_u8(memory, (address + y as u16).into());
            page_cross = address & 0xFF + y as u16 > 0xFF;
        },
        AddressingMode::IndirectX { address, x } => {
            let calculated_address: u16 = address as u16 + x as u16;
            let indexed_value = read_u16(memory, calculated_address.into());
            value = read_u8(memory, indexed_value.into());
            page_cross = false;
        },
        AddressingMode::IndirectY { address, y } => {
            let indexed_value = read_u16(memory, address.into());
            let calculated_address: u16 = indexed_value + y as u16;
            value =  read_u8(memory, calculated_address.into());
            page_cross = calculated_address > 0xFF;
        },
    }

    return (value, page_cross);
}

pub fn write_block(memory: &mut [u8], address: usize, data: &[u8]) {
    let len = data.len() - 1; // todo: ensure not off by 1
    let mut i: usize = 0;
    while i < len {
        memory[address + i] = data[i];
        i += 1;
    }
}

pub fn write_with_addressing_mode(memory: &mut [u8], data: &[u8], addressing_mode: AddressingMode) {
    match addressing_mode {
        AddressingMode::ZeroPage { address } => {
            write_block(memory, address.into(), data);
        },
        AddressingMode::ZeroPageX { address, x } => {
            write_block(memory, ((address as u16 + x as u16) & 0xFF).into(), data);
        },
        AddressingMode::ZeroPageY { address, y } => {
            write_block(memory, ((address as u16 + y as u16) & 0xFF).into(), data);
        },
        AddressingMode::Absolute { address } => {
            write_block(memory, address.into(), data);
        },
        AddressingMode::AbsoluteX { address, x } => {
            write_block(memory, (address + x as u16).into(), data);
        },
        AddressingMode::AbsoluteY { address, y } => {
            write_block(memory, (address + y as u16).into(), data);
        },
        AddressingMode::IndirectX { address, x } => {
            let calculated_address: u16 = address as u16 + x as u16;
            let indexed_value = read_u16(memory, calculated_address.into());
            write_block(memory, indexed_value.into(), data);
        },
        AddressingMode::IndirectY { address, y } => {
            let indexed_value = read_u16(memory, address.into());
            let calculated_address: u16 = indexed_value + y as u16;
            write_block(memory, calculated_address.into(), data);
        },
    }
}

// returns address as well as if a page change occurred
pub fn relative_offset_page_change(address: u16, offset: i8, ) -> (u16, bool) {
    let page_change = (offset >= 0 && (address as i32 & 0xFF + offset as i32 > 0xFF)) 
        || (offset < 0 && (offset.abs() > (address & 0xFF).try_into().unwrap()));
    let new_address = (address as i32 + offset as i32) as u16;
    return (new_address, page_change);
}