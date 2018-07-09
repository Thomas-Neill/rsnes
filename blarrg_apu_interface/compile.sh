#!/bin/bash
g++ apu.cpp -Ilib/include -Isrc/include lib/*.cpp -lSDL2 -c
ar rc libapu.a *.o
ranlib libapu.a
rm ../libraries/libapu.a
mv ./libapu.a ../libraries/libapu.a
