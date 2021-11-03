use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let max_tries = 5;
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("I'm thinking of a number between 1 and 100...");

    println!("Guess the number!");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\n{} is too low! Go higher:", guess),
            Ordering::Greater => println!("\n{} is too high! Go lower:", guess),
            Ordering::Equal => {
                println!("\nYou got it! The secret number was {}.", secret_number);
                break;
            }
        }
    }
}
