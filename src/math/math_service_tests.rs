use crate::math_service::*;

#[test]
fn add_test() {
    let result = add(2, 2);
    assert_eq!(result, 4);
    assert_ne!(result, 5);
    assert!("freemanke".contains('k'));
}

#[test]
#[should_panic]
fn divide_test() {
    divide(2, 0);
}
