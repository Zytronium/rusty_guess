use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret: u8 = rand::rng().random_range(1..=100);
    // println!("Debug secret: {}", secret);
    let mut input = String::new();

    println!("Guess a random number between 1 and 100. Type \"hint <guess>\" \
            to see if your next guess is too high or low.");

    loop {
        input.clear();
        println!("Enter your guess:");
        io::stdin().read_line(&mut input).unwrap();
        let hint = input.trim().to_lowercase().starts_with("hint");
        let guess: u8;

        if hint {
            guess = match input.to_lowercase().replace("hint", "").trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Please enter a valid number between 1 and 100 and try again, or prefix your guess with \"hint\" for a hint.");
                    continue;
                }
            }
        } else {
            guess = match input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Please enter a valid number between 1 and 100 and try again, or prefix your guess with \"hint\" for a hint.");
                    continue;
                }
            };
        }

        if guess == secret {
            println!("Correct, you win!");
            break;
        } else if hint {
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
