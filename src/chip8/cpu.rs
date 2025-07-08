use crate::chip8::{display::Display, keypad::Keypad, memory::Memory, timers::Timers};

pub struct Cpu {
    pub v: [u8; 16],    // General purpose registers V0..VF
    pub i: u16,         // Index register
    pub pc: u16,        // Program counter
    pub sp: u8,         // Stack pointer
    pub stack: [u16; 16],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: 0x200, // Programs start at 0x200
            sp: 0,
            stack: [0; 16],
        }
    }

    pub fn execute_opcode(
        &mut self,
        opcode: u16,
        memory: &mut Memory,
        display: &mut Display,
        keypad: &mut Keypad,
        timers: &mut Timers,
    ) {
        // Example: decode and execute a few opcodes
        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                0x00E0 => display.clear(), // CLS
                0x00EE => { // RET
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
                _ => {}
            },
            0x1000 => { // JP addr
                self.pc = opcode & 0x0FFF;
                return; // Skip pc increment
            }
            0x2000 => { // CALL addr
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = opcode & 0x0FFF;
                return;
            }
            // ... Implement all CHIP-8 opcodes here ...
            _ => {}
        }
        self.pc += 2;
    }
}