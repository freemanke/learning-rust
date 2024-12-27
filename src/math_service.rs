pub fn add(first: i8, second: i8) -> i8 {
    first + second
}

pub fn divide(first: i8, second: i8) -> i8 {
    first / second
}

mod tests {
    use super::*;

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
}