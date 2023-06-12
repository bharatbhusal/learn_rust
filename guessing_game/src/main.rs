use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Welcome to guessing game on RUST");
    let secrect = rand::thread_rng().gen_range(1, 100);
    let mut count = 0;
    let mut is_correct = false;

    while !is_correct {
        println!("Enter your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("IO Error");
        count += 1;
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a valid integer guess".red());
                continue;
            }
        };
        match guess.cmp(&secrect) {
            Ordering::Equal => {
                println!("{}", "Correct guess".green());
                println!("You got it in {}th attemp", count);
                is_correct = true;
            }
            Ordering::Greater => println!("{}", "Guess is larger".red()),
            Ordering::Less => println!("{}", "Guess is smaller".red()),
        };
    }
}
