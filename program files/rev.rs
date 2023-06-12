// Write a function that reverses a given string.
// For example, the input "Hello, World!" should be reversed to "!dlroW ,olleH".

use std::io::{self, Write};

fn main() {
    let string = input();
    let reverse = reversed(&string);
    println!("Reverse of \"{}\" is \"{}\"", string, reverse);
}

fn input() -> String {
    let mut inp = String::new();
    io::stdout().flush().expect("Error");
    io::stdin().read_line(&mut inp).expect("Error");
    let inp = inp.trim();
    inp.to_string()
}

fn reversed(data: &str) -> String {
    let mut vec: Vec<char> = data.chars().collect();
    let len = vec.len();
    for i in 0..len / 2 {
        vec.swap(i, len - i - 1);
    }
    vec.iter().collect::<String>()
}
