use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};
use const_format::formatcp;

const MIN: u8 = 1;
const MAX: u8 = 100;
const INVALID_MSG: &str = formatcp!(
    "Please enter a valid number between {} and {} and try again, \
     or prefix your guess with \"hint\" for a hint.",
    MIN, MAX
);

fn main() {
    let secret: u8 = rand::rng().random_range(MIN..=MAX);
    // println!("Debug secret: {}", secret);
    let mut input = String::new();

    println!(
        "Guess a random number between {MIN} and {MAX}. \
        Type \"hint <guess>\" to see if your next guess is too high or low."
    );

    loop {
        input.clear();
        print!("Enter your guess: ");
        io::stdout().flush().expect("failed to flush stdout");

        io::stdin().read_line(&mut input).unwrap();
        
        let trimmed = input.trim();

        let mut parts = trimmed.split_whitespace();
        let first = parts.next().unwrap();
        let give_hint = first.eq_ignore_ascii_case("hint");
        let guess: u8;

        if give_hint {
            guess = match trimmed.to_lowercase().replace("hint", "").trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("{}", INVALID_MSG);
                    continue;
                }
            }
        } else {
            guess = match trimmed.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("{}", INVALID_MSG);
                    continue;
                }
            };
        }

        if guess == secret {
            println!("Correct, you win!");
            break;
        } else if give_hint {
            match guess.cmp(&secret) {
                Ordering::Less => println!("Too small, try again!"),
                Ordering::Greater => println!("Too big, try again!"),
                Ordering::Equal => {
                    // Should not happen since we already detect this above
                    println!("Correct, you win!");
                    break;
                }
            }
        } else {
            println!("Incorrect. Try again!");
        }
    }
}
