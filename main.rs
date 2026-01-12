use std::{io, env};

fn main() {
    let key="DEBUG";
    println!("Please enter text for DEBUG env variable:");

    // Create a mutable string to store the input.
    let mut input = String::new();

    // Read the line from stdin, handle potential errors, and store the result in 'input'.
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

// set the debug environment variable
    env::set_var(key,input);

    // print the current environment variables
    
    println!("All environment variables:");
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }


    
}

