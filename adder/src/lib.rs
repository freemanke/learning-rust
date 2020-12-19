#[cfg(test)]
mod tests {

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_small(){
        let small = Rectangle { width: 2, height: 3 };
        let larger = Rectangle { width: 50, height: 23 };
        assert!(larger.can_hold(&small))
    }

}
