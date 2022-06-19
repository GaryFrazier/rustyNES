// see structure here https://www.nesdev.org/wiki/INES
pub struct ROM {
    pub header: Header
}

pub struct Header {
    pub nes_title: [u8; 0x4],
    pub prg_rom_length: u8, // in 16 kb blocks
    pub chr_rom_length: u8, // in 8 kb blocks

    // lower 4 bits of flags 6
    pub vertical_mirroring: bool,
    pub persistent_memory: bool,
    pub trainer: bool,
    pub ignore_mirroring: bool,
    
    // mapper number, lower nibble from flags 6 then upper nibble from flag 7 (both are in upper 4 bits dont forget)
    pub mapper: u8,

    // lower 4 bits of flags 7
    pub vs_unisystem: bool,
    pub play_choice_10: bool,
    pub nes_2: bool, // nes 2.0 format, true if bits 3 and 4 equal 2 (0b10?)

    // flags 8, not a flag but alright, it says on site
    pub prg_ram_size: u8,

    // flags 9, rest are reservered to 0
    pub pal_tv: bool,

    // flags 10
    pub tv_system: u8, //TV system (0: NTSC; 2: PAL; 1/3: dual compatible)

    // 2 empty bytes
    pub has_prg_ram: bool,
    pub bus_conflicts: bool,
    
    // 2 more empty bytes
    pub padding: [u8; 0x5],
}