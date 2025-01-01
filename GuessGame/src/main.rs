use std::io;
use rand::Rng;


fn main() {
    println!("Guess a number! ");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=101);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

    match guess.trim().parse::<u32>(){
        Ok(num) => {
            if num == secret_number {
                println!("You win!");
            } else {
                println!("You lose! The secret number was: {}", secret_number);
            }
        },
        Err(_) => {
            println!("Please input a number!");
        }
    }

}
