// Write a program that takes user input for their age and determines if they are eligible to vote in a specific country.
// Use appropriate data types and control flow statements to display the result.
use std::cmp::Ordering;
use std::io::{self, Write};

const VOTE_ELIGIBLE_AGE: u8 = 20;

fn main() {
    let mut name = String::new();
    print!("Name: ");
    io::stdout().flush().expect("Error");
    io::stdin().read_line(&mut name).expect("Error");

    let mut age = String::new();
    print!("Age: ");
    io::stdout().flush().expect("Error");
    io::stdin().read_line(&mut age).expect("Error");
    let age: u8 = age.trim().parse().unwrap_or(0);

    match age.partial_cmp(&VOTE_ELIGIBLE_AGE) {
        Some(Ordering::Less) => println!("You can not vote"),
        None => println!("You can not vote"),
        _ => println!("You can vote"),
    }
}
