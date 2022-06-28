mod register;
mod instructions;
use std::fmt;
use crate::config;
use crate::ppu;
use crate::ram;
use crate::ram::AddressingMode;
/* 
    The NES cpu is a modified version of the 6502 processing unit, instructions
    defined here will be reflecting those of the 6502.

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

pub struct CPU {
    pub registers: register::Registers,
    pub memory: [u8; 0x10000],
    pub cycle: u32,
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            registers: register::Registers {..Default::default()},
            memory: [0; 0x10000],
            cycle: 0,
        }
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "registers: {}\n\tcycle: {}", self.registers, self.cycle)
    }
}

pub fn run_cycle(emulator: &mut config::Emulator) {
    if emulator.cpu.cycle == 0 {
        run_next_instruction(emulator);
    }

    if emulator.cpu.cycle > 0 {
        emulator.cpu.cycle -= 1;
    }
}

// reads next byte in program, increments program counter
pub fn read_program_byte(emulator: &mut config::Emulator) -> u8 {
    let val = ram::read_u8(mapped_address, &mut emulator.cpu.memory, emulator.cpu.registers.pc.into());
    emulator.cpu.registers.pc += 1;
    return val;
}

pub fn read_program_word(emulator: &mut config::Emulator) -> u16 {
    let val = ram::read_u16(mapped_address, &mut emulator.cpu.memory, emulator.cpu.registers.pc.into());
    emulator.cpu.registers.pc += 2;
    return val;
}

pub fn mapped_address(addr: usize) -> usize {
    if addr > 0x7ff && addr < 0x2000 {
        return addr & 0x7ff;
    }

    if addr > 0x2007 && addr < 0x4000 {
        return 0x2000 + (addr & 0x7);
    }

    return addr;
}

fn run_next_instruction(emulator: &mut config::Emulator) {
    // read next byte at the program counter location to get the opcode
    let opcode = ram::read_u8(mapped_address, &mut emulator.cpu.memory, emulator.cpu.registers.pc.into());
    emulator.cpu.registers.pc += 1;

    let mut opcode_iterator = instructions::OPCODES.iter();

    // we unwrap the find here so it crashes if the opcode is invalid, for now
    //println!("{:#04x} {:#04x}", emulator.cpu.registers.pc - 1, opcode);

    // for nestest
    let error_code = ram::read_u8(mapped_address, &mut emulator.cpu.memory, 0x2);
    if error_code > 0 {
        println!("error {}", ram::read_u8(mapped_address, &mut emulator.cpu.memory, 0x2)); 
    }
    
    emulator.cpu.cycle += execute_instruction(emulator, *opcode_iterator.find(|&x| x.1 == opcode).unwrap());
}

// executes the given instruction on the emulator, returns number of cycles it took to complete
fn execute_instruction(emulator: &mut config::Emulator, instruction: (&str, u8, i32, fn(&mut config::Emulator) -> u32)) -> u32 {
    return instruction.3(emulator);
}

fn read_stack_u8(emulator: &mut config::Emulator) -> u8 {
    emulator.cpu.registers.sp += 1;
    let result = ram::read_u8(mapped_address, &mut emulator.cpu.memory, (0x0100 + emulator.cpu.registers.sp as u16).into());
    return result;
}

fn read_stack_u16(emulator: &mut config::Emulator) -> u16 {
    let lower = read_stack_u8(emulator);
    let upper = read_stack_u8(emulator);
    
    let result = lower as u16 | ((upper as u16) << 8);
    return result;
}

fn write_stack_u8(emulator: &mut config::Emulator, value: u8) {
    ram::write_block(mapped_address, &mut emulator.cpu.memory, (0x0100 + emulator.cpu.registers.sp as u16).into(), &value.to_le_bytes());
	emulator.cpu.registers.sp -= 1;
}

fn write_stack_u16(emulator: &mut config::Emulator, value: u16) {
    write_stack_u8(emulator, ((value >> 8) & 0xFF) as u8);
    write_stack_u8(emulator, (value & 0xFF) as u8);
}

/*pub fn power_up(emulator: &mut config::Emulator) {
    emulator.cpu.registers.pc = ram::read_u16(&mut emulator.cpu.memory, 0xFFFC); // 0xFFFC is the reset vector
    //emulator.cpu.registers.pc = 0x34;
    emulator.cpu.registers.sp = 0xFD;
    emulator.cpu.registers.a = 0;
    emulator.cpu.registers.x = 0;
    emulator.cpu.registers.y = 0;

    let null_mem = [0; 0x1];
    ram::write_block(&mut emulator.cpu.memory, 0x4017, &null_mem);
    ram::write_block(&mut emulator.cpu.memory, 0x4015, &null_mem);

    let null_mem = [0; 0xF];
    ram::write_block(&mut emulator.cpu.memory, 0x4000, &null_mem);

    let null_mem = [0; 0x3];
    ram::write_block(&mut emulator.cpu.memory, 0x4010, &null_mem);
}*/

