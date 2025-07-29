#[cfg(not(target_arch = "wasm32"))]
use minifb::Key;
use std::collections::HashMap;

pub struct Keypad {
    pub keys: [bool; 16],
    #[cfg(not(target_arch = "wasm32"))]
    key_map: HashMap<Key, usize>,
}

impl Keypad {
    pub fn new() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut key_map = HashMap::new();
            key_map.insert(Key::X, 0x0);
            key_map.insert(Key::Key1, 0x1);
            key_map.insert(Key::Key2, 0x2);
            key_map.insert(Key::Key3, 0x3);
            key_map.insert(Key::Q, 0x4);
            key_map.insert(Key::W, 0x5);
            key_map.insert(Key::E, 0x6);
            key_map.insert(Key::A, 0x7);
            key_map.insert(Key::S, 0x8);
            key_map.insert(Key::D, 0x9);
            key_map.insert(Key::Z, 0xA);
            key_map.insert(Key::C, 0xB);
            key_map.insert(Key::Key4, 0xC);
            key_map.insert(Key::R, 0xD);
            key_map.insert(Key::F, 0xE);
            key_map.insert(Key::V, 0xF);

            Keypad {
                keys: [false; 16],
                key_map,
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            Keypad {
                keys: [false; 16],
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn update(&mut self, window: &minifb::Window) {
        self.keys = [false; 16];
        for (key, &chip8_key) in &self.key_map {
            if window.is_key_down(*key) {
                self.keys[chip8_key] = true;
            }
        }
    }

    /// Set or clear a key state manually (used for WASM & JS)
    pub fn set_key(&mut self, key: usize, pressed: bool) {
        if let Some(slot) = self.keys.get_mut(key) {
            *slot = pressed;
        }
    }

    pub fn is_key_down(&self, key: usize) -> bool {
        self.keys.get(key).copied().unwrap_or(false)
    }

    pub fn wait_for_key(&self) -> Option<usize> {
        self.keys.iter().position(|&pressed| pressed)
    }
}