use crate::cpu;
use crate::ram;
use std::fmt;

#[derive(Default)]
pub struct Emulator {
    pub cpu: cpu::CPU,
    pub ram: ram::RAM
}

impl fmt::Display for Emulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cpu:\n\t{}", self.cpu)
    }
}