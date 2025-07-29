use crate::chip8::{display::Display, keypad::Keypad, memory::Memory, timers::Timers};
use rand::Rng;

pub struct Cpu {
    pub v: [u8; 16],  
    pub i: u16,   
    pub pc: u16,   
    pub sp: u8,  
    pub stack: [u16; 16],
    pub waiting_for_key: Option<u8>, 
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: 0x200, 
            sp: 0,
            stack: [0; 16],
            waiting_for_key: None,
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
        if let Some(reg) = self.waiting_for_key {
            if let Some(key) = keypad.wait_for_key() {
                self.v[reg as usize] = key as u8;
                self.waiting_for_key = None;
                self.pc += 2;
            }
            return;
        }

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                0x00E0 => {
                    display.clear();
                    self.pc += 2;
                }
                0x00EE => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                    self.pc += 2;
                }
                _ => {
                    self.pc += 2;
                }
            },
            0x1000 => {
                self.pc = opcode & 0x0FFF;
            }
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = opcode & 0x0FFF;
            }
            0x3000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.pc += if self.v[x] == nn { 4 } else { 2 };
            }
            0x4000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.pc += if self.v[x] != nn { 4 } else { 2 };
            }
            0x5000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if (opcode & 0x000F) == 0 {
                    self.pc += if self.v[x] == self.v[y] { 4 } else { 2 };
                } else {
                    self.pc += 2;
                }
            }
            0x6000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.v[x] = nn;
                self.pc += 2;
            }
            0x7000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.v[x] = self.v[x].wrapping_add(nn);
                self.pc += 2;
            }
            0x8000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                match opcode & 0x000F {
                    0x0 => { 
                        self.v[x] = self.v[y];
                    }
                    0x1 => { 
                        self.v[x] |= self.v[y];
                    }
                    0x2 => { 
                        self.v[x] &= self.v[y];
                    }
                    0x3 => { 
                        self.v[x] ^= self.v[y];
                    }
                    0x4 => { 
                        let (res, carry) = self.v[x].overflowing_add(self.v[y]);
                        self.v[0xF] = if carry { 1 } else { 0 };
                        self.v[x] = res;
                    }
                    0x5 => { 
                        self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
                        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                    }
                    0x6 => { 
                        self.v[0xF] = self.v[x] & 0x1;
                        self.v[x] >>= 1;
                    }
                    0x7 => { 
                        self.v[0xF] = if self.v[y] > self.v[x] { 1 } else { 0 };
                        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                    }
                    0xE => { 
                        self.v[0xF] = (self.v[x] & 0x80) >> 7;
                        self.v[x] <<= 1;
                    }
                    _ => {}
                }
                self.pc += 2;
            }
            0x9000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if (opcode & 0x000F) == 0 {
                    self.pc += if self.v[x] != self.v[y] { 4 } else { 2 };
                } else {
                    self.pc += 2;
                }
            }
            0xA000 => {
                self.i = opcode & 0x0FFF;
                self.pc += 2;
            }
            0xB000 => {
                self.pc = (opcode & 0x0FFF) + self.v[0] as u16;
            }
            0xC000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                let mut rng = rand::thread_rng();
                self.v[x] = rng.r#gen::<u8>() & nn;
                self.pc += 2;
            }
            0xD000 => {
                let x = self.v[((opcode & 0x0F00) >> 8) as usize];
                let y = self.v[((opcode & 0x00F0) >> 4) as usize];
                let n = (opcode & 0x000F) as usize;
                let mut sprite = Vec::new();
                for i in 0..n {
                    sprite.push(memory.read(self.i + i as u16));
                }
                let collision = display.draw_sprite(x, y, &sprite);
                self.v[0xF] = if collision { 1 } else { 0 };
                self.pc += 2;
            }
            0xE000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                match opcode & 0x00FF {
                    0x9E => {
                        self.pc += if keypad.is_key_down(self.v[x] as usize) { 4 } else { 2 };
                    }
                    0xA1 => {
                        self.pc += if !keypad.is_key_down(self.v[x] as usize) { 4 } else { 2 };
                    }
                    _ => {
                        self.pc += 2;
                    }
                }
            }
            0xF000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                match opcode & 0x00FF {
                    0x07 => {
                        self.v[x] = timers.delay;
                        self.pc += 2;
                    }
                    0x0A => {
                        self.waiting_for_key = Some(x as u8);
                    }
                    0x15 => {
                        timers.delay = self.v[x];
                        self.pc += 2;
                    }
                    0x18 => {
                        timers.sound = self.v[x];
                        self.pc += 2;
                    }
                    0x1E => {
                        self.i = self.i.wrapping_add(self.v[x] as u16);
                        self.pc += 2;
                    }
                    0x29 => {
                        self.i = 0x50 + (self.v[x] as u16) * 5;
                        self.pc += 2;
                    }
                    0x33 => {
                        let vx = self.v[x];
                        memory.write(self.i, vx / 100);
                        memory.write(self.i + 1, (vx % 100) / 10);
                        memory.write(self.i + 2, vx % 10);
                        self.pc += 2;
                    }
                    0x55 => {
                        for offset in 0..=x {
                            memory.write(self.i + offset as u16, self.v[offset]);
                        }
                        self.pc += 2;
                    }
                    0x65 => {
                        for offset in 0..=x {
                            self.v[offset] = memory.read(self.i + offset as u16);
                        }
                        self.pc += 2;
                    }
                    _ => {
                        self.pc += 2;
                    }
                }
            }
            _ => {
                self.pc += 2;
            }
        }
    }
}