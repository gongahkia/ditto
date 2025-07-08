use ditto::chip8::Timers;

#[test]
fn test_timers_tick() {
    let mut timers = Timers::new();
    timers.delay = 2;
    timers.sound = 1;
    timers.tick();
    assert_eq!(timers.delay, 1);
    assert_eq!(timers.sound, 0);
    timers.tick();
    assert_eq!(timers.delay, 0);
    assert_eq!(timers.sound, 0);
}