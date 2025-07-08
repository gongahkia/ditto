pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Display {
    pub buffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Self {
            buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [0; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let mut collision = false;
        for (row, byte) in sprite.iter().enumerate() {
            for bit in 0..8 {
                let px = ((x as usize) + bit) % SCREEN_WIDTH;
                let py = ((y as usize) + row) % SCREEN_HEIGHT;
                let idx = py * SCREEN_WIDTH + px;
                let pixel = (byte >> (7 - bit)) & 1;
                if pixel == 1 {
                    if self.buffer[idx] == 0xFFFFFFFF {
                        collision = true;
                    }
                    self.buffer[idx] ^= 0xFFFFFFFF;
                }
            }
        }
        collision
    }

    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }
}