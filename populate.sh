#!/bin/bash

mkdir -p ./src/roms
cd ./src/roms
curl -L -o chip8-roms.zip https://github.com/kripod/chip8-roms/archive/refs/heads/master.zip
unzip -o chip8-roms.zip
mv chip8-roms-master/*.ch8 ./
rm -rf chip8-roms-master chip8-roms.zip