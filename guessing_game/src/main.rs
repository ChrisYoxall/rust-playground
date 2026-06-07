use std::cmp::Ordering;
use std::io;

// First tutorial from The Book https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html

fn main() {
    println!("Guess the number!");

    // Variables are immutable by default. Use the 'mut' keyword to make the mutable
    let secret_number = rand::random_range(1..=100);

    // Display the secret number to simplify testing
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // Create variable and assign a new empty 'String' instance. Later on we append user input
        // using 'read_line' and use a reference to this variable (i.e. use '&'). A reference is a
        // way to let multiple parts of your code access one piece of data without needing to copy
        // that data into memory multiple times.
        let mut guess = String::new();

        // Need to do something with the 'Result' returned from 'read_line'. Will use 'expect' here
        // to keep this example short, but should instead write error handling code.
        // Refer to Chapter 9 of The Book
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadow the previous variable guess with a new one
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
