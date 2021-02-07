use super::bcd;

#[test]
fn test_bcd() {
    assert_eq!(bcd(0), [0, 0, 0]);
    assert_eq!(bcd(5), [0, 0, 5]);
    assert_eq!(bcd(12), [0, 1, 2]);
    assert_eq!(bcd(123), [1, 2, 3]);
}
