use wasm_bindgen::prelude::*;
use crate::chip8::{Chip8, display::{SCREEN_WIDTH, SCREEN_HEIGHT}};

#[wasm_bindgen]
pub struct WasmChip8 {
    chip8: Chip8,
}

#[wasm_bindgen]
impl WasmChip8 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmChip8 {
        // Optional: better panic messages in browser console
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        WasmChip8 {
            chip8: Chip8::new(),
        }
    }

    /// Load a ROM into the emulator
    #[wasm_bindgen]
    pub fn load_rom(&mut self, rom: &[u8]) {
        self.chip8.load_rom(rom);
    }

    /// Perform one CPU cycle (fetch + execute opcode)
    #[wasm_bindgen]
    pub fn cycle(&mut self) {
        self.chip8.cycle();
    }

    /// Get a pointer to the display buffer (u32 pixels)
    #[wasm_bindgen(getter)]
    pub fn display_buffer_ptr(&self) -> *const u32 {
        self.chip8.display.buffer.as_ptr()
    }

    /// Width of the display in pixels
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> usize {
        SCREEN_WIDTH
    }

    /// Height of the display in pixels
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> usize {
        SCREEN_HEIGHT
    }

    /// Set the pressed state for a given key (0x0..0xF)
    #[wasm_bindgen]
    pub fn set_key(&mut self, key: u8, pressed: bool) {
        if let Some(k) = self.chip8.keypad.keys.get_mut(key as usize) {
            *k = pressed;
        }
    }

    /// Reset emulator
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.chip8 = Chip8::new();
    }
}