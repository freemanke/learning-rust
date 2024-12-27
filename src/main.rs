pub mod math;
use math::math_service;

fn main() {
    println!("Learning Rust!");
    let result = math_service::add(1, 2);
    println!("The result of {} + {} is {}", 1, 2, result);

    // 数据类型
    let name: &str = "freemanke";
    println!("The name is {}", name);
    println!("The first char is {}", &name[0..1]);

    let friends: String = String::from("hello");
    println!("{}", friends);

    let is_success: bool = true;
    println!("The success result is {}", if is_success { 1 } else { 0 });

    let height: f32 = 200.56;
    println!("The height is {}", height);

    let age: u8 = 20;
    println!("The age is {}", age);

    let question_mark: char = '!';
    println!("The question mark is {}", question_mark);

    let names = [1, 2, 3, 4, 5];
    println!(
        "{}",
        names
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    let name_age_sexes = ("freemanke", 43, true);
    println!(
        "{} {} {}",
        name_age_sexes.0, name_age_sexes.1, name_age_sexes.2
    );
}
