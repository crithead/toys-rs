// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the secret number (between 1 and 100, inclusive).");

    let secret = rand::rng().random_range(1..=100);

    loop {
        println!("What is your guess?");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Number not found");
        println!("You guessed {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("{} is low.", guess)
            }
            Ordering::Greater => {
                println!("{} is high.", guess)
            }
            Ordering::Equal => {
                println!("Correct!");
                println!("The secret number is {}.", secret);
                break;
            }
        }
    }
}
