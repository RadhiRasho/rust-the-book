use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    loop {
        println!("Guess the number!");

        let secrect_number = rand::thread_rng().gen_range(1..=100);

        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {guess}");

        match guess.cmp(&secrect_number) {
            Ordering::Less => println!("Too Small!\n"),
            Ordering::Greater => println!("Too Big!\n"),
            Ordering::Equal => {
                println!("You Win!\n");
                break;
            }
        }
    }
}
