pub mod chip8;

#[cfg(not(target_arch = "wasm32"))]
extern crate minifb;