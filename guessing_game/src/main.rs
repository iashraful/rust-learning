use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("-----------------------------------");
    println!("========== Gueesing Game ==========");
    println!("-----------------------------------");

    let random_num: u8 = rand::thread_rng().gen_range(1..=100);

    println!("Guess your number: ");
    let mut tmp: String = String::new(); // This just allocate a memory location with empty/null value.
    io::stdin()
        .read_line(&mut tmp)
        .expect("Failed to get the input.");

    let guess: u8 = tmp.trim().parse().expect("Expecting a number.");
    println!("Secrect number is: {random_num}");
    println!("You guessed: {guess}");

    match guess.cmp(&random_num) {
        Ordering::Equal => println!("You won!"),
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
    }
}
