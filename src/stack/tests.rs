use super::*;

#[test]
fn test_stack() {
    let mut stack = Stack::new();
    assert_eq!(stack.pop(), None);
    stack.push(3).unwrap();
    stack.push(1).unwrap();
    stack.push(2).unwrap();
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), None);
    assert_eq!(stack.pop(), None);
}
