use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::fmt;

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

impl fmt::Display for ROM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "header:\n{}", self.header)
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

    // 2 empty bits
    pub has_prg_ram: bool, //(0: present; 1: not present)
    pub bus_conflicts: bool,
    // 2 more empty bits

    // pub padding: [u8; 0x5],
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\t\tprg_rom_length: {}\n\t\tchr_rom_length: {}\n\t\tvertical_mirroring: {}\n\t\tpersistent_memory: {}\n\t\ttrainer: {}\n\t\tignore_mirroring: {}\n\t\tmapper: {}",
            self.prg_rom_length,
            self.chr_rom_length,
            self.vertical_mirroring,
            self.persistent_memory,
            self.trainer,
            self.ignore_mirroring,
            self.mapper)
    }
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
    pub fn load_file(&mut self, file_name: &String) -> io::Result<()> {
        let f = File::open(file_name)?;
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        
        // Read file into vector.
        reader.read_to_end(&mut buffer)?;
        
        // Read header to determine how to load rest of file.
        self.load_header(&buffer);
        for value in buffer {
            //println!("BYTE: {}", value);
        }

        Ok(())
    }

    fn load_header(&mut self, buffer: &Vec<u8>) {
        self.header.nes_title.copy_from_slice(&buffer[0..=3]); 
        self.header.prg_rom_length = buffer[4];
        self.header.chr_rom_length = buffer[5];

        let flags6: u8 = buffer[6];
        let flags7: u8 = buffer[7];
        let flags8: u8 = buffer[8];
        let flags9: u8 = buffer[9];
        let flags10: u8 = buffer[10];

        self.header.mapper = ((flags6 & 0xF0) >> 4) | (flags7 & 0xF0);

        self.header.vertical_mirroring  = flags6 & 0x1 == 0x1;
        self.header.persistent_memory  = flags6 & 0x2 == 0x2;
        self.header.trainer  = flags6 & 0x4 == 0x4;
        self.header.ignore_mirroring  = flags6 & 0x8 == 0x8;

        self.header.vs_unisystem  = flags7 & 0x1 == 0x1;
        self.header.play_choice_10  = flags7 & 0x2 == 0x2;
        self.header.nes_2  = flags7 & 0x4 == 0x0 && flags7 & 0x8 == 0x8; // equal to 2

        self.header.prg_ram_size = flags8;
        
        self.header.pal_tv = flags9 & 0x1 == 0x1;

        self.header.tv_system = (flags10 & 0x1) | ((flags10 & 0x2) << 1);
        self.header.has_prg_ram = flags10 & 0x10 == 0x0;
        self.header.bus_conflicts = flags10 & 0x20 == 0x20;
    }
}