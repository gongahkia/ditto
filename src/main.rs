mod chip8;

use chip8::{Chip8, display::SCREEN_WIDTH, display::SCREEN_HEIGHT};
use minifb::{Key, Window, WindowOptions};
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_rom>", args[0]);
        std::process::exit(1);
    }
    let rom_data = fs::read(&args[1]).expect("Failed to read ROM file");
    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom_data);
    let mut window = Window::new(
        "CHIP-8 Emulator - Ditto",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X10,
            ..WindowOptions::default()
        },
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    while window.is_open() && !window.is_key_down(Key::Escape) {
        chip8.cycle();
        let buffer = chip8.display.get_buffer();
        window
            .update_with_buffer(buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
        chip8.keypad.update(&window);
        chip8.timers.tick();
    }
}