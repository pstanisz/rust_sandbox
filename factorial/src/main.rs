// Copyright (c) 2022, Piotr Staniszewski

use std::io;

fn first_factorial(x : u64) -> u64 {
    if x == 0 {
        1
    } else {
        match x.checked_mul(first_factorial(x - 1)) {
            Some(value) => {
                value
            }
            None => {
                println!("Overflow!");
                0
            }
        }
    }
}

fn main() {
    println!("Calculating factorial");

    println!("Specify number: ");

    let mut value_text = String::new();

    io::stdin()
        .read_line(&mut value_text)
        .expect("Failed to read value");

    let value: u64 = value_text
        .trim()
        .parse()
        .expect("Unsigned number expected!");

    let result = first_factorial(value);

    println!("{value}! = {result}");
}
