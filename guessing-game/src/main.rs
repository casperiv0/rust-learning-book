use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    // creates an infinite loop,
    // this allows us to keep asking for user input until the correct number is guessed
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess. Please try again.");

        // parse the input into a number
        // using match here allows us to refine and handle errors.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // possible ways to compare

        // if/else
        // if guess < secret_number {
        //     println!("Too small...");
        // } else if guess > secret_number {
        //     println!("Too big...");
        // } else {
        //     println!("You win!");
        // }

        // or via match
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small..."),
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


// Learned concepts
// match, if/else and let