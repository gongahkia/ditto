pub struct Timers {
    pub delay: u8,
    pub sound: u8,
}

impl Timers {
    pub fn new() -> Self {
        Self { delay: 0, sound: 0 }
    }

    pub fn tick(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }
        if self.sound > 0 {
            self.sound -= 1;
        }
    }
}