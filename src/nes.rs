use cpu::CPU;
use cpu::Interrupt;
use ppu::PPU;
use ppu::PPUStatus;
use ines::INES;
use mappers::get_mapper;
use mappers::Mapper;
use apu::*;
use bincode::*;

use std::time::Instant;
use std::time::Duration;

use std::rc::Rc;
use std::cell::RefCell;
use std::fs::*;
use std::fs::create_dir;
use std::io::Write;
use std::io::Read;
use std::path::Path;

use sdl2;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureAccess;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum::RGB24;
use sdl2::keyboard::Scancode;
use sdl2::audio::*;

pub struct NES<'a> {
    canvas: Canvas<Window>,
    texture: Texture<'a>,
    pump: EventPump,
    cpu: CPU,
    mapper: Rc<RefCell<Mapper>>,
    ppu: Rc<RefCell<PPU>>,
    done: bool,
    last_draw: Instant,
    savefile: Option<String>,
    savestates: [Vec<u8>;9],
    filename: String
}
#[derive(Serialize,Deserialize)]
struct NES_State {
    cpu: Vec<u8>,
    apu: Vec<u8>,
    ppu: Vec<u8>,
    mapper: Vec<u8>
}

extern fn apu_contents(cpu:*mut CPU,c: u32) -> i32 {
    unsafe {
        (*cpu).contents(c as u16) as i32
    }
}


