use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Generating random number ...");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    println!("Guess the number!");

    loop {
        println!("Input you guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line?");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You guessed the number!");
                println!();
                println!("Exiting ...");
                break;
            }
        }
    }
}
