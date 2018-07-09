use instruction::*;
use apu::*;
use ppu::*;
use mappers::Mapper;
use std::rc::Rc;
use std::cell::RefCell;
use bincode::*;
enum Flags {
    Carry = 0,
    Zero = 1,
    Interrupt = 2,
    Decimal = 3,
    Break = 4,
    BadBit = 5,
    Overflow = 6,
    Sign = 7,
}

#[derive(Serialize,Deserialize,Clone,Copy)]
pub enum Interrupt {
    Null,
    NMI,
    IRQ,
}

fn mask(x: Flags) -> u8 {
    1 << x as u8
}

fn maskout(x: Flags) -> u8 {
    !mask(x)
}

fn set_flags(p:&mut u8,value: u8) {
    *p &= maskout(Flags::Zero) & maskout(Flags::Sign);
    if value == 0 {
        *p |= mask(Flags::Zero);
    }
    if value & 0x80 != 0 {
        *p |= mask(Flags::Sign);
    }
}

fn signum(x: i8) -> i8 {
    if x < 0 {return -1;}
    if x > 0 {return 1;}
    return 0;
}

#[allow(non_snake_case)]
pub struct CPU {
    SP: u8,
    P: u8,
    A: u8,
    X: u8,
    Y: u8,
    mapper: Rc<RefCell<Mapper>>,
    ppu: Rc<RefCell<PPU>>,
    ram: [u8; 0x800],
    cycles_delay: u32,
    pub PC: u16, //for testing purposes
    cycles: u32,
    pub interrupt: Interrupt,
    pub inputs: [bool;8],
    pub inputs2: [bool;8],
    inputindex: u8,
    inputindex2: u8,
    strobe: u8,
    pub elapsed: i32
}


#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
struct CPU_Serial { //no refcells
    SP: u8,
    P: u8,
    A: u8,
    X: u8,
    Y: u8,
    ram: Vec<u8>,
    cycles_delay: u32,
    PC: u16, //for testing purposes
    cycles: u32,
    interrupt: Interrupt,
    inputs: [bool;8],
    inputs2: [bool;8],
    inputindex: u8,
    inputindex2: u8,
    strobe: u8,
    elapsed: i32
}

impl CPU {
    pub fn serialize(&self) -> Vec<u8> {
        let mut vram = vec![];
        vram.extend_from_slice(&self.ram);
        let serial = CPU_Serial { //no refcells
            SP: self.SP,
            P: self.P,
            A: self.A,
            X: self.X,
            Y: self.Y,
            ram: vram,
            cycles_delay: self.cycles_delay,
            PC: self.PC,
            cycles: self.cycles,
            interrupt: self.interrupt,
            inputs: self.inputs,
            inputs2: self.inputs2,
            inputindex: self.inputindex,
            inputindex2: self.inputindex2,
            strobe: self.strobe,
            elapsed: self.elapsed
        };
        serialize(&serial).unwrap()
    }
    pub fn deserialize(&mut self,data:&[u8]) {
        let serial: CPU_Serial = deserialize(data).unwrap();
        self.SP = serial.SP;
        self.P = serial.P;
        self.A = serial.A;
        self.X = serial.X;
        self.Y = serial.Y;
        self.ram.copy_from_slice(&serial.ram);
        self.cycles_delay = serial.cycles_delay;
        self.PC = serial.PC;
        self.cycles = serial.cycles;
        self.interrupt = serial.interrupt;
        self.inputs = serial.inputs;
        self.inputs2 = serial.inputs2;
        self.inputindex = serial.inputindex;
        self.inputindex2 = serial.inputindex2;
        self.strobe = serial.strobe;
        self.elapsed = serial.elapsed;
    }
    pub fn new(mapper: Rc<RefCell<Mapper>>, ppu: Rc<RefCell<PPU>>) -> CPU {
        let mut cpu = CPU {
            SP: 0xFF,
            P: 0x34,
            A: 0,
            X: 0,
            Y: 0,
            mapper: mapper,
            ppu: ppu,
            ram: [0; 0x800],
            cycles_delay: 0,
            PC: 0,
            cycles: 0,
            interrupt: Interrupt::Null,
            inputs: [false; 8],
            inputs2: [false; 8],
            inputindex: 8,
            inputindex2: 8,
            strobe: 0,
            elapsed: 0
        };
        cpu.PC = cpu.contents(0xFFFC) as u16 + ((cpu.contents(0xFFFD) as u16)<< 8);
        cpu
    }
    pub fn frame(&mut self) {
        self.elapsed = 0;
    }
    pub fn cycle(&mut self) {
        if self.cycles_delay == 0 {
            match self.interrupt {
                Interrupt::Null => self.instr(),
                Interrupt::NMI => self.jump_interrupt(0xFFFA, false),
                Interrupt::IRQ => if self.P & mask(Flags::Interrupt) == 0 {
                    self.jump_interrupt(0xFFFE, false);
                } else {
                    self.interrupt = Interrupt::Null;
                    self.instr();
                }
            }
        }
        self.cycles_delay -= 1;
        self.cycles += 1;
        self.elapsed += 1;
    }

