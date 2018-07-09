use std::rc::Rc;
use std::cell::RefCell;
use mappers::Mapper;
use bincode::*;
const COLORS: [u32;64] = [
        0x666666, 0x002A88, 0x1412A7, 0x3B00A4, 0x5C007E, 0x6E0040, 0x6C0600, 0x561D00,
		0x333500, 0x0B4800, 0x005200, 0x004F08, 0x00404D, 0x000000, 0x000000, 0x000000,
		0xADADAD, 0x155FD9, 0x4240FF, 0x7527FE, 0xA01ACC, 0xB71E7B, 0xB53120, 0x994E00,
		0x6B6D00, 0x388700, 0x0C9300, 0x008F32, 0x007C8D, 0x000000, 0x000000, 0x000000,
		0xFFFEFF, 0x64B0FF, 0x9290FF, 0xC676FF, 0xF36AFF, 0xFE6ECC, 0xFE8170, 0xEA9E22,
		0xBCBE00, 0x88D800, 0x5CE430, 0x45E082, 0x48CDDE, 0x4F4F4F, 0x000000, 0x000000,
		0xFFFEFF, 0xC0DFFF, 0xD3D2FF, 0xE8C8FF, 0xFBC2FF, 0xFEC4EA, 0xFECCC5, 0xF7D8A5,
        0xE4E594, 0xCFEF96, 0xBDF4AB, 0xB3F3CC, 0xB5EBF2, 0xB8B8B8, 0x000000, 0x000000];

fn get_bit(what:u16,whr:u8) -> u8 {
    return ((what & (1 << whr)) >> whr) as u8
}
#[derive(PartialEq)]
pub enum PPUStatus {
    Nothing,
    VBlank,
    HBlank
}

pub struct PPU {
    mapper: Rc<RefCell<Mapper>>,
    //PPUCTRL flags
    vram_increment: u16,
    sprite_pattern_base: u16,
    background_pattern_base: u16,
    big_sprites: bool,
    pub generate_nmi: bool,
    //PPUMASK flags
    greyscale: bool,
    show_left_background: bool,
    show_left_sprites: bool,
    show_background: bool,
    show_sprites: bool,
    emphasize_red: bool,
    emphasize_green: bool,
    emphasize_blue: bool,
    //PPUSTATUS flags
    sprite_overflow: bool,
    sprite_zero_hit: bool,
    vblank: bool,
    //OAM
    oamaddr: u8,
    oam: [u8;0x100],
    //loopy's scroll registers
    v:u16,
    t:u16,
    x:u8,
    w:bool,
    //background buffers and registers
    nametable_byte:u8,
    bitmap_low_input:u8,
    bitmap_high_input:u8,
    bitmap_low_shift:u16,
    bitmap_high_shift:u16,
    attribute_input:u8,
    attribute_low_shift:u8,
    attribute_high_shift:u8,
    attribute_low_input: u8,
    attribute_high_input: u8,
    palette:[u8;0x20],
    scanline:u16,
    scancycle:u16,
    oddframe:bool,
    //sprite buffers
    sprite_high_bitmaps:[u8;8],
    sprite_low_bitmaps:[u8;8],
    sprite_x_counters:[u8;8],
    sprite_attributes:[u8;8],
    sprite_indices:[usize;8],
    found: usize,

    //screen buffer
    pub screen : [u8;256*240*3],

    read_buffer: u8
}

