use crate::cpu;
use crate::rom;
use crate::ppu;
use std::fmt;

#[derive(Default)]
pub struct Emulator {
    pub cpu: cpu::CPU,
    pub rom: rom::ROM,
    pub ppu: ppu::PPU,
}

impl fmt::Display for Emulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rom\n\t{}\ncpu:\n\t{}", self.rom, self.cpu)
    }
}