pub fn reset(emulator: &mut config::Emulator) {
    emulator.cpu.registers.pc = ram::read_u16(mapped_address, &mut emulator.cpu.memory, 0xFFFC); // 0xFFFC is the reset vector, c000 for nestest
    emulator.cpu.registers.sp = 0xFD;
    emulator.cpu.registers.a = 0;
    emulator.cpu.registers.x = 0;
    emulator.cpu.registers.y = 0;

    let null_mem = [0; 0x1];
    ram::write_block(mapped_address, &mut emulator.cpu.memory, 0x4017, &null_mem);
    ram::write_block(mapped_address, &mut emulator.cpu.memory, 0x4015, &null_mem);

    let null_mem = [0; 0xF];
    ram::write_block(mapped_address, &mut emulator.cpu.memory, 0x4000, &null_mem);

    let null_mem = [0; 0x3];
    ram::write_block(mapped_address, &mut emulator.cpu.memory, 0x4010, &null_mem);

    emulator.cpu.cycle += 8;
}

pub fn irq(emulator: &mut config::Emulator) {
    if !emulator.cpu.registers.status.contains(register::Status::I) {
        write_stack_u16(emulator, emulator.cpu.registers.pc);

        emulator.cpu.registers.status.set(register::Status::B, false);
        //emulator.cpu.registers.status.set(register::Status::U, true);
        emulator.cpu.registers.status.set(register::Status::I, true);

        write_stack_u8(emulator, emulator.cpu.registers.status.bits());
        emulator.cpu.registers.pc = ram::read_u16(mapped_address, &mut emulator.cpu.memory, 0xFFFE);

        emulator.cpu.cycle += 7
    }
    
}

pub fn nmi(emulator: &mut config::Emulator) {
    write_stack_u16(emulator, emulator.cpu.registers.pc);

    emulator.cpu.registers.status.set(register::Status::B, false);
    //emulator.cpu.registers.status.set(register::Status::U, true);
    emulator.cpu.registers.status.set(register::Status::I, true);

    write_stack_u8(emulator, emulator.cpu.registers.status.bits());
    emulator.cpu.registers.pc = ram::read_u16(mapped_address, &mut emulator.cpu.memory, 0xFFFA);

    emulator.cpu.cycle += 8
}

fn handle_ppu_memory_read(emulator: &mut config::Emulator, address: usize) -> Option<u8> {
    let mapped_addr = mapped_address(address);

    match mapped_addr {
        0x2002 => {
            // data = (status.reg & 0xE0) | (ppu_data_buffer & 0x1F); may not need for now
            // Clear the vertical blanking flag
            ppu::set_vblank(emulator, false);
            // Reset Loopy's Address latch flag
            emulator.ppu.ppu_addr_latch = false;
            //return Some(data);
        },
        0x2007 => {
            let mut data: u8;
            // Reads from the NameTable ram get delayed one cycle, 
            // so output buffer which contains the data from the 
            // previous read request
            data = emulator.ppu.ppu_data;
            // then update the buffer for next time
            emulator.ppu.ppu_data = ram::read_u8(mapped_address, &mut emulator.ppu.memory, emulator.ppu.ppu_addr.into());
            // However, if the address was in the palette range, the
            // data is not delayed, so it returns immediately
            if emulator.ppu.ppu_addr >= 0x3F00 {
                data = emulator.ppu.ppu_data
            }
            // All reads from PPU data automatically increment the nametable
            // address depending upon the mode set in the control register.
            // If set to vertical mode, the increment is 32, so it skips
            // one whole nametable row; in horizontal mode it just increments
            // by 1, moving to the next column
            let mut ppu_increment = 1;
            if ppu::get_control_increment_mode(emulator) {
                ppu_increment = 32; 
            }

            emulator.ppu.ppu_addr += ppu_increment;
            return Some(data);
        },
        _ => {

        }
    }

    return None;
}


fn handle_ppu_memory_write(emulator: &mut config::Emulator, address: usize, data: &[u8]) {
    let mapped_addr = mapped_address(address);
    match mapped_addr {
        0x2000 => {
            emulator.ppu.ppu_ctrl = data[0];
        },
        0x2001 => {
            emulator.ppu.ppu_mask = data[0];
        },
        0x2006 => {
            if emulator.ppu.ppu_addr_latch == false {
                emulator.ppu.ppu_addr = (emulator.ppu.ppu_addr & 0xFF00) | data[0] as u16;
                emulator.ppu.ppu_addr_latch = true;
            } else {
                emulator.ppu.ppu_addr = (emulator.ppu.ppu_addr & 0x00FF) | ((data[0] as u16) << 8);
                emulator.ppu.ppu_addr_latch = false;
            }
        },
        0x2007 => {
            ram::write_block(ppu::mapped_address, &mut emulator.ppu.memory, emulator.ppu.ppu_addr.into(), data);
            // All writes from PPU data automatically increment the nametable
            // address depending upon the mode set in the control register.
            // If set to vertical mode, the increment is 32, so it skips
            // one whole nametable row; in horizontal mode it just increments
            // by 1, moving to the next column
            let mut ppu_increment = 1;
            if ppu::get_control_increment_mode(emulator) {
                ppu_increment = 32; 
            }

            emulator.ppu.ppu_addr +=  ppu_increment;
        },
        _ => {

        }
    }
}


