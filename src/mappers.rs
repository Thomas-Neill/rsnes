use ines::INES;
use std::rc::Rc;
use std::cell::RefCell;
use bincode::*;

pub trait Mapper {
    fn contents(&mut self,index:u16) -> u8;
    fn set_contents(&mut self,index:u16,what:u8);
    fn vram_contents(&mut self,index:u16) -> u8;
    fn set_vram_contents(&mut self,index:u16,what:u8);
    fn get_savedata(&mut self) -> [u8; 0x2000];
    fn scanline(&mut self) { //true = IRQ false = no IRQ

    }
    fn interrupt(&mut self) -> bool {
        false
    }
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self,data:&[u8]);
}

struct NROM {
    ines: INES,
    prgram: [u8;0x2000],
    nametables:[[u8;0x400];2]
}

#[derive(Serialize,Deserialize)]
struct NROM_Serial {
    prgram: Vec<u8>,
    nametables: Vec<Vec<u8>>
}

impl Mapper for NROM {
    fn serialize(&self) -> Vec<u8> {
        let mut vprgram = vec![];
        vprgram.extend_from_slice(&self.prgram);
        let mut vnametables = vec![vec![],vec![]];
        vnametables[0].extend_from_slice(&self.nametables[0]);
        vnametables[1].extend_from_slice(&self.nametables[1]);
        serialize(&NROM_Serial {prgram: vprgram, nametables: vnametables}).unwrap()
    }
    fn deserialize(&mut self,data:&[u8]) {
        let x: NROM_Serial = deserialize(data).unwrap();
        self.prgram.copy_from_slice(&x.prgram);
        self.nametables[0].copy_from_slice(&x.nametables[0]);
        self.nametables[1].copy_from_slice(&x.nametables[1]);
    }
    fn contents(&mut self,location:u16) -> u8 {
        let mut location = location as usize;
        match location {
            0x6000...0x7FFF => self.prgram[location - 0x6000],
            0x8000...0xFFFF => {
                if self.ines.prgrom_size == 1 { location &= 0xBFFF; }
                self.ines.prgrom[location - 0x8000]
            }
            _ => 0
        }
    }
    fn set_contents(&mut self,location:u16,what:u8) {
        match location {
            0x6000...0x7FFF => self.prgram[location as usize - 0x6000] = what,
            _ => ()
        }
    }
    fn vram_contents(&mut self,location:u16) -> u8 {
        let mut location = location as usize;
        match location {
            0...0x1FFF => self.ines.chrrom[location],
            0x2000...0x3EFF => {
                location &= 0xFFF;
                let nametable:usize = match location {
                    0...0x3FF => 0,
                    0x400...0x7FF => (self.ines.vertical_mirroring) as usize,
                    0x800...0xBFF => (!self.ines.vertical_mirroring) as usize,
                    0xC00...0xFFF => 1,
                    _ => 2
                };
                location &= 0x3FF;
                self.nametables[nametable][location]
            }
            _ => 0
        }
    }
    fn set_vram_contents(&mut self,location:u16,what:u8) {
        let mut location = location as usize;
        match location {
            0...0x1FFF => {
                self.ines.chrrom[location] = what;
            }
            0x2000...0x3EFF => {
                location &= 0xFFF;
                let nametable:usize = match location {
                    0...0x3FF => 0,
                    0x400...0x7FF => (self.ines.vertical_mirroring) as usize,
                    0x800...0xBFF => (!self.ines.vertical_mirroring) as usize,
                    0xC00...0xFFF => 1,
                    _ => 2
                };
                location &= 0x3FF;
                self.nametables[nametable][location] = what;
            }
            _ => ()
        }
    }
    fn get_savedata(&mut self) -> [u8; 0x2000] {
        self.prgram.clone()
    }
}

struct MMC1 {
    ines: INES,
    input: u8,

    mirroringtype: u8,
    prgrommode: u8,
    chrrommode: u8,
    chr0bank: usize,
    chr1bank: usize,
    enableprgram: bool,
    prgbank: usize,

    prgram: [u8;0x2000],
    nametables:[[u8;0x400];2]
}

