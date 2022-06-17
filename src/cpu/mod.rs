mod register;
mod instructions;
use std::fmt;
use crate::config;

/* 
    The NES cpu is a modified version of the 6502 processing unit, instructions
    defined here will be reflecting those of the 6502.
*/

#[derive(Default)]
pub struct CPU {
    pub registers: register::Registers,
    pub cycle: u32,
    pub wait_cycles: u32
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "registers: {}\n\tcycle: {}\n\twait_cycles: {}", self.registers, self.cycle, self.wait_cycles)
    }
}

// runs one clock cycle on the cpu, if the previous instruction took longer than a cycle
// it sets the wait_cycles property, and this fn will do nothing untill the wait_cycles is decremented to 0
pub fn run_cycle(emulator: &mut config::Emulator) {
    if emulator.cpu.wait_cycles == 0 {
        run_next_instruction(emulator);
    }

    emulator.cpu.wait_cycles -= 1;
}

// reads next byte in program, increments program counter
pub fn read_program_byte(emulator: &mut config::Emulator) -> u8 {
    let val = emulator.ram.read_u8(emulator.cpu.registers.pc.into());
    emulator.cpu.registers.pc += 1;
    return val;
}

fn run_next_instruction(emulator: &mut config::Emulator) {
    // read next byte at the program counter location to get the opcode
    let opcode = emulator.ram.read_u8(emulator.cpu.registers.pc.into());
    emulator.cpu.registers.pc += 1;

    let mut opcode_iterator = instructions::OPCODES.iter();

    // we unwrap the find here so it crashes if the opcode is invalid, for now
    emulator.cpu.wait_cycles = execute_instruction(emulator, *opcode_iterator.find(|&x| x.1 == opcode).unwrap());
}

// executes the given instruction on the emulator, returns number of cycles it took to complete
fn execute_instruction(emulator: &mut config::Emulator, instruction: (&str, u8, i32, fn(&mut config::Emulator) -> u32)) -> u32 {
    return instruction.3(emulator);
}