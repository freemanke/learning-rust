use std::fs::File;
use std::io::{Read, Write};

#[test]
fn read_file() {
    let file_name = "/tmp/read_file.txt";
    let mut file = File::create(file_name).unwrap();

    let content = "This is 10";
    file.write(String::from(content).as_bytes()).unwrap();

    let mut buffer = String::new();
    let mut file = File::open(file_name).unwrap();
    file.read_to_string(&mut buffer).unwrap();

    assert_eq!(buffer, String::from(content));
}