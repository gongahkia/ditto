use ditto::chip8::{Chip8, memory::ROM_START};

#[test]
fn test_rom_loading() {
    let mut chip8 = Chip8::new();
    let rom = vec![0x60, 0x0A, 0x61, 0x05]; 
    chip8.load_rom(&rom);
    assert_eq!(chip8.memory.ram[ROM_START], 0x60);
    assert_eq!(chip8.memory.ram[ROM_START + 1], 0x0A);
    assert_eq!(chip8.memory.ram[ROM_START + 2], 0x61);
    assert_eq!(chip8.memory.ram[ROM_START + 3], 0x05);
}

#[test]
fn test_opcode_execution() {
    let mut chip8 = Chip8::new();
    let rom = vec![0x60, 0x0A, 0x61, 0x05, 0x80, 0x14];
    chip8.load_rom(&rom);
    chip8.cycle(); 
    chip8.cycle(); 
    chip8.cycle(); 
    assert_eq!(chip8.cpu.v[0], 0x0A + 0x05);
}
