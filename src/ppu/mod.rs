use std::fmt;
use crate::config;
/* 
    ppu memory map https://www.nesdev.org/wiki/PPU_memory_map

    $0000-$0FFF	$1000	Pattern table 0
    $1000-$1FFF	$1000	Pattern table 1
    $2000-$23FF	$0400	Nametable 0
    $2400-$27FF	$0400	Nametable 1
    $2800-$2BFF	$0400	Nametable 2
    $2C00-$2FFF	$0400	Nametable 3
    $3000-$3EFF	$0F00	Mirrors of $2000-$2EFF
    $3F00-$3F1F	$0020	Palette RAM indexes
    $3F20-$3FFF	$00E0	Mirrors of $3F00-$3F1F

    oam
    $00, $04, $08, $0C	Sprite Y coordinate
    $01, $05, $09, $0D	Sprite tile #
    $02, $06, $0A, $0E	Sprite attribute
    $03, $07, $0B, $0F	Sprite X coordinate
*/

//static SCREEN_WIDTH: u32 = 256;
//static SCREEN_HEIGHT: u32 = 240;

// index 0 based
static TOTAL_SCANLINES: u32 = 262; // Vblank beyond screen height, 261 is pre render line
static CYCLES_PER_SCANLINE: u32 = 341;

pub struct PPU {
    pub memory: [u8; 0x4000],
    pub oam: [u8; 0x100],
    pub cycle: u32,
    pub scanline: u32,

    pub ppu_ctrl: u8,
    pub ppu_mask: u8,
    pub ppu_status: u8,
    pub oam_addr: u16,
    pub ppu_scroll_latch: bool,
    pub ppu_addr_latch: bool,
    pub ppu_scroll: u8,
    pub ppu_addr: u16,
    pub ppu_data: u8,
    pub odd_frame: bool,
}

impl Default for PPU {
    fn default() -> PPU {
        PPU {
            memory: [0; 0x4000],
            oam: [0; 0x100],
            cycle: 0,
            scanline: 0,
            
            ppu_ctrl: 0,
            ppu_mask: 0,
            ppu_status: 0,
            oam_addr: 0,
            ppu_scroll_latch: false,
            ppu_addr_latch: false,
            ppu_scroll: 0,
            ppu_addr: 0,
            ppu_data: 0,
            odd_frame: false,
        }
    }
}

impl fmt::Display for PPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

pub fn mapped_address(addr: usize) -> usize {
    if addr > 0x2FFF && addr < 0x3F00 {
        return 0x2000 + (addr & 0xEFF);
    }

    if addr > 0x3F1F {
        return 0x3F00 + (addr & 0x1F);
    }

    return addr;
}

pub fn run_cycle(emulator: &mut config::Emulator) {
    match emulator.ppu.scanline {
        0..=239 => process_visible_scanline(emulator),
        240 => process_post_scanline(emulator),
        241 => process_nmi(emulator),
        261 => process_pre_scanline(emulator),
        _ => println!("Invalid Scanline Detected")
    }

    // Update cycle and scanline counters:
    emulator.ppu.cycle += 1;
    if emulator.ppu.cycle >= CYCLES_PER_SCANLINE {
        emulator.ppu.cycle %= CYCLES_PER_SCANLINE;
        emulator.ppu.scanline += 1;
        if emulator.ppu.scanline >= TOTAL_SCANLINES {
            emulator.ppu.scanline = 0;
            emulator.ppu.odd_frame = !emulator.ppu.odd_frame;
        }
    }
}

fn process_visible_scanline(emulator: &mut config::Emulator) {

}

fn process_post_scanline(emulator: &mut config::Emulator) {
    
}

fn process_nmi(emulator: &mut config::Emulator) {
    
}

fn process_pre_scanline(emulator: &mut config::Emulator) {
    
}

pub fn reset(emulator: &mut config::Emulator) {
    emulator.ppu.cycle = 0;
    emulator.ppu.scanline = 0;
    emulator.ppu.ppu_ctrl = 0;
    emulator.ppu.ppu_mask = 0;
    emulator.ppu.ppu_status = 0;
    emulator.ppu.oam_addr = 0;
    emulator.ppu.ppu_scroll_latch = false;
    emulator.ppu.ppu_addr_latch = false;
    emulator.ppu.ppu_scroll = 0;
    emulator.ppu.ppu_addr = 0;
    emulator.ppu.ppu_data = 0;
    emulator.ppu.odd_frame = false;
}

pub fn get_control_increment_mode(emulator: &mut config::Emulator) -> bool {
    return emulator.ppu.ppu_status & 0x04 == 0x04;
}

pub fn set_vblank(emulator: &mut config::Emulator, value: bool) {
    if value {
        emulator.ppu.ppu_status |= 0b1000_0000;
    } else {
        emulator.ppu.ppu_status &= 0b0111_1111;
    }
}