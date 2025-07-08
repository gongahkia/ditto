use ditto::chip8::{Chip8, memory::ROM_START};

#[test]
fn test_ld_and_add() {
    let mut chip8 = Chip8::new();
    let rom = vec![0x60, 0x0A, 0x61, 0x05, 0x80, 0x14];
    chip8.load_rom(&rom);
    chip8.cycle(); 
    chip8.cycle(); 
    chip8.cycle(); 
    assert_eq!(chip8.cpu.v[0], 0x0A + 0x05);
}

#[test]
fn test_jump_and_call() {
    let mut chip8 = Chip8::new();
    let rom = vec![0x13, 0x00, 0x24, 0x00];
    chip8.load_rom(&rom);
    chip8.cycle(); 
    assert_eq!(chip8.cpu.pc, 0x300);
    chip8.cycle(); 
    assert_eq!(chip8.cpu.pc, 0x400);
    assert_eq!(chip8.cpu.sp, 1);
    assert_eq!(chip8.cpu.stack[0], 0x300);
}

#[test]
fn test_skip_instructions() {
    let mut chip8 = Chip8::new();
    let rom = vec![0x30, 0x0A, 0x41, 0x05];
    chip8.load_rom(&rom);
    chip8.cpu.v[0] = 0x0A;
    chip8.cpu.v[1] = 0x06;
    chip8.cycle(); 
    assert_eq!(chip8.cpu.pc, 0x204);
    chip8.cycle(); 
    assert_eq!(chip8.cpu.pc, 0x208);
}