    pub fn contents(&mut self, location: u16) -> u8 {
        match location {
            0...0x1FFF => self.ram[location as usize& 0x7FF],
            0x2000 => 0,
            0x2001 => 0,
            0x2002 => self.ppu.borrow_mut().read_status(),
            0x2003 => 0,
            0x2004 => self.ppu.borrow_mut().read_oam_data(),
            0x2005 => 0,
            0x2006 => 0,
            0x2007 => self.ppu.borrow_mut().read_data(),
            0x2008...0x3FFF => self.contents(location & 0x2007),
            0x4000...0x4014 => 0,
            0x4015 => apu_read(self.elapsed+self.cycles_delay as i32),
            0x4016 => {
                if self.inputindex == 8 {
                    1
                } else {
                    let result = if self.inputs[self.inputindex as usize] { 1 } else { 0 };
                    self.inputindex += 1;
                    if {self.strobe&1 != 0} {self.inputindex = 0;}
                    result
                }
            }
            0x4017 => {
                if self.inputindex2 == 8 {
                    1
                } else {
                    let result = if self.inputs2[self.inputindex2 as usize] { 1 } else { 0 };
                    self.inputindex2 += 1;
                    if {self.strobe&1 != 0} {self.inputindex2 = 0;}
                    result
                }
            }
            0x4016...0x401F => 0,
            0x4020...0xFFFF => self.mapper.borrow_mut().contents(location),
            _ => panic!("DNE")
        }
    }
    fn set_contents(&mut self, location: u16, value: u8) {
        match location {
            0...0x1FFF => self.ram[location as usize & 0x7FF] = value,
            0x2000 => self.ppu.borrow_mut().set_control(value),
            0x2001 => self.ppu.borrow_mut().set_mask(value),
            0x2002 => (),
            0x2003 => self.ppu.borrow_mut().set_oam_address(value),
            0x2004 => self.ppu.borrow_mut().set_oam_data(value),
            0x2005 => self.ppu.borrow_mut().set_scroll(value),
            0x2006 => self.ppu.borrow_mut().set_address(value),
            0x2007 => self.ppu.borrow_mut().set_data(value),
            0x2008...0x3FFF => self.set_contents(location & 0x2007,value),
            0x4000...0x4013 => apu_write(self.elapsed+self.cycles_delay as i32,location,value),
            0x4015 => apu_write(self.elapsed+self.cycles_delay as i32,location,value),
            0x4017 => apu_write(self.elapsed+self.cycles_delay as i32,location,value),
            0x4014 => {
                for i in 0..0x100 {
                    let val = self.contents(((value as u16) << 8) + i);
                    self.ppu.borrow_mut().set_oam_data(val);
                }
                self.cycles_delay += 513 + (self.cycles % 2);
            }
            0x4016 => {
                    self.strobe = value;
                    if {self.strobe & 1 != 0} {self.inputindex = 0; self.inputindex2 = 0;}
            }
            0x4018...0x401F => (),
            0x4020...0xFFFF => self.mapper.borrow_mut().set_contents(location,value),
            _ => panic!("DNE")
        }
    }
    fn stack_push(&mut self, what: u8) {
        let index = 0x100 + self.SP as u16;
        self.set_contents(index, what);
        self.SP = self.SP.wrapping_sub(1);
    }
    fn stack_pop(&mut self) -> u8{
        self.SP = self.SP.wrapping_add(1);
        let index = 0x100 + self.SP as u16;
        self.contents(index)
    }
    fn fetch_argument(&mut self, addrmode: AddressingMode) -> u8 {
        use instruction::AddressingMode::*;
        let pc = self.PC;
        match addrmode {
            Immediate => self.contents(pc - 1),
            Relative => self.contents(pc - 1),
            Accumulator => self.A,
            ZeroPage => {
                let index = self.contents(pc - 1) as u16;
                self.contents(index)
            }
            ZeroPageX => {
                let index = self.contents(pc - 1) as u16 + self.X as u16 & 0xFF;
                self.contents(index)
            }
            ZeroPageY => {
                let index = self.contents(pc - 1) as u16 + self.Y as u16 & 0xFF;
                self.contents(index)
            }
            Absolute => {
                let index = self.contents(pc - 2) as u16 + ((self.contents(pc - 1) as u16) << 8);
                self.contents(index)
            }
            AbsoluteX => {
                let index = (self.contents(pc - 2) as u16).wrapping_add((self.contents(pc - 1) as u16) << 8).wrapping_add(self.X as u16);
                self.contents(index)
            }
            AbsoluteY => {
                let index = (self.contents(pc - 2) as u16).wrapping_add((self.contents(pc - 1) as u16) << 8).wrapping_add(self.Y as u16);
                self.contents(index)
            }
            IndirectX => {
                let indexindex = self.contents(pc - 1) as u16 + self.X as u16 & 0xFF;
                let index = self.contents(indexindex) as u16 + ((self.contents(indexindex + 1 & 0xFF) as u16) << 8);
                self.contents(index)
            }
            IndirectY => {
                let indexindex = self.contents(pc - 1) as u16;
                let index = (self.contents(indexindex) as u16).wrapping_add((self.contents(indexindex + 1 & 0xFF) as u16) << 8).wrapping_add(self.Y as u16);
                self.contents(index)
            }
            _ => 0,
        }
    }
    fn replace_argument(&mut self, addrmode: AddressingMode, value: u8) {
        use instruction::AddressingMode::*;
        let pc = self.PC;
        match addrmode {
            Accumulator => self.A = value,
            ZeroPage => {
                let index = self.contents(pc - 1) as u16;
                self.set_contents(index, value);
            }
            ZeroPageX => {
                let index = self.contents(pc - 1) as u16 + self.X as u16 & 0xFF;
                self.set_contents(index, value);
            }
            ZeroPageY => {
                let index = self.contents(pc - 1) as u16 + self.Y as u16 & 0xFF;
                self.set_contents(index, value);
            }
            Absolute => {
                let index = self.contents(pc - 2) as u16 + ((self.contents(pc - 1) as u16) << 8);
                self.set_contents(index, value);
            }
            AbsoluteX => {
                let index = (self.contents(pc - 2) as u16).wrapping_add((self.contents(pc - 1) as u16) << 8).wrapping_add(self.X as u16);
                self.set_contents(index,value);
            }
            AbsoluteY => {
                let index = (self.contents(pc - 2) as u16).wrapping_add((self.contents(pc - 1) as u16) << 8).wrapping_add(self.Y as u16);
                self.set_contents(index,value);
            }
            IndirectX => {
                let indexindex = self.contents(pc - 1) as u16 + self.X as u16 & 0xFF;
                let index = self.contents(indexindex) as u16 + ((self.contents(indexindex + 1 & 0xFF) as u16) << 8);
                self.set_contents(index,value);
            }
            IndirectY => {
                let indexindex = self.contents(pc - 1) as u16;
                let index = (self.contents(indexindex) as u16).wrapping_add((self.contents(indexindex + 1 & 0xFF) as u16) << 8).wrapping_add(self.Y as u16);
                self.set_contents(index,value);
            }
            _ => (),
        }
    }
    fn jump_interrupt(&mut self, location: u16, isbrk: bool) {
        let pc = self.PC;
        self.stack_push((pc >> 8) as u8);
        self.stack_push(pc as u8);
        let p_push = self.P | if isbrk { mask(Flags::Break) } else { 0 } | mask(Flags::BadBit);
        self.stack_push(p_push);
        self.PC = self.contents(location) as u16 + ((self.contents(location + 1) as u16) << 8);
        self.cycles_delay = 7;
        self.interrupt = Interrupt::Null;
    }
    fn branch(&mut self, delta: u8) {
        self.PC = ((self.PC as i32) + (delta as i8 as i32)) as u16;
    }
    fn pprint_arg(&mut self,addrmode: AddressingMode) -> String {
        use instruction::AddressingMode::*;
        let pc = self.PC;
        match addrmode {
            Immediate => format!("{:X}",self.contents(pc - 1)),
            Relative =>  format!("{:X}",self.contents(pc - 1)),
            Accumulator => "A".to_string(),
            ZeroPage => {
                let index = self.contents(pc - 1) as u16;
                format!("{:X}",index)
            }
            ZeroPageX => {
                let index = self.contents(pc - 1) as u16 + self.X as u16 & 0xFF;
                format!("{:X}",index)
            }
            ZeroPageY => {
                let index = self.contents(pc - 1) as u16 + self.Y as u16 & 0xFF;
                format!("{:X}",index)
            }
            Absolute => {
                let index = self.contents(pc - 2) as u16 + ((self.contents(pc - 1) as u16) << 8);
                format!("{:X}",index)
            }
            AbsoluteX => {
                let index = (self.contents(pc - 2) as u16).wrapping_add((self.contents(pc - 1) as u16) << 8).wrapping_add(self.X as u16);
                format!("{:X}",index)
            }
            AbsoluteY => {
                let index = (self.contents(pc - 2) as u16).wrapping_add((self.contents(pc - 1) as u16) << 8).wrapping_add(self.Y as u16);
                format!("{:X}",index)
            }
            IndirectX => {
                let indexindex = self.contents(pc - 1) as u16 + self.X as u16 & 0xFF;
                let index = self.contents(indexindex) as u16 + ((self.contents(indexindex + 1 & 0xFF) as u16) << 8);
                format!("{:X}",index)
            }
            IndirectY => {
                let indexindex = self.contents(pc - 1) as u16;
                let index = (self.contents(indexindex) as u16).wrapping_add((self.contents(indexindex + 1 & 0xFF) as u16) << 8).wrapping_add(self.Y as u16);
                format!("{:X}",index)
            }
            _ => "".to_string(),
        }
    }
    pub fn debug_print(&mut self) {
        let pc = self.PC;
        let instr = opcode_to_instr(self.contents(pc));
        self.PC += 1 + stride(instr.addrmode);
        let arg = self.pprint_arg(instr.addrmode);
        println!("{:X}: {:?},{:?} @ {}; A: {:X}, X: {:X}, Y: {:X}, P: {:X}, SP: {:X}",self.PC - (1 + stride(instr.addrmode)),instr.instr,instr.addrmode,arg,self.A,self.X,self.Y,self.P,self.SP);
        self.PC -= 1 + stride(instr.addrmode);
    }
    pub fn instr(&mut self) {
        use instruction::InstrType::*;
        use instruction::AddressingMode::*;

        let pc = self.PC;
        let instr = opcode_to_instr(self.contents(pc));
        self.PC += 1 + stride(instr.addrmode);
        //Calculate timing
        let pc = self.PC;
        let pagecross = match instr.addrmode {
            AbsoluteX => {
                let index = self.contents(pc - 2) as u16 + ((self.contents(pc - 1) as u16) << 8);
                index & 0xFF00 != index.wrapping_add(self.X as u16) & 0xFF00
            }
            AbsoluteY => {
                let index = self.contents(pc - 2) as u16 + ((self.contents(pc - 1) as u16) << 8);
                index & 0xFF00 != index.wrapping_add(self.Y as u16) & 0xFF00
            }
            IndirectY => {
                let indexindex = self.contents(pc - 1) as u16;
                let index = self.contents(indexindex) as u16 + ((self.contents((indexindex + 1) & 0xFF) as u16) << 8);
                index & 0xFF00 != index.wrapping_add(self.Y as u16) & 0xFF00
            }
            Relative => {
                let delta = self.contents(pc - 1) as i8 as i32;
                (((self.PC as i32) + delta) as u16) & 0xFF00 != self.PC & 0xFF00
            }
            _ => false
        };
        self.cycles_delay = instr.timing as u32;
        match instr.instr {
            ADC | AND | CMP | EOR | LDA | LDX | LDY | NOP | ORA | SBC | LAR | LAX => {
                if pagecross {
                    self.cycles_delay += 1;
                }
            }
            BCC => if self.P & mask(Flags::Carry) == 0 {
                self.cycles_delay += 1;
                if pagecross {
                    self.cycles_delay += 1;
                }
            },
            BCS => if self.P & mask(Flags::Carry) != 0 {
                self.cycles_delay += 1;
                if pagecross {
                    self.cycles_delay += 1;
                }
            },
            BEQ => if self.P & mask(Flags::Zero) != 0 {
                self.cycles_delay += 1;
                if pagecross {
                    self.cycles_delay += 1;
                }
            },
            BMI => if self.P & mask(Flags::Sign) != 0 {
                self.cycles_delay += 1;
                if pagecross {
                    self.cycles_delay += 1;
                }
            },
            BPL => if self.P & mask(Flags::Sign) == 0 {
                self.cycles_delay += 1;
                if pagecross {
                    self.cycles_delay += 1;
                }
            },
            BNE => if self.P & mask(Flags::Zero) == 0 {
                self.cycles_delay += 1;
                if pagecross {
                    self.cycles_delay += 1;
                }
            },
            BVC => if self.P & mask(Flags::Overflow) == 0 {
                self.cycles_delay += 1;
                if pagecross {
                    self.cycles_delay += 1;
                }
            },
            BVS => if self.P & mask(Flags::Overflow) != 0 {
                self.cycles_delay += 1;
                if pagecross {
                    self.cycles_delay += 1;
                }
            },
            _ => ()
        }
        //all instructions fetch from memory, except stx,sty,sta,aax,and axa
        let mut argument = match instr.instr {
            STX | STY | STA | AAX | AXA => 0,
            _ => self.fetch_argument(instr.addrmode),
        };
        match instr.instr {
            ADC => {
                let result: u16 = self.A as u16 + argument as u16 + (self.P & mask(Flags::Carry)) as u16;
                self.P &= maskout(Flags::Carry) & maskout(Flags::Overflow);
                if result > 0xFF {
                    self.P |= mask(Flags::Carry)
                }
                let result = result as u8;
                set_flags(&mut self.P,result);
                if signum(self.A as i8) == signum(argument as i8)
                    && signum(result as i8) != signum(self.A as i8)
                {
                    self.P |= mask(Flags::Overflow);
                }
                self.A = result as u8;
            }
            AND => {
                self.A &= argument;
                set_flags(&mut self.P,self.A);
            }
            ASL => {
                self.P &= maskout(Flags::Carry);
                if argument & 0x80 != 0 {
                    self.P |= mask(Flags::Carry)
                }
                argument <<= 1;
                self.replace_argument(instr.addrmode, argument);
                set_flags(&mut self.P,argument)
            }
            BCC => {
                if self.P & mask(Flags::Carry) == 0 { self.branch(argument); }
            }
            BCS => {
                if self.P & mask(Flags::Carry) != 0 { self.branch(argument); }
            }
            BEQ => {
                if self.P & mask(Flags::Zero) != 0 { self.branch(argument); }
            }
            BIT => {
                self.P &= maskout(Flags::Zero) & maskout(Flags::Overflow) & maskout(Flags::Sign);
                if self.A & argument == 0 {
                    self.P |= mask(Flags::Zero);
                }
                self.P |= argument & mask(Flags::Overflow);
                self.P |= argument & mask(Flags::Sign);
            }
            BMI => {
                if self.P & mask(Flags::Sign) != 0 { self.branch(argument); }
            }
            BNE => {
                if self.P & mask(Flags::Zero) == 0 { self.branch(argument); }
            }
            BPL => {
                if self.P & mask(Flags::Sign) == 0 { self.branch(argument); }
            }
            BRK => {
                if self.P & mask(Flags::Interrupt) != 0 {
                    self.jump_interrupt(0xFFFE, true);
                } else {
                    self.cycles_delay = 1;
                }
            }
            BVC => {
                if self.P & mask(Flags::Overflow) == 0 { self.branch(argument); }
            }
            BVS => {
                if self.P & mask(Flags::Overflow) != 0 { self.branch(argument); }
            }
            CLC => {
                self.P &= maskout(Flags::Carry);
            }
            CLD => {
                self.P &= maskout(Flags::Decimal);
            }
            CLI => {
                self.P &= maskout(Flags::Interrupt);
            }
            CLV => {
                self.P &= maskout(Flags::Overflow);
            }
            CMP => {
                self.P &= maskout(Flags::Carry) & maskout(Flags::Sign) & maskout(Flags::Zero);
                if self.A >= argument {
                    self.P |= mask(Flags::Carry);
                }
                if self.A == argument {
                    self.P |= mask(Flags::Zero);
                }
                if (self.A.wrapping_sub(argument)) & 0x80 != 0 {
                    self.P |= mask(Flags::Sign);
                }
            }
            CPX => {
                self.P &= maskout(Flags::Carry) & maskout(Flags::Sign) & maskout(Flags::Zero);
                if self.X >= argument {
                    self.P |= mask(Flags::Carry);
                }
                if self.X == argument {
                    self.P |= mask(Flags::Zero);
                }
                if (self.X.wrapping_sub(argument)) & 0x80 != 0 {
                    self.P |= mask(Flags::Sign);
                }
            }
            CPY => {
                self.P &= maskout(Flags::Carry) & maskout(Flags::Sign) & maskout(Flags::Zero);
                if self.Y >= argument {
                    self.P |= mask(Flags::Carry);
                }
                if self.Y == argument {
                    self.P |= mask(Flags::Zero);
                }
                if (self.Y.wrapping_sub(argument)) & 0x80 != 0 {
                    self.P |= mask(Flags::Sign);
                }
            }
            DEC => {
                argument = argument.wrapping_sub(1);
                set_flags(&mut self.P,argument);
                self.replace_argument(instr.addrmode, argument);
            }
            DEX => {
                self.X = self.X.wrapping_sub(1);
                set_flags(&mut self.P,self.X);
            }
            DEY => {
                self.Y = self.Y.wrapping_sub(1);
                set_flags(&mut self.P,self.Y);
            }
            EOR => {
                self.A ^= argument;
                set_flags(&mut self.P,self.A);
            }
            INC => {
                argument = argument.wrapping_add(1);
                set_flags(&mut self.P,argument);
                self.replace_argument(instr.addrmode, argument);
            }
            INX => {
                self.X = self.X.wrapping_add(1);
                set_flags(&mut self.P,self.X);
            }
            INY => {
                self.Y = self.Y.wrapping_add(1);argument = argument.wrapping_sub(1);
                set_flags(&mut self.P,argument);
                self.replace_argument(instr.addrmode, argument);
                set_flags(&mut self.P,self.Y);
            }
            JMP => {
                let pc = self.PC;
                let mut index = self.contents(pc - 2) as u16 + ((self.contents(pc - 1) as u16) << 8);
                if instr.addrmode == Indirect {
                    let mut ip1 = index + 1;
                    if (index & 0xFF) == 0xFF {
                        ip1 = index & 0xFF00;
                    }
                    index = self.contents(index) as u16 + ((self.contents(ip1) as u16) << 8);
                }
                self.PC = index;
            }
            JSR => {
                let pc = self.PC;
                self.stack_push(((pc - 1) >> 8) as u8);
                self.stack_push((pc - 1) as u8);
                self.PC = self.contents(pc - 2) as u16 + ((self.contents(pc - 1) as u16) << 8);
            }
            LDA => {
                self.A = argument;
                set_flags(&mut self.P,self.A);
            }
            LDX => {
                self.X = argument;
                set_flags(&mut self.P,self.X);
            }
            LDY => {
                self.Y = argument;
                set_flags(&mut self.P,self.Y);
            }
            LSR => {
                let result = argument >> 1;
                set_flags(&mut self.P,result);
                self.P &= maskout(Flags::Carry);
                if argument & 1 != 0 {
                    self.P |= mask(Flags::Carry);
                }
                self.replace_argument(instr.addrmode, result);
            }
            NOP => (),
            ORA => {
                self.A |= argument;
                set_flags(&mut self.P,self.A);
            }
            PHA => {
                let a = self.A;
                self.stack_push(a);
            }
            PHP => {
                let p = self.P | mask(Flags::BadBit) | mask(Flags::Break);
                self.stack_push(p);
            }
            PLA => {
                self.A = self.stack_pop();
                set_flags(&mut self.P,self.A);
            }
            PLP => {
                self.P = self.stack_pop() & maskout(Flags::BadBit) & maskout(Flags::Break);
            }
            ROL => {
                let new = self.P & mask(Flags::Carry);
                self.P &= maskout(Flags::Carry);
                if argument & 0x80 != 0 {
                    self.P |= mask(Flags::Carry);
                }
                argument <<= 1;
                argument += new;
                set_flags(&mut self.P,argument);
                self.replace_argument(instr.addrmode, argument);
            }
            ROR => {
                let new = (self.P & mask(Flags::Carry)) << 7;
                self.P &= maskout(Flags::Carry);
                if argument & 1 != 0 {
                    self.P |= mask(Flags::Carry);
                }
                argument >>= 1;
                argument += new;
                set_flags(&mut self.P,argument);
                self.replace_argument(instr.addrmode, argument);
            }
            RTI => {
                self.P = self.stack_pop() & maskout(Flags::BadBit) & maskout(Flags::Break);
                let lo = self.stack_pop() as u16;
                let hi = self.stack_pop() as u16;
                self.PC = (hi << 8) + lo;
            }
            RTS => {
                let lo = self.stack_pop() as u16;
                let hi = self.stack_pop() as u16;
                self.PC = (hi << 8) + lo + 1;
            }
            SBC => {
                argument ^= 0xFF;
                let result: u16 = self.A as u16 + argument as u16 + (self.P & mask(Flags::Carry)) as u16;
                self.P &= maskout(Flags::Carry) & maskout(Flags::Overflow);
                if result > 0xFF {
                    self.P |= mask(Flags::Carry)
                }
                let result = result as u8;
                set_flags(&mut self.P,result);
                if signum(self.A as i8) == signum(argument as i8)
                    && signum(result as i8) != signum(self.A as i8)
                {
                    self.P |= mask(Flags::Overflow);
                }
                self.A = result as u8;
            }
            SEC => {
                self.P |= mask(Flags::Carry);
            }
            SED => {
                self.P |= mask(Flags::Decimal);
            }
            SEI => {
                self.P |= mask(Flags::Interrupt);
            }
            STA => {
                let a = self.A;
                self.replace_argument(instr.addrmode, a);
            }
            STX => {
                let x = self.X;
                self.replace_argument(instr.addrmode, x);
            }
            STY => {
                let y = self.Y;
                self.replace_argument(instr.addrmode, y);
            }
            TAX => {
                self.X = self.A;
                set_flags(&mut self.P,self.X);
            }
            TAY => {
                self.Y = self.A;
                set_flags(&mut self.P,self.Y);
            }
            TSX => {
                self.X = self.SP;
                set_flags(&mut self.P,self.X);
            }
            TXA => {
                self.A = self.X;
                set_flags(&mut self.P,self.A);
            }
            TXS => {
                self.SP = self.X;
            }
            TYA => {
                self.A = self.Y;
                set_flags(&mut self.P,self.A);
            }
            LAX => {
                self.A = argument;
                self.X = argument;
                set_flags(&mut self.P,argument);
            }
            AAX => {
                let result = self.A & self.X;
                self.replace_argument(instr.addrmode,result);
            }
            DCP => {
                argument = argument.wrapping_sub(1);
                self.replace_argument(instr.addrmode, argument);
                self.P &= maskout(Flags::Carry) & maskout(Flags::Sign) & maskout(Flags::Zero);
                if self.A >= argument {
                    self.P |= mask(Flags::Carry);
                }
                if self.A == argument {
                    self.P |= mask(Flags::Zero);
                }
                if (self.A.wrapping_sub(argument)) & 0x80 != 0 {
                    self.P |= mask(Flags::Sign);
                }
            }
            ISC => {
                argument = argument.wrapping_add(1);
                self.replace_argument(instr.addrmode, argument);
                argument ^= 0xFF;
                let result: u16 = self.A as u16 + argument as u16 + (self.P & mask(Flags::Carry)) as u16;
                self.P &= maskout(Flags::Carry) & maskout(Flags::Overflow);
                if result > 0xFF {
                    self.P |= mask(Flags::Carry)
                }
                let result = result as u8;
                set_flags(&mut self.P,result);
                if signum(self.A as i8) == signum(argument as i8)
                    && signum(result as i8) != signum(self.A as i8)
                {
                    self.P |= mask(Flags::Overflow);
                }
                self.A = result as u8;
            }
            SLO => {
                self.P &= maskout(Flags::Carry);
                if argument & 0x80 != 0 {
                    self.P |= mask(Flags::Carry)
                }
                argument <<= 1;
                self.replace_argument(instr.addrmode, argument);
                self.A |= argument;
                set_flags(&mut self.P,self.A);
            }
            RLA => {
                let new = self.P & mask(Flags::Carry);
                self.P &= maskout(Flags::Carry);
                if argument & 0x80 != 0 {
                    self.P |= mask(Flags::Carry);
                }
                argument <<= 1;
                argument += new;
                self.replace_argument(instr.addrmode, argument);
                self.A &= argument;
                set_flags(&mut self.P,self.A);
            }
            SRE => {
                let result = argument >> 1;
                self.P &= maskout(Flags::Carry);
                if argument & 1 != 0 {
                    self.P |= mask(Flags::Carry);
                }
                self.replace_argument(instr.addrmode, result);
                self.A ^= result;
                set_flags(&mut self.P,self.A);
            }
            RRA => {
                let new = (self.P & mask(Flags::Carry)) << 7;
                self.P &= maskout(Flags::Carry);
                if argument & 1 != 0 {
                    self.P |= mask(Flags::Carry);
                }
                argument >>= 1;
                argument += new;
                self.replace_argument(instr.addrmode, argument);

                let result: u16 = self.A as u16 + argument as u16 + (self.P & mask(Flags::Carry)) as u16;
                self.P &= maskout(Flags::Carry) & maskout(Flags::Overflow);
                if result > 0xFF {
                    self.P |= mask(Flags::Carry)
                }
                let result = result as u8;
                set_flags(&mut self.P,result);
                if signum(self.A as i8) == signum(argument as i8)
                    && signum(result as i8) != signum(self.A as i8)
                {
                    self.P |= mask(Flags::Overflow);
                }
                self.A = result as u8;
            }
            //I have decided to only implement opcodes needed by nestest; the rest will be left unimplemented to decrease
            //file size and time
            _ => panic!("Bad opcode: {:?}", instr.instr),
        }
    }
}
