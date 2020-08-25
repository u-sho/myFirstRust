use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    guess_game();
}

fn guess_game () {
    println!("Guess the Secret Number!");

    let secret_num: u8  = get_u8_rand();

    loop {
        let guess_num: u8 = get_u8_user_guess();
        output_game_result(secret_num, guess_num);
        if guess_num == secret_num {
            break;
        }
    }
}

fn get_u8_rand() -> u8 {
    return rand::thread_rng().gen_range(0, 255);
}

fn get_u8_user_guess() -> u8 {
    println!("Please input your guess.");

    loop {
        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Faild to readline");

        let user_guess: u8 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a integer from 0 to 255.");
                continue;
            }
        };

        break user_guess;
    }
}

fn output_game_result(secret_num: u8, guess_num: u8) {
    match guess_num.cmp(&secret_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }
}
