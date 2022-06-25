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
pub fn read(addr_mapper: fn(usize)-> usize, memory: &mut [u8], address: usize , number_of_bytes: usize) -> &[u8] {
    let mapped_addr = addr_mapper(address.try_into().unwrap());
    let ending_address = mapped_addr + (number_of_bytes as usize);
    &memory[mapped_addr.into()..ending_address.into()]
}

// read byte array to int (little endian)
pub fn read_u8(addr_mapper: fn(usize)-> usize, memory: &mut [u8], address: usize ) -> u8 {
    u8::from_le_bytes(read(addr_mapper, memory, address, 1).try_into().expect("tried to parse u8 with incorrect length slice"))
}

pub fn read_u16(addr_mapper: fn(usize)-> usize, memory: &mut [u8], address: usize ) -> u16 {
    u16::from_le_bytes(read(addr_mapper, memory, address, 2).try_into().expect("tried to parse u16 with incorrect length slice"))
}

// return value at address as well as a bool indicating if a page cross happened
pub fn read_with_addressing_mode(addr_mapper: fn(usize)-> usize, memory: &mut [u8], addressing_mode: AddressingMode) -> (u8, bool) {
    let value: u8;
    let page_cross: bool;

    match addressing_mode {
        AddressingMode::ZeroPage { address } => {
            value = read_u8(addr_mapper, memory, address.into());
            page_cross = false;
        },
        AddressingMode::ZeroPageX { address, x } => {
            value = read_u8(addr_mapper, memory, ((address as u16 + x as u16) & 0xFF).into());
            page_cross = false;
        },
        AddressingMode::ZeroPageY { address, y } => {
            value = read_u8(addr_mapper, memory, ((address as u16 + y as u16) & 0xFF).into());
            page_cross = false;
        },
        AddressingMode::Absolute { address } => {
            value = read_u8(addr_mapper, memory, address.into());
            page_cross = false;
        },
        AddressingMode::AbsoluteX { address, x } => {
            value = read_u8(addr_mapper, memory, (address.wrapping_add(x as u16)).into());
            page_cross = address & 0xFF + x as u16 > 0xFF;
        },
        AddressingMode::AbsoluteY { address, y } => {
            value = read_u8(addr_mapper, memory, (address.wrapping_add(y as u16)).into());
            page_cross = address & 0xFF + y as u16 > 0xFF;
        },
        AddressingMode::IndirectX { address, x } => {
            let calculated_address: u16 = (address as u16).wrapping_add(x as u16);
            let indexed_value = read_u16(addr_mapper, memory, calculated_address.into());
            value = read_u8(addr_mapper, memory, indexed_value.into());
            page_cross = false;
        },
        AddressingMode::IndirectY { address, y } => {
            let indexed_value = read_u16(addr_mapper, memory, address.into());
            let calculated_address: u16 = indexed_value.wrapping_add(y as u16);
            value =  read_u8(addr_mapper, memory, calculated_address.into());
            page_cross = calculated_address > 0xFF;
        },
    }

    return (value, page_cross);
}

pub fn write_block(addr_mapper: fn(usize)-> usize, memory: &mut [u8], address: usize, data: &[u8]) {
    let len = data.len(); // todo: ensure not off by 1
    let mut i: usize = 0;
    let mapped_addr = addr_mapper(address.try_into().unwrap());
    while i < len {
        memory[(mapped_addr + i) as usize] = data[i];
        i += 1;
    }
}

// returns address as well as if a page change occurred
pub fn relative_offset_page_change(address: u16, offset: i8) -> (u16, bool) {
    let page_change = (offset >= 0 && (address as i32 & 0xFF + offset as i32 > 0xFF)) 
        || (offset < 0 && (offset.abs() as i32 > (address as i32 & 0xFF)));
    let new_address = (address as i32 + offset as i32) as u16;
    return (new_address, page_change);
}