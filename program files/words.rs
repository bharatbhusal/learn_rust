// Write a program that takes a sentence as input and splits it into individual words using string manipulation and vector data structure.
// Print each word on a separate line.

use std::io::{self, Write};

fn main() {
    let mut sentence = String::new();
    print!("Sentence: ");
    io::stdout().flush().expect("Error during flushing");
    io::stdin()
        .read_line(&mut sentence)
        .expect("Error during input");
    let sentence = sentence.trim();
    let words: Vec<&str> = words(sentence);
    // let words: Vec<&str> = sentence.split_whitespace().collect();
    display(words);
}

fn words(text: &str) -> Vec<&str> {
    let byte_sent = text.as_bytes();
    let mut res: Vec<&str> = Vec::new();
    let mut prev_count = 0;
    for (ind, &each) in byte_sent.iter().enumerate() {
        if each == b' ' {
            res.push(&text[prev_count..ind]);
            prev_count = ind + 1;
        }
    }
    res.push(&text[prev_count..]);
    res
}

fn display(data: Vec<&str>) {
    for each in data {
        println!("{}", each);
    }
}
