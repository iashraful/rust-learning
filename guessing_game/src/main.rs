use std::io;

fn main() {
    println!("-----------------------------------");
    println!("========== Gueesing Game ==========");
    println!("-----------------------------------");
    println!("Guess your number: ");
    let mut guess: String = String::new(); // This just allocate a memory location with empty/null value.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to get the input.");
    println!("You guessed: {guess}");
}
