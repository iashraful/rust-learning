fn main() {
    // mutable variable
    // Here, mut is the keyword for mutability
    let mut a: u32 = 0;
    println!("a = {a}");
    a = 10;
    println!("Now a = {a}");
    // Number literals
    a = 1_000;
    println!("1000 and 1_000 are same as {a}");
    // Another function called
    another_function();
    conditional_statements(12);
}

fn another_function() {
    println!("This is from another function");

    let x = {
        let y = 5;
        y + 5
    };
    println!("X value should be 10. X = {x}");

    no_return_keyword_is_necessary();
}

fn no_return_keyword_is_necessary() -> i8 {
    10
}

fn conditional_statements(num: i16) {
    if num < 10 {
        println!("Condition is true.");
    } else {
        println!("Condition is false.")
    }
}
