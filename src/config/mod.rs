use crate::cpu;
use crate::rom;
use std::fmt;

#[derive(Default)]
pub struct Emulator {
    pub cpu: cpu::CPU,
    pub rom: rom::ROM
}

impl fmt::Display for Emulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rom\n\t{}\ncpu:\n\t{}", self.rom, self.cpu)
    }
}