mod register;
mod instructions;
use std::fmt;
use crate::config;
use crate::ram;

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
    pub memory: [u8; 0xffff],
    pub cycle: u32,
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            registers: register::Registers {..Default::default()},
            memory: [0; 0xffff],
            cycle: 0,
        }
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "registers: {}\n\tcycle: {}", self.registers, self.cycle)
    }
}

// only runs if it is below the target cycle in order to maintain timing
pub fn run_cycle(emulator: &mut config::Emulator, target_cycle: u32) {
    if emulator.cpu.cycle < target_cycle {
        run_next_instruction(emulator);
        println!("{}", emulator);
    }
}

// reads next byte in program, increments program counter
pub fn read_program_byte(emulator: &mut config::Emulator) -> u8 {
    let val = ram::read_u8(&mut emulator.cpu.memory, emulator.cpu.registers.pc.into());
    emulator.cpu.registers.pc += 1;
    return val;
}

pub fn read_program_word(emulator: &mut config::Emulator) -> u16 {
    let val = ram::read_u16(&mut emulator.cpu.memory, emulator.cpu.registers.pc.into());
    emulator.cpu.registers.pc += 2;
    return val;
}

fn run_next_instruction(emulator: &mut config::Emulator) {
    // read next byte at the program counter location to get the opcode
    let opcode = ram::read_u8(&mut emulator.cpu.memory, emulator.cpu.registers.pc.into());
    emulator.cpu.registers.pc += 1;

    let mut opcode_iterator = instructions::OPCODES.iter();

    // we unwrap the find here so it crashes if the opcode is invalid, for now
    emulator.cpu.cycle += execute_instruction(emulator, *opcode_iterator.find(|&x| x.1 == opcode).unwrap());
}

// executes the given instruction on the emulator, returns number of cycles it took to complete
fn execute_instruction(emulator: &mut config::Emulator, instruction: (&str, u8, i32, fn(&mut config::Emulator) -> u32)) -> u32 {
    return instruction.3(emulator);
}

fn read_stack_u8(emulator: &mut config::Emulator) -> u8 {
    let result = ram::read_u8(&mut emulator.cpu.memory, (0x0100 + emulator.cpu.registers.sp as u16).into());
	emulator.cpu.registers.sp += 1;
    return result;
}

fn read_stack_u16(emulator: &mut config::Emulator) -> u16 {
    let result = ram::read_u16(&mut emulator.cpu.memory, (0x0100 + emulator.cpu.registers.sp as u16).into());
	emulator.cpu.registers.sp += 2;
    return result;
}

fn write_stack_u8(emulator: &mut config::Emulator, value: u8) {
    ram::write_block(&mut emulator.cpu.memory, (0x0100 + emulator.cpu.registers.sp as u16).into(), &value.to_le_bytes());
	emulator.cpu.registers.sp -= 1;
}

fn write_stack_u16(emulator: &mut config::Emulator, value: u16) {
    ram::write_block(&mut emulator.cpu.memory, (0x0100 + emulator.cpu.registers.sp as u16 - 1).into(), &value.to_le_bytes());
	emulator.cpu.registers.sp -= 2;
}

pub fn power_up(emulator: &mut config::Emulator) {
    emulator.cpu.registers.pc = 0x34;
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
}