#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Accumulator,
    Relative,
    Implied,
    Indirect,
}
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum InstrType {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    AAC,
    AAX,
    ARR,
    ASR,
    ATX,
    AXA,
    AXS,
    DCP,
    ISC,
    KIL,
    LAR,
    LAX,
    RLA,
    RRA,
    SLO,
    SRE,
    SXA,
    SYA,
    XAA,
    XAS,
}

pub struct Instruction {
    pub instr: InstrType,
    pub addrmode: AddressingMode,
    pub timing: u8,
}

pub fn stride(addrmode: AddressingMode) -> u16 {
    use self::AddressingMode::*;
    match addrmode {
        Immediate => 1,
        ZeroPage => 1,
        ZeroPageX => 1,
        ZeroPageY => 1,
        Absolute => 2,
        AbsoluteX => 2,
        AbsoluteY => 2,
        IndirectX => 1,
        IndirectY => 1,
        Indirect => 2,
        Accumulator => 0,
        Relative => 1,
        Implied => 0,
    }
}

//beware: massive data tables await

pub fn opcode_to_instr(opcode: u8) -> Instruction {
    use self::InstrType::*;
    use self::AddressingMode::*;
    match opcode {
        0x69 => Instruction {
            instr: ADC,
            addrmode: Immediate,
            timing: 2,
        },
        0x65 => Instruction {
            instr: ADC,
            addrmode: ZeroPage,
            timing: 3,
        },
        0x75 => Instruction {
            instr: ADC,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0x6D => Instruction {
            instr: ADC,
            addrmode: Absolute,
            timing: 4,
        },
        0x7D => Instruction {
            instr: ADC,
            addrmode: AbsoluteX,
            timing: 4,
        },
        0x79 => Instruction {
            instr: ADC,
            addrmode: AbsoluteY,
            timing: 4,
        },
        0x61 => Instruction {
            instr: ADC,
            addrmode: IndirectX,
            timing: 6,
        },
        0x71 => Instruction {
            instr: ADC,
            addrmode: IndirectY,
            timing: 5,
        },
        0x29 => Instruction {
            instr: AND,
            addrmode: Immediate,
            timing: 2,
        },
        0x25 => Instruction {
            instr: AND,
            addrmode: ZeroPage,
            timing: 3,
        },
        0x35 => Instruction {
            instr: AND,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0x2D => Instruction {
            instr: AND,
            addrmode: Absolute,
            timing: 4,
        },
        0x3D => Instruction {
            instr: AND,
            addrmode: AbsoluteX,
            timing: 4,
        },
        0x39 => Instruction {
            instr: AND,
            addrmode: AbsoluteY,
            timing: 4,
        },
        0x21 => Instruction {
            instr: AND,
            addrmode: IndirectX,
            timing: 6,
        },
        0x31 => Instruction {
            instr: AND,
            addrmode: IndirectY,
            timing: 5,
        },
        0x0A => Instruction {
            instr: ASL,
            addrmode: Accumulator,
            timing: 2,
        },
        0x06 => Instruction {
            instr: ASL,
            addrmode: ZeroPage,
            timing: 5,
        },
        0x16 => Instruction {
            instr: ASL,
            addrmode: ZeroPageX,
            timing: 6,
        },
        0x0E => Instruction {
            instr: ASL,
            addrmode: Absolute,
            timing: 6,
        },
        0x1E => Instruction {
            instr: ASL,
            addrmode: AbsoluteX,
            timing: 7,
        },
        0x90 => Instruction {
            instr: BCC,
            addrmode: Relative,
            timing: 2,
        },
        0xB0 => Instruction {
            instr: BCS,
            addrmode: Relative,
            timing: 2,
        },
        0xF0 => Instruction {
            instr: BEQ,
            addrmode: Relative,
            timing: 2,
        },
        0x30 => Instruction {
            instr: BMI,
            addrmode: Relative,
            timing: 2,
        },
        0xD0 => Instruction {
            instr: BNE,
            addrmode: Relative,
            timing: 2,
        },
        0x10 => Instruction {
            instr: BPL,
            addrmode: Relative,
            timing: 2,
        },
        0x50 => Instruction {
            instr: BVC,
            addrmode: Relative,
            timing: 2,
        },
        0x70 => Instruction {
            instr: BVS,
            addrmode: Relative,
            timing: 2,
        },
        0x24 => Instruction {
            instr: BIT,
            addrmode: ZeroPage,
            timing: 3,
        },
        0x2C => Instruction {
            instr: BIT,
            addrmode: Absolute,
            timing: 4,
        },
        0x00 => Instruction {
            instr: BRK,
            addrmode: Implied,
            timing: 0,
        },
        0x18 => Instruction {
            instr: CLC,
            addrmode: Implied,
            timing: 2,
        },
        0xD8 => Instruction {
            instr: CLD,
            addrmode: Implied,
            timing: 2,
        },
        0x58 => Instruction {
            instr: CLI,
            addrmode: Implied,
            timing: 2,
        },
        0xB8 => Instruction {
            instr: CLV,
            addrmode: Implied,
            timing: 2,
        },
        0xC9 => Instruction {
            instr: CMP,
            addrmode: Immediate,
            timing: 2,
        },
        0xC5 => Instruction {
            instr: CMP,
            addrmode: ZeroPage,
            timing: 3,
        },
        0xD5 => Instruction {
            instr: CMP,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0xCD => Instruction {
            instr: CMP,
            addrmode: Absolute,
            timing: 4,
        },
        0xDD => Instruction {
            instr: CMP,
            addrmode: AbsoluteX,
            timing: 4,
        },
        0xD9 => Instruction {
            instr: CMP,
            addrmode: AbsoluteY,
            timing: 4,
        },
        0xC1 => Instruction {
            instr: CMP,
            addrmode: IndirectX,
            timing: 6,
        },
        0xD1 => Instruction {
            instr: CMP,
            addrmode: IndirectY,
            timing: 5,
        },
        0xE0 => Instruction {
            instr: CPX,
            addrmode: Immediate,
            timing: 2,
        },
        0xE4 => Instruction {
            instr: CPX,
            addrmode: ZeroPage,
            timing: 3,
        },
        0xEC => Instruction {
            instr: CPX,
            addrmode: Absolute,
            timing: 4,
        },
        0xC0 => Instruction {
            instr: CPY,
            addrmode: Immediate,
            timing: 2,
        },
        0xC4 => Instruction {
            instr: CPY,
            addrmode: ZeroPage,
            timing: 3,
        },
        0xCC => Instruction {
            instr: CPY,
            addrmode: Absolute,
            timing: 4,
        },
        0xC6 => Instruction {
            instr: DEC,
            addrmode: ZeroPage,
            timing: 5,
        },
        0xD6 => Instruction {
            instr: DEC,
            addrmode: ZeroPageX,
            timing: 6,
        },
        0xCE => Instruction {
            instr: DEC,
            addrmode: Absolute,
            timing: 6,
        },
        0xDE => Instruction {
            instr: DEC,
            addrmode: AbsoluteX,
            timing: 7,
        },
        0xCA => Instruction {
            instr: DEX,
            addrmode: Implied,
            timing: 2,
        },
        0x88 => Instruction {
            instr: DEY,
            addrmode: Implied,
            timing: 2,
        },
        0x49 => Instruction {
            instr: EOR,
            addrmode: Immediate,
            timing: 2,
        },
        0x45 => Instruction {
            instr: EOR,
            addrmode: ZeroPage,
            timing: 3,
        },
        0x55 => Instruction {
            instr: EOR,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0x4D => Instruction {
            instr: EOR,
            addrmode: Absolute,
            timing: 4,
        },
        0x5D => Instruction {
            instr: EOR,
            addrmode: AbsoluteX,
            timing: 4,
        },
        0x59 => Instruction {
            instr: EOR,
            addrmode: AbsoluteY,
            timing: 4,
        },
        0x41 => Instruction {
            instr: EOR,
            addrmode: IndirectX,
            timing: 6,
        },
        0x51 => Instruction {
            instr: EOR,
            addrmode: IndirectY,
            timing: 5,
        },
        0xE6 => Instruction {
            instr: INC,
            addrmode: ZeroPage,
            timing: 5,
        },
        0xF6 => Instruction {
            instr: INC,
            addrmode: ZeroPageX,
            timing: 6,
        },
        0xEE => Instruction {
            instr: INC,
            addrmode: Absolute,
            timing: 6,
        },
        0xFE => Instruction {
            instr: INC,
            addrmode: AbsoluteX,
            timing: 7,
        },
        0xE8 => Instruction {
            instr: INX,
            addrmode: Implied,
            timing: 2,
        },
        0xC8 => Instruction {
            instr: INY,
            addrmode: Implied,
            timing: 2,
        },
        0x4C => Instruction {
            instr: JMP,
            addrmode: Absolute,
            timing: 3,
        },
        0x6C => Instruction {
            instr: JMP,
            addrmode: Indirect,
            timing: 5,
        },
        0x20 => Instruction {
            instr: JSR,
            addrmode: Absolute,
            timing: 6,
        },
        0xA9 => Instruction {
            instr: LDA,
            addrmode: Immediate,
            timing: 2,
        },
        0xA5 => Instruction {
            instr: LDA,
            addrmode: ZeroPage,
            timing: 3,
        },
        0xB5 => Instruction {
            instr: LDA,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0xAD => Instruction {
            instr: LDA,
            addrmode: Absolute,
            timing: 4,
        },
        0xBD => Instruction {
            instr: LDA,
            addrmode: AbsoluteX,
            timing: 4,
        },
        0xB9 => Instruction {
            instr: LDA,
            addrmode: AbsoluteY,
            timing: 4,
        },
        0xA1 => Instruction {
            instr: LDA,
            addrmode: IndirectX,
            timing: 6,
        },
        0xB1 => Instruction {
            instr: LDA,
            addrmode: IndirectY,
            timing: 5,
        },
        0xA2 => Instruction {
            instr: LDX,
            addrmode: Immediate,
            timing: 2,
        },
        0xA6 => Instruction {
            instr: LDX,
            addrmode: ZeroPage,
            timing: 3,
        },
        0xB6 => Instruction {
            instr: LDX,
            addrmode: ZeroPageY,
            timing: 4,
        },
        0xAE => Instruction {
            instr: LDX,
            addrmode: Absolute,
            timing: 4,
        },
        0xBE => Instruction {
            instr: LDX,
            addrmode: AbsoluteY,
            timing: 4,
        },
        0xA0 => Instruction {
            instr: LDY,
            addrmode: Immediate,
            timing: 2,
        },
        0xA4 => Instruction {
            instr: LDY,
            addrmode: ZeroPage,
            timing: 3,
        },
        0xB4 => Instruction {
            instr: LDY,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0xAC => Instruction {
            instr: LDY,
            addrmode: Absolute,
            timing: 4,
        },
        0xBC => Instruction {
            instr: LDY,
            addrmode: AbsoluteX,
            timing: 4,
        },
        0x4A => Instruction {
            instr: LSR,
            addrmode: Accumulator,
            timing: 2,
        },
        0x46 => Instruction {
            instr: LSR,
            addrmode: ZeroPage,
            timing: 5,
        },
        0x56 => Instruction {
            instr: LSR,
            addrmode: ZeroPageX,
            timing: 6,
        },
        0x4E => Instruction {
            instr: LSR,
            addrmode: Absolute,
            timing: 6,
        },
        0x5E => Instruction {
            instr: LSR,
            addrmode: AbsoluteX,
            timing: 7,
        },
        0xEA => Instruction {
            instr: NOP,
            addrmode: Implied,
            timing: 2,
        },
        0x09 => Instruction {
            instr: ORA,
            addrmode: Immediate,
            timing: 2,
        },
        0x05 => Instruction {
            instr: ORA,
            addrmode: ZeroPage,
            timing: 3,
        },
        0x15 => Instruction {
            instr: ORA,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0x0D => Instruction {
            instr: ORA,
            addrmode: Absolute,
            timing: 4,
        },
        0x1D => Instruction {
            instr: ORA,
            addrmode: AbsoluteX,
            timing: 4,
        },
        0x19 => Instruction {
            instr: ORA,
            addrmode: AbsoluteY,
            timing: 4,
        },
        0x01 => Instruction {
            instr: ORA,
            addrmode: IndirectX,
            timing: 6,
        },
        0x11 => Instruction {
            instr: ORA,
            addrmode: IndirectY,
            timing: 5,
        },
        0x48 => Instruction {
            instr: PHA,
            addrmode: Implied,
            timing: 3,
        },
        0x08 => Instruction {
            instr: PHP,
            addrmode: Implied,
            timing: 3,
        },
        0x68 => Instruction {
            instr: PLA,
            addrmode: Implied,
            timing: 4,
        },
        0x28 => Instruction {
            instr: PLP,
            addrmode: Implied,
            timing: 4,
        },
        0x2A => Instruction {
            instr: ROL,
            addrmode: Accumulator,
            timing: 2,
        },
        0x26 => Instruction {
            instr: ROL,
            addrmode: ZeroPage,
            timing: 5,
        },
        0x36 => Instruction {
            instr: ROL,
            addrmode: ZeroPageX,
            timing: 6,
        },
        0x2E => Instruction {
            instr: ROL,
            addrmode: Absolute,
            timing: 6,
        },
        0x3E => Instruction {
            instr: ROL,
            addrmode: AbsoluteX,
            timing: 7,
        },
        0x6A => Instruction {
            instr: ROR,
            addrmode: Accumulator,
            timing: 2,
        },
        0x66 => Instruction {
            instr: ROR,
            addrmode: ZeroPage,
            timing: 5,
        },
        0x76 => Instruction {
            instr: ROR,
            addrmode: ZeroPageX,
            timing: 6,
        },
        0x6E => Instruction {
            instr: ROR,
            addrmode: Absolute,
            timing: 6,
        },
        0x7E => Instruction {
            instr: ROR,
            addrmode: AbsoluteX,
            timing: 7,
        },
        0x40 => Instruction {
            instr: RTI,
            addrmode: Implied,
            timing: 6,
        },
        0x60 => Instruction {
            instr: RTS,
            addrmode: Implied,
            timing: 6,
        },
        0xE9 => Instruction {
            instr: SBC,
            addrmode: Immediate,
            timing: 2,
        },
        0xE5 => Instruction {
            instr: SBC,
            addrmode: ZeroPage,
            timing: 3,
        },
        0xF5 => Instruction {
            instr: SBC,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0xED => Instruction {
            instr: SBC,
            addrmode: Absolute,
            timing: 4,
        },
        0xFD => Instruction {
            instr: SBC,
            addrmode: AbsoluteX,
            timing: 4,
        },
        0xF9 => Instruction {
            instr: SBC,
            addrmode: AbsoluteY,
            timing: 4,
        },
        0xE1 => Instruction {
            instr: SBC,
            addrmode: IndirectX,
            timing: 6,
        },
        0xF1 => Instruction {
            instr: SBC,
            addrmode: IndirectY,
            timing: 5,
        },
        0x38 => Instruction {
            instr: SEC,
            addrmode: Implied,
            timing: 2,
        },
        0xF8 => Instruction {
            instr: SED,
            addrmode: Implied,
            timing: 2,
        },
        0x78 => Instruction {
            instr: SEI,
            addrmode: Implied,
            timing: 2,
        },
        0x85 => Instruction {
            instr: STA,
            addrmode: ZeroPage,
            timing: 3,
        },
        0x95 => Instruction {
            instr: STA,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0x8D => Instruction {
            instr: STA,
            addrmode: Absolute,
            timing: 4,
        },
        0x9D => Instruction {
            instr: STA,
            addrmode: AbsoluteX,
            timing: 5,
        },
        0x99 => Instruction {
            instr: STA,
            addrmode: AbsoluteY,
            timing: 5,
        },
        0x81 => Instruction {
            instr: STA,
            addrmode: IndirectX,
            timing: 6,
        },
        0x91 => Instruction {
            instr: STA,
            addrmode: IndirectY,
            timing: 6,
        },
        0x86 => Instruction {
            instr: STX,
            addrmode: ZeroPage,
            timing: 3,
        },
        0x96 => Instruction {
            instr: STX,
            addrmode: ZeroPageY,
            timing: 4,
        },
        0x8E => Instruction {
            instr: STX,
            addrmode: Absolute,
            timing: 4,
        },
        0x84 => Instruction {
            instr: STY,
            addrmode: ZeroPage,
            timing: 3,
        },
        0x94 => Instruction {
            instr: STY,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0x8C => Instruction {
            instr: STY,
            addrmode: Absolute,
            timing: 4,
        },
        0xAA => Instruction {
            instr: TAX,
            addrmode: Implied,
            timing: 2,
        },
        0xA8 => Instruction {
            instr: TAY,
            addrmode: Implied,
            timing: 2,
        },
        0xBA => Instruction {
            instr: TSX,
            addrmode: Implied,
            timing: 2,
        },
        0x8A => Instruction {
            instr: TXA,
            addrmode: Implied,
            timing: 2,
        },
        0x9A => Instruction {
            instr: TXS,
            addrmode: Implied,
            timing: 2,
        },
        0x98 => Instruction {
            instr: TYA,
            addrmode: Implied,
            timing: 2,
        },
        0x0B => Instruction {
            instr: AAC,
            addrmode: Immediate,
            timing: 2,
        },
        0x2B => Instruction {
            instr: AAC,
            addrmode: Immediate,
            timing: 2,
        },
        0x87 => Instruction {
            instr: AAX,
            addrmode: ZeroPage,
            timing: 3,
        },
        0x97 => Instruction {
            instr: AAX,
            addrmode: ZeroPageY,
            timing: 4,
        },
        0x83 => Instruction {
            instr: AAX,
            addrmode: IndirectX,
            timing: 6,
        },
        0x8F => Instruction {
            instr: AAX,
            addrmode: Absolute,
            timing: 4,
        },
        0x6B => Instruction {
            instr: ARR,
            addrmode: Immediate,
            timing: 2,
        },
        0x4B => Instruction {
            instr: ASR,
            addrmode: Immediate,
            timing: 2,
        },
        0xAB => Instruction {
            instr: ATX,
            addrmode: Immediate,
            timing: 2,
        },
        0x9F => Instruction {
            instr: AXA,
            addrmode: AbsoluteY,
            timing: 5,
        },
        0x93 => Instruction {
            instr: AXA,
            addrmode: IndirectY,
            timing: 6,
        },
        0xCB => Instruction {
            instr: AXS,
            addrmode: Immediate,
            timing: 2,
        },
        0xC7 => Instruction {
            instr: DCP,
            addrmode: ZeroPage,
            timing: 5,
        },
        0xD7 => Instruction {
            instr: DCP,
            addrmode: ZeroPageX,
            timing: 6,
        },
        0xCF => Instruction {
            instr: DCP,
            addrmode: Absolute,
            timing: 6,
        },
        0xDF => Instruction {
            instr: DCP,
            addrmode: AbsoluteX,
            timing: 7,
        },
        0xDB => Instruction {
            instr: DCP,
            addrmode: AbsoluteY,
            timing: 7,
        },
        0xC3 => Instruction {
            instr: DCP,
            addrmode: IndirectX,
            timing: 8,
        },
        0xD3 => Instruction {
            instr: DCP,
            addrmode: IndirectY,
            timing: 8,
        },
        0x04 => Instruction {
            instr: NOP,
            addrmode: ZeroPage,
            timing: 3,
        },
        0x14 => Instruction {
            instr: NOP,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0x34 => Instruction {
            instr: NOP,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0x44 => Instruction {
            instr: NOP,
            addrmode: ZeroPage,
            timing: 3,
        },
        0x54 => Instruction {
            instr: NOP,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0x64 => Instruction {
            instr: NOP,
            addrmode: ZeroPage,
            timing: 3,
        },
        0x74 => Instruction {
            instr: NOP,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0x80 => Instruction {
            instr: NOP,
            addrmode: Immediate,
            timing: 2,
        },
        0x82 => Instruction {
            instr: NOP,
            addrmode: Immediate,
            timing: 2,
        },
        0x89 => Instruction {
            instr: NOP,
            addrmode: Immediate,
            timing: 2,
        },
        0xC2 => Instruction {
            instr: NOP,
            addrmode: Immediate,
            timing: 2,
        },
        0xD4 => Instruction {
            instr: NOP,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0xE2 => Instruction {
            instr: NOP,
            addrmode: Immediate,
            timing: 2,
        },
        0xF4 => Instruction {
            instr: NOP,
            addrmode: ZeroPageX,
            timing: 4,
        },
        0xE7 => Instruction {
            instr: ISC,
            addrmode: ZeroPage,
            timing: 5,
        },
        0xF7 => Instruction {
            instr: ISC,
            addrmode: ZeroPageX,
            timing: 6,
        },
        0xEF => Instruction {
            instr: ISC,
            addrmode: Absolute,
            timing: 6,
        },
        0xFF => Instruction {
            instr: ISC,
            addrmode: AbsoluteX,
            timing: 7,
        },
        0xFB => Instruction {
            instr: ISC,
            addrmode: AbsoluteY,
            timing: 7,
        },
        0xE3 => Instruction {
            instr: ISC,
            addrmode: IndirectX,
            timing: 8,
        },
        0xF3 => Instruction {
            instr: ISC,
            addrmode: IndirectY,
            timing: 8,
        },
        0x02 => Instruction {
            instr: KIL,
            addrmode: Implied,
            timing: 0,
        },
        0x12 => Instruction {
            instr: KIL,
            addrmode: Implied,
            timing: 0,
        },
        0x22 => Instruction {
            instr: KIL,
            addrmode: Implied,
            timing: 0,
        },
        0x32 => Instruction {
            instr: KIL,
            addrmode: Implied,
            timing: 0,
        },
        0x42 => Instruction {
            instr: KIL,
            addrmode: Implied,
            timing: 0,
        },
        0x52 => Instruction {
            instr: KIL,
            addrmode: Implied,
            timing: 0,
        },
        0x62 => Instruction {
            instr: KIL,
            addrmode: Implied,
            timing: 0,
        },
        0x72 => Instruction {
            instr: KIL,
            addrmode: Implied,
            timing: 0,
        },
        0x92 => Instruction {
            instr: KIL,
            addrmode: Implied,
            timing: 0,
        },
        0xB2 => Instruction {
            instr: KIL,
            addrmode: Implied,
            timing: 0,
        },
        0xD2 => Instruction {
            instr: KIL,
            addrmode: Implied,
            timing: 0,
        },
        0xF2 => Instruction {
            instr: KIL,
            addrmode: Implied,
            timing: 0,
        },
        0xBB => Instruction {
            instr: LAR,
            addrmode: AbsoluteY,
            timing: 4,
        },
        0xA7 => Instruction {
            instr: LAX,
            addrmode: ZeroPage,
            timing: 3,
        },
        0xB7 => Instruction {
            instr: LAX,
            addrmode: ZeroPageY,
            timing: 4,
        },
        0xAF => Instruction {
            instr: LAX,
            addrmode: Absolute,
            timing: 4,
        },
        0xBF => Instruction {
            instr: LAX,
            addrmode: AbsoluteY,
            timing: 4,
        },
        0xA3 => Instruction {
            instr: LAX,
            addrmode: IndirectX,
            timing: 6,
        },
        0xB3 => Instruction {
            instr: LAX,
            addrmode: IndirectY,
            timing: 5,
        },
        0x1A => Instruction {
            instr: NOP,
            addrmode: Implied,
            timing: 2,
        },
        0x3A => Instruction {
            instr: NOP,
            addrmode: Implied,
            timing: 2,
        },
        0x5A => Instruction {
            instr: NOP,
            addrmode: Implied,
            timing: 2,
        },
        0x7A => Instruction {
            instr: NOP,
            addrmode: Implied,
            timing: 2,
        },
        0xDA => Instruction {
            instr: NOP,
            addrmode: Implied,
            timing: 2,
        },
        0xFA => Instruction {
            instr: NOP,
            addrmode: Implied,
            timing: 2,
        },
        0x27 => Instruction {
            instr: RLA,
            addrmode: ZeroPage,
            timing: 5,
        },
        0x37 => Instruction {
            instr: RLA,
            addrmode: ZeroPageX,
            timing: 6,
        },
        0x2F => Instruction {
            instr: RLA,
            addrmode: Absolute,
            timing: 6,
        },
        0x3F => Instruction {
            instr: RLA,
            addrmode: AbsoluteX,
            timing: 7,
        },
        0x3B => Instruction {
            instr: RLA,
            addrmode: AbsoluteY,
            timing: 7,
        },
        0x23 => Instruction {
            instr: RLA,
            addrmode: IndirectX,
            timing: 8,
        },
        0x33 => Instruction {
            instr: RLA,
            addrmode: IndirectY,
            timing: 8,
        },
        0x67 => Instruction {
            instr: RRA,
            addrmode: ZeroPage,
            timing: 5,
        },
        0x77 => Instruction {
            instr: RRA,
            addrmode: ZeroPageX,
            timing: 6,
        },
        0x6F => Instruction {
            instr: RRA,
            addrmode: Absolute,
            timing: 6,
        },
        0x7F => Instruction {
            instr: RRA,
            addrmode: AbsoluteX,
            timing: 7,
        },
        0x7B => Instruction {
            instr: RRA,
            addrmode: AbsoluteY,
            timing: 7,
        },
        0x63 => Instruction {
            instr: RRA,
            addrmode: IndirectX,
            timing: 8,
        },
        0x73 => Instruction {
            instr: RRA,
            addrmode: IndirectY,
            timing: 8,
        },
        0xEB => Instruction {
            instr: SBC,
            addrmode: Immediate,
            timing: 2,
        },
        0x07 => Instruction {
            instr: SLO,
            addrmode: ZeroPage,
            timing: 5,
        },
        0x17 => Instruction {
            instr: SLO,
            addrmode: ZeroPageX,
            timing: 6,
        },
        0x0F => Instruction {
            instr: SLO,
            addrmode: Absolute,
            timing: 6,
        },
        0x1F => Instruction {
            instr: SLO,
            addrmode: AbsoluteX,
            timing: 7,
        },
        0x1B => Instruction {
            instr: SLO,
            addrmode: AbsoluteY,
            timing: 7,
        },
        0x03 => Instruction {
            instr: SLO,
            addrmode: IndirectX,
            timing: 8,
        },
        0x13 => Instruction {
            instr: SLO,
            addrmode: IndirectY,
            timing: 8,
        },
        0x47 => Instruction {
            instr: SRE,
            addrmode: ZeroPage,
            timing: 5,
        },
        0x57 => Instruction {
            instr: SRE,
            addrmode: ZeroPageX,
            timing: 6,
        },
        0x4F => Instruction {
            instr: SRE,
            addrmode: Absolute,
            timing: 6,
        },
        0x5F => Instruction {
            instr: SRE,
            addrmode: AbsoluteX,
            timing: 7,
        },
        0x5B => Instruction {
            instr: SRE,
            addrmode: AbsoluteY,
            timing: 7,
        },
        0x43 => Instruction {
            instr: SRE,
            addrmode: IndirectX,
            timing: 8,
        },
        0x53 => Instruction {
            instr: SRE,
            addrmode: IndirectY,
            timing: 8,
        },
        0x9E => Instruction {
            instr: SXA,
            addrmode: AbsoluteY,
            timing: 2,
        },
        0x9C => Instruction {
            instr: SYA,
            addrmode: AbsoluteX,
            timing: 2,
        },
        0x0C => Instruction {
            instr: NOP,
            addrmode: Absolute,
            timing: 4,
        },
        0x1C => Instruction {
            instr: NOP,
            addrmode: AbsoluteX,
            timing: 4,
        },
        0x3C => Instruction {
            instr: NOP,
            addrmode: AbsoluteX,
            timing: 4,
        },
        0x5C => Instruction {
            instr: NOP,
            addrmode: AbsoluteX,
            timing: 4,
        },
        0x7C => Instruction {
            instr: NOP,
            addrmode: AbsoluteX,
            timing: 4,
        },
        0xDC => Instruction {
            instr: NOP,
            addrmode: AbsoluteX,
            timing: 4,
        },
        0xFC => Instruction {
            instr: NOP,
            addrmode: AbsoluteX,
            timing: 4,
        },
        0x8B => Instruction {
            instr: XAA,
            addrmode: Immediate,
            timing: 2,
        },
        0x9B => Instruction {
            instr: XAS,
            addrmode: Immediate,
            timing: 2,
        },
        _ => panic!("Should never happen..."),
    }
}
