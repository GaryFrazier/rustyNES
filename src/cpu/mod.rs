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
    pub registers: register::Registers
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "registers: {}", self.registers)
    }
}

pub fn run_next_instruction(emulator: &mut config::Emulator) {
    // read next byte at the program counter location to get the opcode
    let opcode = emulator.ram.read_u8(emulator.cpu.registers.pc.into());
    emulator.cpu.registers.pc += 1;

    let mut opcode_iterator = instructions::OPCODES.iter();

    // we unwrap the find here so it crashes if the opcode is invalid, for now
    execute_instruction(emulator, *opcode_iterator.find(|&x| x.1 == opcode).unwrap());
}

fn execute_instruction(emulator: &mut config::Emulator, instruction: (&str, u8, i32, fn(&mut config::Emulator))) {
    instruction.3(emulator);
}