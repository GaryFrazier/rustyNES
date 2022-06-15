mod register;
use std::fmt;

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