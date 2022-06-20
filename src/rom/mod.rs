use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

// see structure here https://www.nesdev.org/wiki/INES
pub struct ROM {
    pub header: Header,
    pub trainer: [u8; 0x200], // only used if trainer header is set
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub playchoice_inst_rom: [u8; 0x2000], // only used if playchoice stuff header is set
    pub playchoice_prom: [u8; 0x20], // only used if playchoice stuff header is set
}

impl Default for ROM {
    fn default() -> ROM {
        ROM {
            header: Header { ..Default::default() },
            trainer: [0; 0x200],
            prg_rom: Vec::new(),
            chr_rom: Vec::new(),
            playchoice_inst_rom: [0; 0x2000],
            playchoice_prom: [0; 0x20],
        }
    }
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
    // pub padding: [u8; 0x5],
}

impl Default for Header {
    fn default() -> Header {
        Header {
            nes_title: [0; 0x4],
            prg_rom_length: 0,
            chr_rom_length: 0,
            vertical_mirroring: false,
            persistent_memory: false,
            trainer: false,
            ignore_mirroring: false,
            mapper: 0,
            vs_unisystem: false,
            play_choice_10: false,
            nes_2: false,
            prg_ram_size: 0,
            pal_tv: false,
            tv_system: 0,
            has_prg_ram: false,
            bus_conflicts: false,
        }
    }
}

impl ROM {
    pub fn load_file(&self, file_name: &String) -> io::Result<()> {
        let f = File::open(file_name)?;
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        
        // Read file into vector.
        reader.read_to_end(&mut buffer)?;
        
        // Read.
        for value in buffer {
            //println!("BYTE: {}", value);
        }

        Ok(())
    }
}