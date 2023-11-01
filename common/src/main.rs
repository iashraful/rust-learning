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
}

fn another_function() {
    println!("This is from another function");
}
