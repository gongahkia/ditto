use ditto::chip8::Keypad;

#[test]
fn test_keypad_initial_state() {
    let keypad = Keypad::new();
    assert!(keypad.keys.iter().all(|&k| !k));
}

#[test]
fn test_keypad_is_key_down_and_wait_for_key() {
    let mut keypad = Keypad::new();
    keypad.keys[5] = true;
    assert!(keypad.is_key_down(5));
    assert_eq!(keypad.wait_for_key(), Some(5));
    keypad.keys[5] = false;
    assert!(!keypad.is_key_down(5));
    assert_eq!(keypad.wait_for_key(), None);
}