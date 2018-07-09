extern crate sdl2;
extern crate bincode;
#[macro_use]
extern crate serde_derive;
extern crate serde;
mod instruction;
mod cpu;
mod apu;
mod ppu;
mod ines;
mod mappers;
mod nes;
mod test;

use cpu::*;
use nes::NES;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = args[1].to_string();
    let save = if args.len() <3 {None} else {Some(args[2].to_string())};
    if file == "TEST" {
        CPU::test();
    } else {
        NES::start(file,save);
    }
}
