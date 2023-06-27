use std::ffi::c_int;
use std::io;

fn main() {
    println!("Guess the Number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    let number: c_int;

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read Line");

    println!("You guessed: {guess}");
}
