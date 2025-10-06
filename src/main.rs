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
        // TODO: handle error when input is not expected format
        let hint = input.trim().to_lowercase().starts_with("hint");
        let guess: u8;
        if hint {
            guess = input.trim().to_lowercase().replace("hint", "").trim().parse::<u8>().unwrap();
        } else {
            guess = input.trim().parse::<u8>().unwrap();
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
