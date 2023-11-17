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
    loop_basics();
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

fn loop_basics() {
    let result = loop {
        println!("looping...");
        break 100;
    };
    println!("{result}");
    // Loop with label. 
    let mut counter = 0;
    'counting_loop: loop {
        println!("Counter {counter}");
        let mut inner_counter = 0;
        loop {
            if inner_counter == 2 {
                break;
            }
            if counter == 2 {
                break 'counting_loop;
            }
            println!("Inner counter: {inner_counter}");
            inner_counter += 1;
        }
        counter += 1
    }
}
