use super::*;

#[test]
fn test_display() {
    let mut display = Display::new();
    let f = display.draw((0, 0), &crate::memory::FONTS_SET[..5]);
    assert_eq!(f, false);
    let expect = [
        true, true, true, true, false, false, false, false, // 0xF0
        true, false, false, true, false, false, false, false, // 0x90
        true, false, false, true, false, false, false, false, // 0x90
        true, false, false, true, false, false, false, false, // 0x90
        true, true, true, true, false, false, false, false, // 0xF0
    ];
    for y in 0..5 {
        for x in 0..8 {
            assert_eq!(display.0[x + y * usize::from(WIDTH)], expect[x + y * 8]);
        }
    }

    let f = display.draw((0, 0), &crate::memory::FONTS_SET[..5]);
    assert_eq!(f, true);
    assert!(display.0.iter().all(|&o| o == false));
}