fn bitwise_reverse(x:u8) -> u8 {
    let mut num = x;
    //thanks StackOverflow
    num = ((num & 0xf0) >> 4) | ((num & 0x0f) << 4);
    num = ((num & 0xcc) >> 2) | ((num & 0x33) << 2);
    num = ((num & 0xaa) >> 1) | ((num & 0x55) << 1);
    return num;
}
#[derive(Serialize,Deserialize)]
pub struct PPU_Serial { //PPU w/o one reference.... kms
    vram_increment: u16,
    sprite_pattern_base: u16,
    background_pattern_base: u16,
    big_sprites: bool,
    generate_nmi: bool,
    greyscale: bool,
    show_left_background: bool,
    show_left_sprites: bool,
    show_background: bool,
    show_sprites: bool,
    emphasize_red: bool,
    emphasize_green: bool,
    emphasize_blue: bool,
    sprite_overflow: bool,
    sprite_zero_hit: bool,
    vblank: bool,
    oamaddr: u8,
    oam: Vec<u8>,
    v:u16,
    t:u16,
    x:u8,
    w:bool,
    nametable_byte:u8,
    bitmap_low_input:u8,
    bitmap_high_input:u8,
    bitmap_low_shift:u16,
    bitmap_high_shift:u16,
    attribute_input:u8,
    attribute_low_shift:u8,
    attribute_high_shift:u8,
    attribute_low_input: u8,
    attribute_high_input: u8,
    palette:Vec<u8>,
    scanline:u16,
    scancycle:u16,
    oddframe:bool,
    sprite_high_bitmaps:[u8;8],
    sprite_low_bitmaps:[u8;8],
    sprite_x_counters:[u8;8],
    sprite_attributes:[u8;8],
    sprite_indices:[usize;8],
    found: usize,
    screen : Vec<u8>,
    read_buffer: u8
}

