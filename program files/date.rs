// Write a program that takes a string representing a date in the format "YYYY-MM-DD" and extracts the year, month, and day
// into separate variables. Use match expressions and a custom struct to store the date components.
// Display the extracted values in a user-friendly format.

use std::io::{self, Write};

fn main() {
    let date = input();
    let date = Date {
        year: date[0..4].to_string().parse().unwrap_or(0),
        month: date[5..7].to_string().parse().unwrap_or(0),
        day: date[8..].to_string().parse().unwrap_or(0),
    };
    println!("{}:{}:{}", date.year, date.month, date.day);
}

fn input() -> String {
    let mut date_string = String::new();
    io::stdout().flush().expect("error");
    io::stdin().read_line(&mut date_string).expect("Error");
    let date_string = date_string.trim();
    date_string.to_string()
}

struct Date {
    year: u16,
    month: u8,
    day: u8,
}
