use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game!");
    println!("Remember your guess should be between 1 and 100");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess: String = String::new();

        println!("Please provide your guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Couldn't read the line!");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("Please pick a number between 1 and a 100!");

            continue;
        }

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Correct number!");
                break;
            }
        }
    }
}
