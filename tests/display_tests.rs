use ditto::chip8::{Display};

#[test]
fn test_clear_display() {
    let mut display = Display::new();
    display.buffer[0] = 0xFFFFFFFF;
    display.clear();
    assert!(display.buffer.iter().all(|&p| p == 0));
}

#[test]
fn test_draw_sprite_collision() {
    let mut display = Display::new();
    let sprite = [0xFF]; 
    let collision1 = display.draw_sprite(0, 0, &sprite);
    assert!(!collision1);
    let collision2 = display.draw_sprite(0, 0, &sprite);
    assert!(collision2);
}