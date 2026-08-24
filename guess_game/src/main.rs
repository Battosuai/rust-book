use rand::Rng;
use std::cmp::Ordering;
use std::{io, println};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number {secret_number}");

    loop {
        println!("Input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
