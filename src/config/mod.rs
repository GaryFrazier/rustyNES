use crate::cpu;
use crate::ram;
use crate::rom;
use std::fmt;

#[derive(Default)]
pub struct Emulator {
    pub cpu: cpu::CPU,
    pub ram: ram::RAM,
    pub rom: rom::ROM
}

impl fmt::Display for Emulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cpu:\n\t{}", self.cpu)
    }
}