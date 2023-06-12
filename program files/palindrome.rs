// Write a function that checks whether a given string is a palindrome or not.
// A palindrome is a word, phrase, number, or other sequence of characters that reads the same
// forward and backward. Ignore capitalization and non-alphanumeric characters.

use std::io::{self, Write};

fn main() {
    if palindrome(&input()) {
        println!("Palindrome");
    } else {
        println!("Not Palindrome");
    }
}

fn palindrome(d: &str) -> bool {
    let mut vec_d: Vec<char> = d.chars().collect();
    let len = vec_d.len();
    for i in 0..len / 2 {
        vec_d.swap(i, len - 1 - i);
    }
    let e: String = vec_d.iter().collect();
    d == e
}

fn input() -> String {
    let mut inp = String::new();
    io::stdout().flush().expect("Error");
    io::stdin().read_line(&mut inp).expect("Error");
    let inp = inp.trim();
    inp.to_string()
}
