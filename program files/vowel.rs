// Write a program that takes a string as input and checks if it starts with a vowel or a consonant.
// Use slice operations and match expressions to determine the result.
// Print "Vowel" or "Consonant" accordingly.
use std::io::{self, Write};

fn main() {
    let mut my_string = String::new();
    print!("Your String: ");
    io::stdout().flush().expect("Error");
    io::stdin().read_line(&mut my_string).expect("Error");
    let my_string = my_string.trim();
    if let Some(first_letter) = my_string.chars().next() {
        match is_vowel(&first_letter) {
            true => println!("The string starts with Vowel"),
            _ => println!("The string starts with Consonant"),
        }
    } else {
        println!("Invalid input");
    }
}

fn is_vowel(letter: &char) -> bool {
    match letter {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