#[derive(Serialize,Deserialize)]
struct MMC1_Serial {
    input: u8,
    mirroringtype: u8,
    prgrommode: u8,
    chrrommode: u8,
    chr0bank: usize,
    chr1bank: usize,
    enableprgram: bool,
    prgbank: usize,
    prgram: Vec<u8>,
    nametables: Vec<Vec<u8>>
}

impl Mapper for MMC1 {
    fn serialize(&self) -> Vec<u8> {
        let mut vprgram = vec![];
        vprgram.extend_from_slice(&self.prgram);
        let mut vnametables = vec![vec![],vec![]];
        vnametables[0].extend_from_slice(&self.nametables[0]);
        vnametables[1].extend_from_slice(&self.nametables[1]);
        let serial = MMC1_Serial {
            input:self.input,
            mirroringtype:self.mirroringtype,
            prgrommode:self.prgrommode,
            chrrommode:self.chrrommode,
            chr0bank:self.chr0bank,
            chr1bank:self.chr1bank,
            enableprgram:self.enableprgram,
            prgbank:self.prgbank,
            prgram:vprgram,
            nametables:vnametables
        };
        serialize(&serial).unwrap()
    }
    fn deserialize(&mut self,data:&[u8]) {
        let x: MMC1_Serial = deserialize(data).unwrap();
        self.input = x.input;
        self.mirroringtype = x.mirroringtype;
        self.prgrommode = x.prgrommode;
        self.chrrommode = x.chrrommode;
        self.chr0bank = x.chr0bank;
        self.chr1bank = x.chr1bank;
        self.enableprgram = x.enableprgram;
        self.prgbank = x.prgbank;
        self.prgram.copy_from_slice(&x.prgram);
        self.nametables[0].copy_from_slice(&x.nametables[0]);
        self.nametables[1].copy_from_slice(&x.nametables[1]);
    }
    fn contents(&mut self,location:u16) -> u8 {
        let mut location = location as usize;
        match location {
            0x6000...0x7FFF =>
                self.prgram[location - 0x6000],
            0x8000...0xFFFF => {
                location -= 0x8000;
                match self.prgrommode {
                    0...1 => { //choose 32 kb bank
                        self.ines.prgrom[0x8000*(self.prgbank >> 1) + location]
                    }
                    2 => {
                        if location < 0x4000 {
                            self.ines.prgrom[location]
                        } else {
                            self.ines.prgrom[0x4000*self.prgbank + (location - 0x4000)]
                        }
                    }
                    3 => {
                        if location < 0x4000 {
                            self.ines.prgrom[0x4000*(self.prgbank % self.ines.prgrom_size as usize) + location]
                        } else {
                            self.ines.prgrom[(0x4000*(self.ines.prgrom_size as usize - 1) + (location - 0x4000))]
                        }
                    }
                    _ => 0
                }
            }
            _ => 0
        }
    }
    fn set_contents(&mut self,location:u16,what:u8) {
        match location {
            0x6000...0x7FFF => {
                self.prgram[location as usize - 0x6000] = what;
            }
            0x8000...0xFFFF => {
                if what & 0x80 != 0 {
                    self.input = 0x10;
                    self.prgrommode = 3;
                } else {
                    let old = self.input & 1;
                    self.input >>= 1;
                    self.input |= (what & 1) << 4;
                    if old != 0 {
                        match location {
                            0x8000...0x9FFF => {
                                self.mirroringtype = self.input & 3;
                                self.prgrommode = (self.input >> 2) & 3;
                                self.chrrommode = (self.input >> 4) & 1;
                            }
                            0xA000...0xBFFF => {
                                self.chr0bank = self.input as usize;
                            }
                            0xC000...0xDFFF => {
                                self.chr1bank = self.input as usize;
                            }
                            0xE000...0xFFFF => {
                                self.prgbank = (self.input & 0xF) as usize;
                                self.enableprgram = self.input & 0x10 != 0;
                            }
                            _ => ()
                        }
                        self.input = 0x10;
                    }
                }
            }
            _ => ()
        }
    }
    fn vram_contents(&mut self,location:u16) -> u8 {
        let mut location = location as usize;
        match location {
            0...0x1FFF => {
                match self.chrrommode {
                    0 => {
                        self.ines.chrrom[0x2000*(self.chr0bank >> 1) + location]
                    }
                    1 => {
                        if location < 0x1000 {
                            self.ines.chrrom[0x1000*self.chr0bank + location]
                        } else {
                            self.ines.chrrom[0x1000*self.chr1bank + location - 0x1000]
                        }
                    }
                    _ => 0
                }
            }
            0x2000...0x3EFF => {
                location &= 0xFFF;
                let nametable:usize = match location {
                    0...0x3FF => (self.mirroringtype == 1) as usize,
                    0x400...0x7FF => (self.mirroringtype == 2 || self.mirroringtype == 1) as usize,
                    0x800...0xBFF => (self.mirroringtype == 1 || self.mirroringtype == 3) as usize,
                    0xC00...0xFFF => (self.mirroringtype != 0) as usize,
                    _ => 2
                };
                location &= 0x3FF;
                self.nametables[nametable][location]
            }
            _ => 0
        }
    }
    fn set_vram_contents(&mut self,location:u16,what:u8) {
        let mut location = location as usize;
        match location {
            0...0x1FFF if self.ines.chrrom_size == 0 => {
                match self.chrrommode {
                    0 => {
                        self.ines.chrrom[0x2000*(self.chr0bank >> 1) + location] = what
                    }
                    1 => {
                        if location < 0x1000 {
                            self.ines.chrrom[0x1000*self.chr0bank + location] = what
                        } else {
                            self.ines.chrrom[0x1000*self.chr1bank + location - 0x1000] = what
                        }
                    }
                    _ => ()
                }
            }
            0x2000...0x3EFF => {
                location &= 0xFFF;
                let nametable:usize = match location {
                    0...0x3FF => (self.mirroringtype == 1) as usize,
                    0x400...0x7FF => (self.mirroringtype == 2 || self.mirroringtype == 1) as usize,
                    0x800...0xBFF => (self.mirroringtype == 1 || self.mirroringtype == 3) as usize,
                    0xC00...0xFFF => (self.mirroringtype != 0) as usize,
                    _ => 2
                };
                location &= 0x3FF;
                self.nametables[nametable][location] = what;
            }
            _ => ()
        }
    }
    fn get_savedata(&mut self) -> [u8; 0x2000] {
        self.prgram.clone()
    }
}

