// Rust 没有传统的类继承，但可以通过组合和特征来实现类似继承的效果
// Rust 使用特征对象来实现动态分派（类似于传统的多态）

trait Animal {
    fn eat(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn eat(&self) -> String {
        format!("{} dog eat!", self.name)
    }
}

impl Animal for Cat {
    fn eat(&self) -> String {
        format!("{} cat eat!", self.name)
    }
}

mod tests {
    use crate::oop::animal::{Animal, Cat, Dog};

    #[test]
    fn test_eat() {
        let dog = Dog { name: String::from("Dog") };
        let cat = Cat { name: String::from("Cat") };
        assert_eq!(animal_eat(&dog), "Dog dog eat!");
        assert_eq!(animal_eat(&cat), "Cat cat eat!");
    }

    fn animal_eat(animal: &dyn Animal) -> String {
        animal.eat()
    }
}