#[link(name="apu")]
use cpu::CPU;
use std::slice;
use std::io::Write;
extern {
    fn apuinit(read_mem: extern fn(cpu:*mut CPU,c: u32) -> i32,cpu:*mut CPU);
    fn apureset();
    fn apuwrite(elapsed:i32,addr:u16,value:u8);
    fn apuread(elapsed:i32) -> u8;
    fn apurun_frame(elapsed:i32);
    fn aputake_snapshot() -> *const u8;
    fn apuget_snapshot(snapshot: *const u8);
}

pub fn apu_init(read_mem: extern fn(cpu:*mut CPU,c: u32) -> i32,cpu:*mut CPU) {
    unsafe {
        apuinit(read_mem,cpu);
    }
}
pub fn apu_reset() {
    unsafe {
        apureset();
    }
}
pub fn apu_write(elapsed:i32,addr:u16,value:u8) {
    unsafe {
        apuwrite(elapsed,addr,value);
    }
}
pub fn apu_read(elapsed:i32) -> u8 {

    unsafe {
        apuread(elapsed)
    }
}
pub fn apu_run_frame(elapsed:i32) {
    unsafe {
        apurun_frame(elapsed);
    }
}

pub fn apu_take_snapshot() -> Vec<u8> {
    unsafe {
        let ptr = aputake_snapshot();
        let mut v = vec![];
        v.write(slice::from_raw_parts(ptr,72)).unwrap();
        v
    }
}

pub fn apu_get_snapshot(xs:&[u8]) {
    unsafe {
        apuget_snapshot(xs.as_ptr());
    }
}
