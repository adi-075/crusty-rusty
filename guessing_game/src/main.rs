//Importing the IO Library part of the standard library
use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //Entry point
    println!("Welcome to the guessing game!"); //println! macro

    let secret_number = rand::thread_rng().gen_range(1..=100); //start..=end
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess!");

        let mut guess = String::new(); //String::new, a function that returns a new instance of a String
                                       //let is used to create variables
                                       //Rust variables are immutable by default

        io::stdin()
        .read_line(&mut guess)
        /* Takes the user's input in std input and appends it to the string
        a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.*/
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Same as
        //io::stdin().read_line(&mut guess).expect("Failed to read line");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Greater => println!("{}", "Too Large!".red()),
            Ordering::Equal => {
                println!("{}", "You have won!".green());
                break;
            }
        }
    }
}
