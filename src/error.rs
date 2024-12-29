mod tests {
    
    

    #[test]
    fn read_write() {
        let mut file = match File::create("/tmp/rust.txt") {
            Ok(f) => f,
            Err(_) => panic!("Unable to create file")
        };

        file.write_all(b"Hello, world!").unwrap();
        let mut file = File::open("/tmp/rust.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "Hello, world!");
    }

    #[test]
    #[should_panic]
    fn read() {
        File::create("/rust.txt").unwrap();
    }
}