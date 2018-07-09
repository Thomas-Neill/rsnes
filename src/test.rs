mod cpu {
    use cpu::*;
    use ines::INES;
    use ppu::PPU;
    use mappers::get_mapper;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;
    use std::rc::Rc;
    use std::cell::RefCell;
    impl CPU {
        pub fn test() {
            println!("Testing CPU with nestest...");
            let ines = INES::new("roms/nestest.nes".to_string(),None);
            let handle = File::open("test/goodlog").unwrap();
            let mapper = get_mapper(ines);
            let ppu = Rc::new(RefCell::new(PPU::new(mapper.clone())));
            let mut cpu = CPU::new(mapper,ppu);
            cpu.PC = 0xC000;
            for line in BufReader::new(handle).lines() {
                let n = u16::from_str_radix(&line.unwrap(),16).unwrap();
                if cpu.PC != n {
                    panic!("PC should have been: 0x{:X}, but was 0x{:X}",n,cpu.PC);
                }
                cpu.debug_print();
                cpu.instr();
            }
            println!("test passed.");
        }
    }
}