pub struct MMC3 {
    ines: INES,
    prgram: [u8;0x2000],
    nametables:[[u8;0x400];2],
    inputselect: u8,
    prgrommode: bool,
    chrrommode: bool,
    chrbank0: usize,
    chrbank1: usize,
    chrbank2: usize,
    chrbank3: usize,
    chrbank4: usize,
    chrbank5: usize,
    prgbank0: usize,
    prgbank1: usize,
    horizontalmirroring: bool,
    irqreload: u8,
    irqcounter: u8,
    generate_irq: bool,
    interrupt: bool
}
#[derive(Serialize,Deserialize)]
struct MMC3_Serial {
    prgram: Vec<u8>,
    nametables: Vec<Vec<u8>>,
    inputselect: u8,
    prgrommode: bool,
    chrrommode: bool,
    chrbank0: usize,
    chrbank1: usize,
    chrbank2: usize,
    chrbank3: usize,
    chrbank4: usize,
    chrbank5: usize,
    prgbank0: usize,
    prgbank1: usize,
    horizontalmirroring: bool,
    irqreload: u8,
    irqcounter: u8,
    generate_irq: bool,
    interrupt: bool
}

impl Mapper for MMC3 {
    fn serialize(&self) -> Vec<u8> {
        let mut vprgram = vec![];
        vprgram.extend_from_slice(&self.prgram);
        let mut vnametables = vec![vec![],vec![]];
        vnametables[0].extend_from_slice(&self.nametables[0]);
        vnametables[1].extend_from_slice(&self.nametables[1]);
        let serial = MMC3_Serial {
            prgram: vprgram,
            nametables: vnametables,
            inputselect: self.inputselect,
            prgrommode: self.prgrommode,
            chrrommode: self.chrrommode,
            chrbank0: self.chrbank0,
            chrbank1: self.chrbank1,
            chrbank2: self.chrbank2,
            chrbank3: self.chrbank3,
            chrbank4: self.chrbank4,
            chrbank5: self.chrbank5,
            prgbank0: self.prgbank0,
            prgbank1: self.prgbank1,
            horizontalmirroring: self.horizontalmirroring,
            irqreload: self.irqreload,
            irqcounter: self.irqcounter,
            generate_irq: self.generate_irq,
            interrupt: self.interrupt
        };
        serialize(&serial).unwrap()
    }
    fn deserialize(&mut self,data:&[u8]) {
        let x: MMC3_Serial = deserialize(data).unwrap();
        self.prgram.copy_from_slice(&x.prgram);
        self.nametables[0].copy_from_slice(&x.nametables[0]);
        self.nametables[1].copy_from_slice(&x.nametables[1]);
        self.inputselect = x.inputselect;
        self.prgrommode = x.prgrommode;
        self.chrrommode = x.chrrommode;
        self.chrbank0 = x.chrbank0;
        self.chrbank1 = x.chrbank1;
        self.chrbank2 = x.chrbank2;
        self.chrbank3 = x.chrbank3;
        self.chrbank4 = x.chrbank4;
        self.chrbank5 = x.chrbank5;
        self.prgbank0 = x.prgbank0;
        self.prgbank1 = x.prgbank1;
        self.horizontalmirroring = x.horizontalmirroring;
        self.irqreload = x.irqreload;
        self.irqcounter = x.irqcounter;
        self.generate_irq = x.generate_irq;
        self.interrupt = x.interrupt;
    }
    fn contents(&mut self,location:u16) -> u8 {
        let location = location as usize;
        let prgrom_size = self.ines.prgrom_size as usize * 2;
        match location {
            0x6000...0x7FFF => self.prgram[location - 0x6000],
            0x8000...0x9FFF => {
                if self.prgrommode {
                    self.ines.prgrom[0x2000*(prgrom_size - 2) + location - 0x8000]
                } else {
                    self.ines.prgrom[0x2000*self.prgbank0 + location - 0x8000]
                }
            }
            0xA000...0xBFFF => self.ines.prgrom[0x2000*self.prgbank1 + location - 0xA000],
            0xC000...0xDFFF => {
                if !self.prgrommode {
                    self.ines.prgrom[0x2000*(prgrom_size - 2) + location - 0xC000]
                } else {
                    self.ines.prgrom[0x2000*self.prgbank0 + location - 0xC000]
                }
            }
            0xE000...0xFFFF => self.ines.prgrom[0x2000*(prgrom_size - 1) + location - 0xE000],
            _ => 0
        }
    }
    fn set_contents(&mut self,location:u16,what:u8) {
        match location {
            0x6000...0x7FFF => self.prgram[location as usize - 0x6000] = what,
            0x8000...0x9FFF => {
                if location & 1 == 0 {
                    self.inputselect = what & 0x7;
                    self.prgrommode = what & 0x40 != 0;
                    self.chrrommode = what & 0x80 != 0;
                } else {
                    let what = what as usize;
                    match self.inputselect {
                        0 => self.chrbank0 = what >> 1,
                        1 => self.chrbank1 = what >> 1,
                        2 => self.chrbank2 = what,
                        3 => self.chrbank3 = what,
                        4 => self.chrbank4 = what,
                        5 => self.chrbank5 = what,
                        6 => self.prgbank0 = what & 0x3F,
                        7 => self.prgbank1 = what & 0x3F,
                        _ => ()
                    }
                }
            }
            0xA000...0xBFFF => {
                if location & 1 == 0 {
                    self.horizontalmirroring = what & 1 != 0;
                } else {
                    () //not neccesary to emulate; ram write protection
                }
            }
            0xC000...0xDFFF => {
                if location & 1 == 0 {
                    self.irqreload = what;
                } else {
                    self.irqcounter = 0;
                }
            }
            0xE000...0xEFFF => {
                if location & 1 == 0 {
                    self.generate_irq = false;
                } else {
                    self.generate_irq = true;
                }
            }
            _ => ()
        }
    }
    fn vram_contents(&mut self,location:u16) -> u8 {
        let mut location = location as usize;
        if self.chrrommode {
            location ^= 0x1000;
        }
        match location {
            0x0...0x7FF => self.ines.chrrom[0x800*self.chrbank0 + location],
            0x800...0xFFF => self.ines.chrrom[0x800*self.chrbank1 + location - 0x800],
            0x1000...0x13FF => self.ines.chrrom[0x400*self.chrbank2 + location - 0x1000],
            0x1400...0x17FF => self.ines.chrrom[0x400*self.chrbank3 + location - 0x1400],
            0x1800...0x1BFF => self.ines.chrrom[0x400*self.chrbank4 + location - 0x1800],
            0x1C00...0x1FFF => self.ines.chrrom[0x400*self.chrbank5 + location - 0x1C00],
            0x2000...0x3EFF => {
                location &= 0xFFF;
                let nametable:usize = match location {
                    0...0x3FF => 0,
                    0x400...0x7FF => (!self.horizontalmirroring) as usize,
                    0x800...0xBFF => self.horizontalmirroring as usize,
                    0xC00...0xFFF => 1,
                    _ => 2
                };
                location &= 0x3FF;
                self.nametables[nametable][location]
            }
            _ => 0
        }
    }
    fn set_vram_contents(&mut self,location:u16,what:u8) {
        let mut location = location as usize;
        match location {
            0x0...0x7FF => self.ines.chrrom[0x800*self.chrbank0 + location] = what,
            0x800...0xFFF => self.ines.chrrom[0x800*self.chrbank1 + location - 0x800] = what,
            0x1000...0x13FF => self.ines.chrrom[0x400*self.chrbank2 + location - 0x1000] = what,
            0x1400...0x17FF => self.ines.chrrom[0x400*self.chrbank3 + location - 0x1400] = what,
            0x1800...0x1BFF => self.ines.chrrom[0x400*self.chrbank4 + location - 0x1800] = what,
            0x1C00...0x1FFF => self.ines.chrrom[0x400*self.chrbank5 + location - 0x1C00] = what,
            0x2000...0x3EFF => {
                location &= 0xFFF;
                let nametable:usize = match location {
                    0...0x3FF => 0,
                    0x400...0x7FF => (!self.horizontalmirroring) as usize,
                    0x800...0xBFF => self.horizontalmirroring as usize,
                    0xC00...0xFFF => 1,
                    _ => 2
                };
                location &= 0x3FF;
                self.nametables[nametable][location] = what;
            }
            _ => ()
        }
    }
    fn interrupt(&mut self) -> bool {
        let old = self.interrupt;
        self.interrupt = false;
        old
    }
    fn scanline(&mut self) {
        if self.irqcounter == 0 {
            self.irqcounter = self.irqreload;
        } else {
            self.irqcounter -= 1;
            if self.irqcounter == 0 && self.generate_irq {
                self.interrupt = true;
            }
        }
    }
    fn get_savedata(&mut self) -> [u8; 0x2000] {
        self.prgram.clone()
    }
}

