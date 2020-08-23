use rand::Rng;
use std::cmp::Ordering;
use std::io;
// use std::time::SystemTime;
// use regex::Regex;

fn main() {
    // let reg = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    // let sys_time = SystemTime::now();
    // println!("Did our date match? {}", reg.is_match("2020-08-23"));

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    /* input u32 (unsigned 32 bits integer) */
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to readline");
    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Please type a non-negative integer less than 2^32!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
