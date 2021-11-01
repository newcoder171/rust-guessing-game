use rand::prelude::*;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("I'm thinking of a number between 1 and 100...");

    println!("Guess the number!");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Failed to parse your guess.");

        if guess > secret_number {
            println!("\n{} is too high! Go lower:", guess)
        } else if guess < secret_number {
            println!("\n{} is too low! Go higher:", guess)
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
    }
}