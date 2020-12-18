use std::io;

fn main() {
    println!("Hello, world!");
    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("you don't input a name");
    println!("your name {}", name)
}
