## Intro

**Rust** is a memory efficient fast language for low level programming.

## What is all about this repo?

I am very much new to Rust and other low level languages. Such as Go, Rust, C, etc. So, I'm trying to learn Rust to enlearge my knowledge. I'll try to cover up evrything here as much as I can.

## Keypoints

* In Rust we can not write x = y = 10. Because assignment operator does not return value to the variable. In Rust it works different way.
* Rust has three types of loop, they are, `loop`, `for`, `while`
* In rust you can return from loop: `loop`. Example:

```rust
loop {
    if something == 0 {
        break "Return value here."
    }
}
```

* You can also labeled you loop: `loop`. Based on the label you can break the loop from any scope.

```rust
fn main() {
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

```
