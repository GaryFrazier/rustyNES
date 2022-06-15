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
    // read pc, get next inst
    execute_instruction(emulator, instructions::OPCODES[0]);
}

fn execute_instruction(emulator: &mut config::Emulator, instruction: (&str, i32, i32, fn(&mut config::Emulator))) {
    instruction.3(emulator);
}