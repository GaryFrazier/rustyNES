use crate::config;
use crate::ram;

// mappers determine how rom data is loaded as well as how to switch in data when writes are sent to the rom area (https://www.nesdev.org/wiki/Mapper)
// format is (mapper number, initialize function)
pub static MAPPERS: [(u8, fn(&mut config::Emulator)); 1] = [
    (0, |emulator: &mut config::Emulator| {
        //TODO: MAP TO PPU
        if emulator.rom.header.prg_rom_length == 1 {
            // NROM-128
            ram::write_block(&mut emulator.cpu.memory, 0x8000, &emulator.rom.prg_rom);
            ram::write_block(&mut emulator.cpu.memory, 0xC000, &emulator.rom.prg_rom);
        } else {
            // NROM-256
            ram::write_block(&mut emulator.cpu.memory, 0x8000, &emulator.rom.prg_rom);
        }
    })
];