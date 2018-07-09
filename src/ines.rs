use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::vec::Vec;
use std::path::Path;
pub struct INES {
    pub prgrom_size: u8,
    pub chrrom_size: u8,
    pub vertical_mirroring: bool,
    pub persistent_memory: bool,
    pub ignore_mirroring: bool,
    pub mapper: u8,
    pub prgrom: Vec<u8>,
    pub chrrom: Vec<u8>,
    pub savedata: [u8; 0x2000]
}
impl INES {
    pub fn new(filename: String,savefile: Option<String>) -> INES {
        let mut handle = File::open(filename).unwrap();
        let mut header = [0;16];
        let mut result = INES {
            prgrom_size: 0,
            chrrom_size: 0,
            vertical_mirroring: false,
            persistent_memory: false,
            ignore_mirroring: false,
            mapper: 0,
            prgrom: vec![],
            chrrom: vec![],
            savedata: [0;0x2000]
        };
        handle.read_exact(&mut header).unwrap();
        result.prgrom_size = header[4];
        result.chrrom_size = header[5];
        result.vertical_mirroring = header[6] & 0b1 != 0;
        result.persistent_memory = header[6] & 0b10 != 0;
        result.ignore_mirroring = header[6] & 0b100 != 0;
        result.mapper = header[7] & 0b11110000 | (header[6] >> 4) & 0b1111;
        result.prgrom.reserve(0x4000 * result.prgrom_size as usize);
        unsafe {
            result.prgrom.set_len(0x4000 * result.prgrom_size as usize);
        }
        handle.read(result.prgrom.as_mut_slice()).unwrap();
        if result.chrrom_size != 0 {
            result.chrrom.reserve(0x2000 * result.chrrom_size as usize);
            unsafe {
                result.chrrom.set_len(0x2000 * result.chrrom_size as usize);
            }
            handle.read(result.chrrom.as_mut_slice()).unwrap();
        }
        if result.chrrom_size == 0 {
            result.chrrom = vec![0;0x2000*32];
        }
        if savefile.is_some() && result.persistent_memory {
            let path = savefile.unwrap();
            if !Path::new(&path).exists() {
                let mut handl = File::create(path.clone()).unwrap();
                handl.write(&result.savedata).unwrap();
            }
            let mut handle = File::open(path).unwrap();
            handle.read(&mut result.savedata).unwrap();
        }
        result
    }
}
