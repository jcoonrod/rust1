use std::{io, env};

fn main() {
    println!("Please enter some text:");

    // Create a mutable string to store the input.
    let mut input = String::new();

    // Read the line from stdin, handle potential errors, and store the result in 'input'.
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Print the input back to the user. The read line includes the newline character.
    println!("You entered: {}", input);
}
