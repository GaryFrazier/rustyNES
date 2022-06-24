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

static SCREEN_WIDTH: i32 = 256;
static SCREEN_HEIGHT: i32 = 240;
static TOTAL_SCANLINES: i32 = 261; // Vblank beyond screen height, after vblank set scanline to -1
static CYCLES_PER_SCANLINE: i32 = 341;

pub struct PPU {
    pub memory: [u8; 0x4000],
    pub oam: [u8; 0x100],
    pub cycle: u32,
    pub scanline: i32,

    pub ppu_ctrl: u8,
    pub ppu_mask: u8,
    pub ppu_status: u8,
    pub oam_addr: u16,
    pub ppu_scroll_latch: bool,
    pub ppu_addr_latch: bool,
    pub ppu_scroll: u8,
    pub ppu_addr: u8,
    pub ppu_data: u16,
    pub odd_frame: bool,
}

impl Default for PPU {
    fn default() -> PPU {
        PPU {
            memory: [0; 0x4000],
            oam: [0; 0x100],
            cycle: 0,
            scanline: -1,
            
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
    

    if emulator.ppu.cycle > 0 {
        emulator.ppu.cycle -= 1;
    }
}

pub fn reset(emulator: &mut config::Emulator) {
    emulator.ppu.cycle = 0;
    emulator.ppu.scanline = -1;
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