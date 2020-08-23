use std::io;
// use std::time::SystemTime;
// use regex::Regex;

fn main() {
    // let reg = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    // let sys_time = SystemTime::now();
    // println!("Did our date match? {}", reg.is_match("2020-08-23"));

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to readline");

    println!("You guessed: {}", guess);
}