impl PPU {
    pub fn serialize(&self) -> Vec<u8> {
        let mut voam = vec![];
        voam.extend_from_slice(&self.oam);
        let mut vpalette = vec![];
        vpalette.extend_from_slice(&self.palette);
        let mut vscreen = vec![];
        vscreen.extend_from_slice(&self.screen);
        let serial = PPU_Serial {
            vram_increment: self.vram_increment,
            sprite_pattern_base: self.sprite_pattern_base,
            background_pattern_base: self.background_pattern_base,
            big_sprites: self.big_sprites,
            generate_nmi: self.generate_nmi,
            greyscale: self.greyscale,
            show_left_background: self.show_left_background,
            show_left_sprites: self.show_left_sprites,
            show_background: self.show_background,
            show_sprites: self.show_sprites,
            emphasize_red: self.emphasize_red,
            emphasize_green: self.emphasize_green,
            emphasize_blue: self.emphasize_blue,
            sprite_overflow: self.sprite_overflow,
            sprite_zero_hit: self.sprite_zero_hit,
            vblank: self.vblank,
            oamaddr: self.oamaddr,
            oam: voam,
            v: self.v,
            t: self.t,
            x: self.x,
            w: self.w,
            nametable_byte: self.nametable_byte,
            bitmap_low_input: self.bitmap_low_input,
            bitmap_high_input: self.bitmap_high_input,
            bitmap_low_shift: self.bitmap_low_shift,
            bitmap_high_shift: self.bitmap_high_shift,
            attribute_input: self.attribute_input,
            attribute_low_shift: self.attribute_low_shift,
            attribute_high_shift: self.attribute_high_shift,
            attribute_low_input: self.attribute_low_input,
            attribute_high_input: self.attribute_high_input,
            palette: vpalette,
            scanline: self.scanline,
            scancycle: self.scancycle,
            oddframe: self.oddframe,
            sprite_high_bitmaps: self.sprite_high_bitmaps,
            sprite_low_bitmaps: self.sprite_low_bitmaps,
            sprite_x_counters: self.sprite_x_counters,
            sprite_attributes: self.sprite_attributes,
            sprite_indices: self.sprite_indices,
            found: self.found,
            screen: vscreen,
            read_buffer: self.read_buffer
        };
        serialize(&serial).unwrap()
    }
    pub fn deserialize(&mut self,data: &[u8]) {
        let serial:PPU_Serial = deserialize(data).unwrap();
        self.vram_increment = serial.vram_increment;
        self.sprite_pattern_base = serial.sprite_pattern_base;
        self.background_pattern_base = serial.background_pattern_base;
        self.big_sprites = serial.big_sprites;
        self.generate_nmi = serial.generate_nmi;
        self.greyscale = serial.greyscale;
        self.show_left_background = serial.show_left_background;
        self.show_left_sprites = serial.show_left_sprites;
        self.show_background = serial.show_background;
        self.show_sprites = serial.show_sprites;
        self.emphasize_red = serial.emphasize_red;
        self.emphasize_green = serial.emphasize_green;
        self.emphasize_blue = serial.emphasize_blue;
        self.sprite_overflow = serial.sprite_overflow;
        self.sprite_zero_hit = serial.sprite_zero_hit;
        self.vblank = serial.vblank;
        self.oamaddr = serial.oamaddr;
        self.oam.copy_from_slice(&serial.oam);
        self.v = serial.v;
        self.t = serial.t;
        self.x = serial.x;
        self.w = serial.w;
        self.nametable_byte = serial.nametable_byte;
        self.bitmap_low_input = serial.bitmap_low_input;
        self.bitmap_high_input = serial.bitmap_high_input;
        self.bitmap_low_shift = serial.bitmap_low_shift;
        self.bitmap_high_shift = serial.bitmap_high_shift;
        self.attribute_input = serial.attribute_input;
        self.attribute_low_shift = serial.attribute_low_shift;
        self.attribute_high_shift = serial.attribute_high_shift;
        self.attribute_low_input = serial.attribute_low_input;
        self.attribute_high_input = serial.attribute_high_input;
        self.palette.copy_from_slice(&serial.palette);
        self.scanline = serial.scanline;
        self.scancycle = serial.scancycle;
        self.oddframe = serial.oddframe;
        self.sprite_high_bitmaps = serial.sprite_high_bitmaps;
        self.sprite_low_bitmaps = serial.sprite_low_bitmaps;
        self.sprite_x_counters = serial.sprite_x_counters;
        self.sprite_attributes = serial.sprite_attributes;
        self.sprite_indices = serial.sprite_indices;
        self.found = serial.found;
        self.screen.copy_from_slice(&serial.screen);
        self.read_buffer = serial.read_buffer;
    }
    pub fn new(mapper:Rc<RefCell<Mapper>>) -> PPU {
        PPU {
            mapper: mapper,
            vram_increment: 1,
            sprite_pattern_base: 0,
            background_pattern_base: 0,
            big_sprites: false,
            generate_nmi: false,
            greyscale: false,
            show_left_background: true,
            show_left_sprites: true,
            show_background: false,
            show_sprites: false,
            emphasize_red: false,
            emphasize_green: false,
            emphasize_blue: false,
            sprite_overflow: false,
            sprite_zero_hit: false,
            vblank: false,
            oamaddr: 0,
            oam: [0x69;0x100],
            v: 0,
            t: 0,
            x: 0,
            w: false,
            nametable_byte: 0,
            bitmap_low_input: 0,
            bitmap_high_input: 0,
            bitmap_low_shift: 0,
            bitmap_high_shift: 0,
            attribute_input: 0,
            attribute_low_shift: 0,
            attribute_high_shift: 0,
            attribute_low_input: 0,
            attribute_high_input: 0,
            palette: [0;0x20],
            scanline: 0,
            scancycle: 0,
            oddframe: false,
            sprite_high_bitmaps: [0xFF;8],
            sprite_low_bitmaps:  [0xFF;8],
            sprite_x_counters:   [0xFF;8],
            sprite_attributes:   [0xFF;8],
            sprite_indices: [0;8],
            found: 0,
            screen:[0;256*240*3],
            read_buffer: 0
        }
    }
    pub fn set_control(&mut self,what: u8) {
        let what = what as u16;
        self.t &= 0xF3FF;
        self.t |= (what & 0b11) << 10;
        self.vram_increment = if what & 0b100 != 0 {32} else {1};
        self.sprite_pattern_base = if what & 0b1000 != 0 {0x1000} else {0};
        self.background_pattern_base = if what & 0b10000 != 0 {0x1000} else {0};
        self.big_sprites = what & 0b100000 != 0;
        self.generate_nmi = what & 0b10000000 != 0;
    }
    pub fn set_mask(&mut self,what: u8) {
        self.greyscale = (what & 0b1) != 0;
        self.show_left_background = (what & 0b10) != 0;
        self.show_left_sprites = (what & 0b100) != 0;
        self.show_background = (what & 0b1000) != 0;
        self.show_sprites = (what & 0b10000) != 0;
        self.emphasize_red = (what & 0b100000) != 0;
        self.emphasize_green = (what & 0b1000000) != 0;
        self.emphasize_blue = (what & 0b10000000) != 0;
    }
    pub fn read_status(&mut self) -> u8 {
        self.w = false;
        let oldv = self.vblank;
        self.vblank = false;
        return ((oldv as u8) << 7) + ((self.sprite_zero_hit as u8) << 6) + ((self.sprite_overflow as u8) << 5);
    }
    pub fn set_oam_address(&mut self,what: u8) {
        self.oamaddr = what;
    }
    pub fn read_oam_data(&mut self) -> u8 {
        let result = self.oam[self.oamaddr as usize];
        self.oamaddr = self.oamaddr.wrapping_add(1);
        result
    }
    pub fn set_oam_data(&mut self,what:u8) {
        self.oam[self.oamaddr as usize] = what;
        self.oamaddr = self.oamaddr.wrapping_add(1);
    }
    pub fn set_scroll(&mut self,what: u8) {
        let what = what as u16;
        if !self.w {
            self.t &= 0xFFE0;
            self.t |= what >> 3;
            self.x = (what as u8) & 0b111;
        } else {
            self.t &= 0x8C1F;
            self.t |= (what & 0b111) << 12;
            self.t |= (what >> 3) << 5;
        }
        self.w = !self.w;
    }
    pub fn set_address(&mut self,what:u8) {
        let what = what as u16;
        if !self.w {
            self.t &= 0x80FF;
            self.t |= (what & 0b111111) << 8;
        } else {
            self.t &= 0xFF00;
            self.t |= what;
            self.v = self.t;
        }
        self.w = !self.w;
    }
    pub fn read_data(&mut self) -> u8 {
        let temp = self.v;
        self.v = self.v.wrapping_add(self.vram_increment);
        if self.v < 0x3F00 {
            let old = self.read_buffer;
            self.read_buffer = self.contents(temp);
            old
        } else {
            self.contents(temp)
        }
    }
    pub fn set_data(&mut self,what: u8) {
        let temp = self.v;
        self.set_contents(temp,what);
        self.v += self.vram_increment;
    }
    pub fn contents(&mut self,index_: u16) -> u8 {
        let mut index = index_ & 0x3FFF;
        match index {
            0x3F00 ... 0x3FFF => {
                index &= 0x1f;
                match index {
                    0x10 => {index = 0x00;}
                    0x14 => {index = 0x04;}
                    0x18 => {index = 0x08;}
                    0x1C => {index = 0x0C;}
                    _ => ()
                }
                self.palette[index as usize]
            }
            _ => {
                self.mapper.borrow_mut().vram_contents(index)
            }
        }
    }
    pub fn set_contents(&mut self,index_: u16,what:u8) {
        let mut index = index_ & 0x3FFF;
        match index {
            0x3F00 ... 0x3FFF => {
                index &= 0x1f;
                match index {
                    0x10 => {index = 0x00;}
                    0x14 => {index = 0x04;}
                    0x18 => {index = 0x08;}
                    0x1C => {index = 0x0C;}
                    _ => ()
                }
                self.palette[index as usize] = what;
            }
            _ => self.mapper.borrow_mut().set_vram_contents(index,what),
        }
    }
    pub fn evaluate_sprites(&mut self) {
        //println!("evaluatin' sprites @ {:X}",self.scanline);
        let mut oam2 = [0xFF; 0x20];
        let mut n = 0;
        self.found = 0;
        while n < 64 && self.found < 8 {
            oam2[self.found*4] = self.oam[n*4];
            if self.oam[n*4] as u16 <= self.scanline && self.scanline < self.oam[n*4] as u16 + (if self.big_sprites {16} else {8}) {
                oam2[self.found*4 + 1] = self.oam[n*4 + 1];
                oam2[self.found*4 + 2] = self.oam[n*4 + 2];
                oam2[self.found*4 + 3] = self.oam[n*4 + 3]+1;
                self.sprite_indices[self.found] = n;
                self.found += 1;
            }
            n += 1;
        }
        for i in 0..self.found {
            self.sprite_x_counters[i] = oam2[i*4 + 3];
            self.sprite_attributes[i] = oam2[i*4 + 2];
            let mut delta = self.scanline - oam2[i*4] as u16;
            if self.sprite_attributes[i] & 0x80 == 0x80 { //flag to flip sprite vertically
                delta = ((delta & 8) ^ if self.big_sprites {8} else {0}) | (7 - (delta & 7));
            }
            let addr =
                if !self.big_sprites {
                    self.sprite_pattern_base | ((oam2[i*4 + 1] as u16) << 4) | delta
                }
                else {
                    delta = delta & 0b111 | ((delta & 0b1000) << 1);
                    let tileno = (oam2[i*4 + 1] as u16 & 0xFE) << 4;
                    ((oam2[i*4 + 1] as u16 & 1) << 12) | tileno | delta
                };
            self.sprite_high_bitmaps[i] = self.contents(addr | 8);
            self.sprite_low_bitmaps[i] = self.contents(addr);
            if self.sprite_attributes[i] & 0x40 == 0x40 { //flag to flip horizontally
                self.sprite_high_bitmaps[i] = bitwise_reverse(self.sprite_high_bitmaps[i]);
                self.sprite_low_bitmaps[i] = bitwise_reverse(self.sprite_low_bitmaps[i]);
            }
        }
    }

