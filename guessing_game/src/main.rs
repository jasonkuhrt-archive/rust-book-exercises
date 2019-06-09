use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;

fn main() {
    println!("");
    println!("~~ Welcome to the Number Guessing Game! ~~");
    println!("    Guess the secret number, 1 to 100");
    println!("");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess_count = 0;
    loop {
        print!("==> Guess: ");
        std::io::stdout().flush().expect("Failed to flush stdout");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "(Invalid answer, \"{}\" is not a number from 1 to 100, try again)",
                    guess.trim()
                );
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                guess_count = guess_count + 1;
                println!("Nope, too small");
            }
            Ordering::Greater => {
                guess_count = guess_count + 1;
                println!("Nope, too large");
            }
            Ordering::Equal => {
                println!("Got it! Score {}", guess_count * -1);
                break;
            }
        }
    }
}