// interface for ram
pub fn read_u8(emulator: &mut config::Emulator, addr_mapper: fn(usize)-> usize, address: usize ) -> u8 {
    let ppu_res = handle_ppu_memory_read(emulator, address);

    match ppu_res {
        Some(x) => {
            return x;
        },
        None => {
            return ram::read_u8(addr_mapper, &mut emulator.cpu.memory, address);
        }
    }
}

// return value at address as well as a bool indicating if a page cross happened
pub fn read_with_addressing_mode(emulator: &mut config::Emulator, addr_mapper: fn(usize)-> usize, addressing_mode: AddressingMode) -> (u8, bool) {
    let value: u8;
    let page_cross: bool;

    match addressing_mode {
        AddressingMode::ZeroPage { address } => {
            value = read_u8(emulator, addr_mapper, address.into());
            page_cross = false;
        },
        AddressingMode::ZeroPageX { address, x } => {
            value = read_u8(emulator, addr_mapper, ((address as u16 + x as u16) & 0xFF).into());
            page_cross = false;
        },
        AddressingMode::ZeroPageY { address, y } => {
            value = read_u8(emulator, addr_mapper, ((address as u16 + y as u16) & 0xFF).into());
            page_cross = false;
        },
        AddressingMode::Absolute { address } => {
            value = read_u8(emulator, addr_mapper, address.into());
            page_cross = false;
        },
        AddressingMode::AbsoluteX { address, x } => {
            value = read_u8(emulator, addr_mapper, (address.wrapping_add(x as u16)).into());
            page_cross = address & 0xFF + x as u16 > 0xFF;
        },
        AddressingMode::AbsoluteY { address, y } => {
            value = read_u8(emulator, addr_mapper, (address.wrapping_add(y as u16)).into());
            page_cross = address & 0xFF + y as u16 > 0xFF;
        },
        AddressingMode::IndirectX { address, x } => {
            let calculated_address: u16 = (address as u16).wrapping_add(x as u16);
            let indexed_value = ram::read_u16(addr_mapper, &mut emulator.cpu.memory, calculated_address.into());
            value = read_u8(emulator, addr_mapper, indexed_value.into());
            page_cross = false;
        },
        AddressingMode::IndirectY { address, y } => {
            let indexed_value = ram::read_u16(addr_mapper, &mut emulator.cpu.memory, address.into());
            let calculated_address: u16 = indexed_value.wrapping_add(y as u16);
            value =  read_u8(emulator, addr_mapper, calculated_address.into());
            page_cross = calculated_address > 0xFF;
        },
    }

    return (value, page_cross);
}

pub fn write_block(emulator: &mut config::Emulator, addr_mapper: fn(usize)-> usize, address: usize, data: &[u8]) {
    handle_ppu_memory_write(emulator, address, data);
    ram::write_block(addr_mapper, &mut emulator.cpu.memory, address, data);
}

pub fn write_with_addressing_mode(emulator: &mut config::Emulator, addr_mapper: fn(usize) -> usize, data: &[u8], addressing_mode: AddressingMode) {
    match addressing_mode {
        ram::AddressingMode::ZeroPage { address } => {
            write_block(emulator, addr_mapper, address.into(), data);
        },
        ram::AddressingMode::ZeroPageX { address, x } => {
            write_block(emulator, addr_mapper, ((address as u16 + x as u16) & 0xFF).into(), data);
        },
        ram::AddressingMode::ZeroPageY { address, y } => {
            write_block(emulator, addr_mapper, ((address as u16 + y as u16) & 0xFF).into(), data);
        },
        ram::AddressingMode::Absolute { address } => {
            write_block(emulator, addr_mapper, address.into(), data);
        },
        ram::AddressingMode::AbsoluteX { address, x } => {
            write_block(emulator, addr_mapper, (address + x as u16).into(), data);
        },
        ram::AddressingMode::AbsoluteY { address, y } => {
            write_block(emulator, addr_mapper, (address + y as u16).into(), data);
        },
        ram::AddressingMode::IndirectX { address, x } => {
            let calculated_address: u16 = address as u16 + x as u16;
            let indexed_value = ram::read_u16(addr_mapper, &mut emulator.cpu.memory, calculated_address.into());
            write_block(emulator, addr_mapper, indexed_value.into(), data);
        },
        ram::AddressingMode::IndirectY { address, y } => {
            let indexed_value = ram::read_u16(addr_mapper, &mut emulator.cpu.memory, address.into());
            let calculated_address: u16 = indexed_value + y as u16;
            write_block(emulator, addr_mapper, calculated_address.into(), data);
        },
    }
}