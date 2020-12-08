use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    guess_game();
}

fn guess_game() {
    print!("\n");
    println!("##############################");
    println!("## Guess the Secret Number! ##");
    println!("##############################\n");

    let secret_num = _get_rand();
    const TRY_LMT: u8 = 8;

    for cnt in 1u8..TRY_LMT {
        let guess_num: u8 = _input_player_guess(cnt);

        let is_end: i8 = _output_game_result(secret_num, guess_num, cnt + 1u8 == TRY_LMT);

        if is_end != 0i8 {
            return;
        }
    }

    fn _get_rand() -> u8 {
        return rand::thread_rng().gen_range(0u8, 255u8);
    }

    fn _input_player_guess(counter: u8) -> u8 {
        println!("Try {}. Input Your Guess!", counter);

        loop {
            let mut user_guess = String::new();
            io::stdin()
                .read_line(&mut user_guess)
                .expect("Failed to read_line");

            let user_guess: u8 = match user_guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input an integer from 0 to 255.");
                    continue;
                }
            };

            break user_guess;
        }
    }

    /**
     *  1: player win,
     *  0: turn continued,
     * -1: player lose.
     */
    fn _output_game_result(secret_num: u8, guess_num: u8, is_final_try: bool) -> i8 {
        if is_final_try && secret_num != guess_num {
            return __output_lose(secret_num);
        }

        match guess_num.cmp(&secret_num) {
            Ordering::Less => {
                println!("Too small!\n");
                return 0;
            }
            Ordering::Greater => {
                println!("Too big!\n");
                return 0;
            }
            Ordering::Equal => {
                return __output_win();
            }
        }

        fn __output_win() -> i8 {
            return ___output_win_lose(true);
        }

        fn __output_lose(secret_num: u8) -> i8 {
            println!("The Secret Number was {}", secret_num);
            return ___output_win_lose(false);
        }

        fn ___output_win_lose(is_player_win: bool) -> i8 {
            println!("###############");
            println!("## YOU {}! ##", if is_player_win { "WIN!" } else { "LOSE" });
            println!("###############\n");
            return if is_player_win { 1 } else { -1 };
        }
    }
}
