use ditto::chip8::Memory;

#[test]
fn test_memory_read_write() {
    let mut memory = Memory::new();
    memory.write(0x300, 0xAB);
    assert_eq!(memory.read(0x300), 0xAB);
}

#[test]
fn test_load_fontset_and_rom() {
    let mut memory = Memory::new();
    memory.load_fontset();
    assert_eq!(memory.ram[0x50], 0xF0); 
    let rom = vec![0x12, 0x34, 0x56];
    memory.load_rom(&rom);
    assert_eq!(memory.ram[0x200], 0x12);
    assert_eq!(memory.ram[0x201], 0x34);
    assert_eq!(memory.ram[0x202], 0x56);
}