    pub fn cycle(&mut self) -> PPUStatus {
        /*
        Sources:
        https://wiki.nesdev.com/w/index.php/PPU_scrolling
        https://wiki.nesdev.com/w/index.php/PPU_rendering
        The code here is pretty dense, so watch out...
        */
        let mut result = PPUStatus::Nothing;
        let draw = self.show_sprites || self.show_background;
        let isfetchcycle = draw && (self.scanline == 261 || self.scanline < 240) && (0 < self.scancycle && self.scancycle <= 256 && self.scanline != 261 || 321 <= self.scancycle && self.scancycle <= 336);
        if isfetchcycle {
            if self.scanline < 240 && self.scancycle <= 256 {
                let mut attribute_bits = get_bit(self.attribute_low_shift as u16,7-self.x) + 2*get_bit(self.attribute_high_shift as u16,7-self.x);
                let mut bitmap_bits = get_bit(self.bitmap_low_shift,15-self.x) + 2*get_bit(self.bitmap_high_shift,15-self.x);
                let mut background = true;
                if self.scanline != 0  {
                    let mut sprite_px = true;
                    for i in 0..self.found {
                        if self.sprite_x_counters[i] != 0 {
                            self.sprite_x_counters[i] -= 1;
                        }
                        if self.sprite_x_counters[i] == 0 {
                            let sprite_bitmap_bits = (self.sprite_low_bitmaps[i] >> 7) + 2*(self.sprite_high_bitmaps[i] >> 7);
                            self.sprite_low_bitmaps[i] <<= 1;
                            self.sprite_high_bitmaps[i] <<= 1;
                            let priority = self.sprite_attributes[i] & 0x20 == 0;
                            let sprite_attribute_bits = self.sprite_attributes[i] & 0b11;
                            if sprite_bitmap_bits != 0 && bitmap_bits != 0 && self.sprite_indices[i] == 0 {
                                self.sprite_zero_hit = true;
                            }
                            if sprite_px && sprite_bitmap_bits != 0 && !(self.scancycle < 9 && !self.show_left_sprites) {
                                sprite_px = false;
                                if bitmap_bits == 0 || priority {
                                    bitmap_bits = sprite_bitmap_bits;
                                    attribute_bits = sprite_attribute_bits;
                                    background = false;
                                }
                            }
                        }
                    }
                }
                let mut color =
                    self.fetch_color(background,attribute_bits,bitmap_bits);
                let x = self.scancycle as usize - 1;
                let y = self.scanline as usize;
                if background && x < 8 && !self.show_left_background {
                    color = self.fetch_color(true,0,0);
                }
                self.pixel(x,y,color);
            }

            self.bitmap_low_shift <<= 1;
            self.bitmap_high_shift <<= 1;

            self.attribute_low_shift <<= 1;
            self.attribute_low_shift |= self.attribute_low_input;

            self.attribute_high_shift <<= 1;
            self.attribute_high_shift |= self.attribute_high_input;

            match self.scancycle % 8 {
                1 => {
                    //index at nametable base, but ignore fine y stored in high bits
                    let nametable_index = 0x2000 | (self.v & 0xFFF);
                    self.nametable_byte = self.contents(nametable_index);
                }
                3 => {
                    //attribute table: indexed by nametable chosen, hi bits of coarse y & coarse x
                    let attrindex = 0x23C0 | self.v & 0xC00 | (self.v >> 4) & 0x38 | (self.v >> 2) & 0x7;
                    //bits further selected by low bits of coarse y & x
                    let shift = (self.v & 0b10) | (self.v >> 4) & 0b100;
                    self.attribute_input = (self.contents(attrindex) >> shift) & 0b11;
                }
                5 => {
                    //nametable at byte we fetched indexed by fine y
                    let low_addr = (self.v >> 12) | ((self.nametable_byte as u16) << 4) | self.background_pattern_base;
                    self.bitmap_low_input = self.contents(low_addr);
                }
                7 => {
                    let high_addr = (self.v >> 12) | ((self.nametable_byte as u16) << 4) | self.background_pattern_base | 8;
                    self.bitmap_high_input = self.contents(high_addr);
                }
                0 => {
                    self.bitmap_low_shift |= self.bitmap_low_input as u16;
                    self.bitmap_high_shift |= self.bitmap_high_input as u16;
                    self.attribute_low_input = self.attribute_input & 0b1;
                    self.attribute_high_input = (self.attribute_input & 0b10) >> 1;
                }
                _ => ()
            }
        }
        if self.show_sprites && self.scancycle == 321 && self.scanline < 239 {
            self.evaluate_sprites()
        }
        if draw {
            if self.scanline < 240 && self.scancycle == 256 {
                //increment fine y
                if self.v & 0x7000 != 0x7000 {
                    self.v += 0x1000;
                } else {
                    //oops, overflow
                    self.v &= !0x7000;
                    //fetch coarse y and increment that, switching nametables if that overflows
                    let mut y = (self.v & 0x3E0) >> 5;
                    if y == 29 {
                        y = 0;
                        self.v ^= 0x800;
                    } else if y == 31 {
                        y = 0;
                    } else {
                        y += 1;
                    }
                    self.v = (self.v & !0x3E0) | (y << 5);
                }
            }
            if (self.scanline < 240 || self.scanline == 261) && self.scancycle == 257 {
                //copy x-bits
                let mask = 0b10000011111;
                self.v &= !mask;
                self.v |= self.t & mask;
            }
            if self.scanline == 261 && 280 <= self.scancycle && self.scancycle <= 304 {
                //copy y-bits
                let mask = 0b111101111100000;
                self.v &= !mask;
                self.v |= self.t & mask;
            }
            if isfetchcycle && self.scancycle % 8 == 0 {
                //increment fine x
                if self.v & 0x1F == 0x1F {
                    self.v &= !0x1F;
                    self.v ^= 0x400;
                } else {
                    self.v += 1;
                }
            }
        }
        if self.scancycle == 280 && (self.scanline == 261 || self.scanline < 240) && draw {
            result = PPUStatus::HBlank;
        }
        if self.scanline == 241 && self.scancycle == 1 {
            self.vblank = true;
            self.sprite_zero_hit = false;
            result = PPUStatus::VBlank;
        }
        self.scancycle += 1;
        if self.scancycle == 341 || (self.scanline == 261 && self.oddframe && self.scancycle == 340) {
            self.scancycle = 0;
            self.scanline += 1;
            if self.scanline == 262 {
                self.scanline = 0;
                self.oddframe = !self.oddframe;
            }
        }
        return result
    }
    fn fetch_color(&mut self,isbackground: bool,paletteno: u8,color:u8) -> u32 {
        let index = (((!isbackground) as u8) << 4) + (paletteno << 2) + color;
        if color == 0 {
            return COLORS[self.palette[0] as usize & 0x3F];
        } else {
            return COLORS[self.palette[index as usize] as usize & 0x3F];
        }
    }
    fn pixel(&mut self,x: usize,y: usize, color: u32) {
        self.screen[x*3 + y*256*3] = ((color & 0xFF0000) >> 16) as u8;
        self.screen[x*3 + y*256*3 + 1] = ((color & 0xFF00) >> 8) as u8;
        self.screen[x*3 + y*256*3 + 2] = (color & 0xFF) as u8;
    }
}
