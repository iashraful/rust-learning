use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("-----------------------------------");
    println!("========== Gueesing Game ==========");
    println!("-----------------------------------");

    let random_num: u32 = rand::thread_rng().gen_range(1..=100);
    let mut guess_counter: u32 = 0;

    loop {
        println!("Guess your number: ");
        let mut guess: String = String::new(); // This just allocate a memory location with empty/null value.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get the input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input provided.");
                guess_counter += 1;
                continue;
            }
        };
        // println!("Secrect number is: {random_num}");
        // println!("You guessed: {guess}");

        match guess.cmp(&random_num) {
            Ordering::Equal => {
                guess_counter += 1;
                println!("Congrats. You won! Only {guess_counter} guess(es).");
                break;
            }
            Ordering::Less => {
                println!("Too small!");
                guess_counter += 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                guess_counter += 1;
            }
        }
    }
}
