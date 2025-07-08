pub mod cpu;
pub mod display;
pub mod keypad;
pub mod memory;
pub mod timers;
pub mod utils;

use cpu::Cpu;
use display::Display;
use keypad::Keypad;
use memory::Memory;
use timers::Timers;

pub struct Chip8 {
    pub cpu: Cpu,
    pub display: Display,
    pub keypad: Keypad,
    pub memory: Memory,
    pub timers: Timers,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut memory = Memory::new();
        memory.load_fontset();
        Self {
            cpu: Cpu::new(),
            display: Display::new(),
            keypad: Keypad::new(),
            memory,
            timers: Timers::new(),
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory.load_rom(rom);
    }

    pub fn cycle(&mut self) {
        let opcode = self.memory.fetch_opcode(self.cpu.pc);
        self.cpu.execute_opcode(
            opcode,
            &mut self.memory,
            &mut self.display,
            &mut self.keypad,
            &mut self.timers,
        );
    }
}