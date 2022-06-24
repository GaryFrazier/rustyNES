use std::fmt;
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

pub struct PPU {
    pub memory: [u8; 0x4000],
    pub oam: [u8; 0x100],
    pub cycle: u32,
}

impl Default for PPU {
    fn default() -> PPU {
        PPU {
            memory: [0; 0x4000],
            oam: [0; 0x100],
            cycle: 0,
        }
    }
}

impl fmt::Display for PPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
