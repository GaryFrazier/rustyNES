use crate::cpu;
use std::fmt;

#[derive(Default)]
pub struct Emulator {
    pub cpu: cpu::CPU
}

impl fmt::Display for Emulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cpu:\n\t{}", self.cpu)
    }
}