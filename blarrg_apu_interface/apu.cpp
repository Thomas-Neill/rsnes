#include "Nes_Apu.h"
#include "Sound_Queue.h"
#include "apu_snapshot.h"
#include "common.hpp"
#include <iostream>
Nes_Apu apu;
Blip_Buffer buf;
Sound_Queue* soundQueue;

const int OUT_SIZE = 4096;
blip_sample_t outBuf[OUT_SIZE];

extern "C" {

void apuinit(int (*callback)( void* user_data, cpu_addr_t ),void* user_data) {
    buf.sample_rate(96000);
    buf.clock_rate(1789773);

    apu.output(&buf);
    apu.dmc_reader(callback,user_data);

    soundQueue = new Sound_Queue;
    soundQueue->init(96000);
}

void apureset()
{
    apu.reset();
    buf.clear();
}

void apuwrite(int elapsed, u16 addr, u8 v)
{
    apu.write_register(elapsed, addr, v);
}

u8 apuread(int elapsed) {
  return apu.read_status(elapsed);
}

void apurun_frame(int elapsed)
{
    apu.end_frame(elapsed);
    buf.end_frame(elapsed);

    if (buf.samples_avail() >= OUT_SIZE)
        soundQueue->write(outBuf, buf.read_samples(outBuf, OUT_SIZE));
}

apu_snapshot_t* aputake_snapshot() {
    static apu_snapshot_t snapshot;
    apu.save_snapshot(&snapshot);
    return &snapshot;
}

void apuget_snapshot(apu_snapshot_t* reset) {
    apu.load_snapshot(*reset);
}

}
