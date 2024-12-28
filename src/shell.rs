use std::process::Command;

#[test]
fn test_shell() {
    let output = Command::new("uname").arg("-s").output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    if stdout.contains("Darwin") {
        println!("This is MacOS");
    }
    println!("{}", stdout);
}