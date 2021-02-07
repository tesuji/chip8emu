use super::*;

#[test]
fn test_iter_bit() {
    let mut n = BitIter::new(0b1010_0011);
    assert_eq!(n.next(), Some(true));
    assert_eq!(n.next(), Some(false));
    assert_eq!(n.next(), Some(true));
    assert_eq!(n.next(), Some(false));

    assert_eq!(n.next(), Some(false));
    assert_eq!(n.next(), Some(false));
    assert_eq!(n.next(), Some(true));
    assert_eq!(n.next(), Some(true));

    assert_eq!(n.next(), None);
    assert_eq!(n.next(), None);
}

#[test]
fn test_4_nibbles() {
    let abcd = to_4_be_nibles(0x1234);
    assert_eq!(abcd, [1, 2, 3, 4]);
}
