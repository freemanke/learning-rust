// Rust 通过 struct 和 impl 块来封装数据和行为

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn perimeter(&self) -> i32 {
        self.width * 2 + self.height * 2
    }
}

mod tests {
    

    #[test]
    fn test_area() {
        let rectangle = Rectangle {
            width: 100,
            height: 100,
        };
        assert_eq!(rectangle.area(), 10000)
    }

    #[test]
    fn test_perimeter() {
        let rectangle = Rectangle {
            width: 20,
            height: 80,
        };
        assert_eq!(rectangle.perimeter(), 200);
    }
}