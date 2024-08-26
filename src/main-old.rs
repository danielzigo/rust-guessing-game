// To obtain user input and then print the result as output, 
// we need to bring the io input/output library into scope. 
// The io library comes from the standard library, known as std:

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // In Rust, variables are immutable by default
    // To make a variable mutable, we need to use the mut keyword.
    let mut guess = String::new(); // String::new => a function that returns a new instance of a String

    io::stdin()
        // read_line() => reads a line from standard input and appends the contents to a string
        // &mut guess => a mutable reference to the guess variable
        .read_line(&mut guess)
        // expect() => handles the error if the read_line() function fails
        .expect("Failed to read line");

    // i think the above lines would normally read:
    // io:stdin().read_line(&mut guess).expect("Failed to read line")

    println!("You guessed: {}", guess);
}