impl<'a> NES<'a> {
    pub fn start(filename: String,savefile: Option<String>) {
        let ctx = sdl2::init().unwrap();
        let video = ctx.video().unwrap();
        let window = video.window("RSnes",512,480).position_centered().build().unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.clear();
        let creator = canvas.texture_creator();
        let texture = creator.create_texture(RGB24,TextureAccess::Streaming,256,240).unwrap();
        let pump = ctx.event_pump().unwrap();

        let mut nes = NES::new(filename,savefile,texture,pump,canvas);

        nes.go();
    }
    pub fn new(filename: String,savefile: Option<String>,texture:Texture<'a>,pump:EventPump,canvas:Canvas<Window>) -> NES<'a> {
        let ines = INES::new(filename.clone(),savefile.clone());
        let mapper = get_mapper(ines);
        let ppu = Rc::new(RefCell::new(PPU::new(mapper.clone())));
        let cpu = CPU::new(mapper.clone(),ppu.clone());
        NES {
            canvas:canvas,
            texture:texture,
            pump: pump,
            cpu: cpu,
            mapper: mapper,
            ppu: ppu,
            done: false,
            last_draw: Instant::now(),
            savefile: savefile,
            savestates: [vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![],vec![]],
            filename: Path::new(&filename).file_stem().unwrap().to_str().unwrap().to_string()
        }
    }
    pub fn go(&mut self) {
        if Path::new("savestates").join(self.filename.clone()).exists() {
            for i in 0..9 {
                self.savestates[i].reserve(10000);
                File::open(Path::new("savestates").join(self.filename.clone()).join(format!("save{}",i))).unwrap().read_to_end(&mut self.savestates[i]).unwrap();
            }
        }
        apu_init(apu_contents,&mut self.cpu as *mut CPU);
        while !self.done {
            self.cpu.cycle();
            if self.mapper.borrow_mut().interrupt() {
                self.cpu.interrupt = Interrupt::IRQ;
            }
            self.ppu_cycle();
            self.ppu_cycle();
            self.ppu_cycle();
        }
        match self.savefile.clone() {
            Some(filename) => {
                let save = self.mapper.borrow_mut().get_savedata();
                File::create(filename).unwrap().write(&save).unwrap();
            }
            None => ()
        }
        if !Path::new("savestates").join(self.filename.clone()).exists() {
            create_dir(Path::new("savestates").join(self.filename.clone())).unwrap();
        }
        for i in 0..9 {
            File::create(Path::new("savestates").join(self.filename.clone()).join(format!("save{}",i))).unwrap().write(&self.savestates[i]).unwrap();
        }
    }
    fn ppu_cycle(&mut self) {
        let mut ppu = self.ppu.borrow_mut();
        let status = ppu.cycle();
        if status == PPUStatus::VBlank {
            let dat = ppu.serialize();
            ppu.deserialize(&dat);
            apu_run_frame(self.cpu.elapsed);
            self.cpu.frame();
            self.canvas.clear();
            self.texture.update(None,&ppu.screen,256*3).unwrap();
            self.canvas.copy(&self.texture,None,None).unwrap();
            self.canvas.present();
            if ppu.generate_nmi {
                self.cpu.interrupt = Interrupt::NMI;
            }
            //poll events
            for event in self.pump.poll_iter() {
                match event  {
                    Event::Quit {..} => {
                        self.done = true;
                    }
                    Event::KeyDown {scancode:Some(Scancode::Num1),..} => {
                        self.savestates[0] = serialize_nes(&self.cpu,&ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::F1),..} => {
                        deserialize_nes(&self.savestates[0],&mut self.cpu,&mut ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::Num2),..} => {
                        self.savestates[1] = serialize_nes(&self.cpu,&ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::F2),..} => {
                        deserialize_nes(&self.savestates[1],&mut self.cpu,&mut ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::Num3),..} => {
                        self.savestates[2] = serialize_nes(&self.cpu,&ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::F3),..} => {
                        deserialize_nes(&self.savestates[2],&mut self.cpu,&mut ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::Num4),..} => {
                        self.savestates[3] = serialize_nes(&self.cpu,&ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::F4),..} => {
                        deserialize_nes(&self.savestates[3],&mut self.cpu,&mut ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::Num5),..} => {
                        self.savestates[4] = serialize_nes(&self.cpu,&ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::F5),..} => {
                        deserialize_nes(&self.savestates[4],&mut self.cpu,&mut ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::Num6),..} => {
                        self.savestates[5] = serialize_nes(&self.cpu,&ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::F6),..} => {
                        deserialize_nes(&self.savestates[5],&mut self.cpu,&mut ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::Num7),..} => {
                        self.savestates[6] = serialize_nes(&self.cpu,&ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::F7),..} => {
                        deserialize_nes(&self.savestates[6],&mut self.cpu,&mut ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::Num8),..} => {
                        self.savestates[7] = serialize_nes(&self.cpu,&ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::F8),..} => {
                        deserialize_nes(&self.savestates[7],&mut self.cpu,&mut ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::Num8),..} => {
                        self.savestates[8] = serialize_nes(&self.cpu,&ppu,self.mapper.clone());
                    }
                    Event::KeyDown {scancode:Some(Scancode::F8),..} => {
                        deserialize_nes(&self.savestates[8],&mut self.cpu,&mut ppu,self.mapper.clone());
                    }
                    _ => ()
                }
            }
            let st = self.pump.keyboard_state();
            self.cpu.inputs[0] = st.is_scancode_pressed(Scancode::Z); //A
            self.cpu.inputs[1] = st.is_scancode_pressed(Scancode::X); //B
            self.cpu.inputs[2] = st.is_scancode_pressed(Scancode::RShift); //Select
            self.cpu.inputs[3] = st.is_scancode_pressed(Scancode::Return); //Start
            self.cpu.inputs[4] = st.is_scancode_pressed(Scancode::Up); //Up
            self.cpu.inputs[5] = st.is_scancode_pressed(Scancode::Down); //Down
            self.cpu.inputs[6] = st.is_scancode_pressed(Scancode::Left); //Left
            self.cpu.inputs[7] = st.is_scancode_pressed(Scancode::Right); //Right

            self.cpu.inputs2[0] = st.is_scancode_pressed(Scancode::A); //A
            self.cpu.inputs2[1] = st.is_scancode_pressed(Scancode::S); //B
            self.cpu.inputs2[2] = st.is_scancode_pressed(Scancode::F); //Select
            self.cpu.inputs2[3] = st.is_scancode_pressed(Scancode::D); //Start
            self.cpu.inputs2[4] = st.is_scancode_pressed(Scancode::I); //Up
            self.cpu.inputs2[5] = st.is_scancode_pressed(Scancode::K); //Down
            self.cpu.inputs2[6] = st.is_scancode_pressed(Scancode::J); //Left
            self.cpu.inputs2[7] = st.is_scancode_pressed(Scancode::L); //Right
            while self.last_draw.elapsed() < Duration::from_millis(17) {}
            self.last_draw = Instant::now()
        }
        if status == PPUStatus::HBlank {
            self.mapper.borrow_mut().scanline();
        }
    }
}

fn serialize_nes(cpu:&CPU,ppu:&PPU,mapper:Rc<RefCell<Mapper>>) -> Vec<u8> {
    let serial = NES_State {
        cpu: cpu.serialize(),
        apu: apu_take_snapshot(),
        ppu: ppu.serialize(),
        mapper: mapper.borrow().serialize()
    };
    serialize(&serial).unwrap()
}

fn deserialize_nes(data:&[u8],cpu:&mut CPU,ppu:&mut PPU,mapper:Rc<RefCell<Mapper>>) {
    if data.len() != 0 {
        let serial: NES_State = deserialize(data).unwrap();
        cpu.deserialize(&serial.cpu);
        apu_get_snapshot(&serial.apu);
        ppu.deserialize(&serial.ppu);
        mapper.borrow_mut().deserialize(&serial.mapper);
    }
}