pub fn get_mapper(mut ines:INES) -> Rc<RefCell<Mapper>> {
    let prgram = ines.savedata.clone();
    match ines.mapper {
        0 => Rc::new(RefCell::new(NROM {ines:ines,prgram:prgram,nametables:[[0;0x400];2]})),
        1 => {
            Rc::new(RefCell::new(
                MMC1 {
                    ines:ines,
                    prgram:prgram,
                    nametables:[[0;0x400];2],
                    mirroringtype:0,
                    prgrommode:3,
                    chrrommode:0,
                    chr0bank: 0,
                    chr1bank: 0,
                    enableprgram: true,
                    prgbank: 0,
                    input: 1 << 4,
                }))
        }
        4 => {
            Rc::new(RefCell::new({
                MMC3 {
                    ines: ines,
                    prgram: prgram,
                    nametables:[[0;0x400];2],
                    inputselect: 0,
                    prgrommode: false,
                    chrrommode: false,
                    chrbank0: 0,
                    chrbank1: 0,
                    chrbank2: 0,
                    chrbank3: 0,
                    chrbank4: 0,
                    chrbank5: 0,
                    prgbank0: 0,
                    prgbank1: 0,
                    horizontalmirroring: false,
                    irqreload: 0,
                    irqcounter: 0,
                    generate_irq: false,
                    interrupt: false,
                }
            }))
        }
        _ => panic!("Bad id: {}",ines.mapper)
    }
}
