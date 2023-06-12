// Write a program that takes a sentence as input and counts the occurrences of each word.
// Store the results in a HashMap and display the word count for each unique word in descending order.

use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let string = input();
    let dict = count(string);
    display(&dict);
}

fn input() -> String {
    let mut sentence = String::new();
    print!("String: ");
    io::stdout().flush().expect("Error");
    io::stdin().read_line(&mut sentence).expect("Error");
    let sentence = sentence.trim().to_string();
    sentence
}

fn count(line: String) -> HashMap<String, u32> {
    let mut ledger: HashMap<String, u32> = HashMap::new();
    let words_vec: Vec<&str> = line.split_whitespace().collect();
    for each in words_vec {
        if let Some(value) = ledger.get(each) {
            ledger.insert(each.to_string(), value + 1);
        } else {
            ledger.insert(each.to_string(), 1);
        }
    }
    ledger
}

fn display(ledger: &HashMap<String, u32>) {
    let mut keys: Vec<&String> = ledger.keys().collect();
    keys.sort();
    for each in keys {
        println!("{}: {}", each, ledger[each]);
    }
}
