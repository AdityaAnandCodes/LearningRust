use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("I have chosen a number between 1 and 100. Can you guess it?");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        // Read the user's input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // Check the guess
        if guess < secret_number {
            println!("Too small!");
        } else if guess > secret_number {
            println!("Too big!");
        } else {
            println!("You guessed it! The number was {}.", secret_number);
            break;
        }
    }
}
