// Create a HashMap to store the names of students as keys and their corresponding grades as values.
// Implement a function that takes a student's name as input and returns their grade.
// If the student is not found, display an appropriate error message.

use std::collections::HashMap;
fn main() {
    let mut score_board: HashMap<String, f32> = HashMap::new();
    populate_ledger(&mut score_board, String::from("Neeta"), 50.2, false);
    populate_ledger(&mut score_board, String::from("Geetu"), 40.0, false);
    populate_ledger(&mut score_board, String::from("Geetu"), 50.0, true);
    populate_ledger(&mut score_board, String::from("Bharat"), 60.0, false);
    println!("{:#?}", score_board);
    println!("{}", report(&score_board, "Geestu").unwrap_or(&0.0));
}

fn report<'a>(ledger: &'a HashMap<String, f32>, key: &'a str) -> Option<&'a f32> {
    ledger.get(key)
}
fn populate_ledger(
    ledger: &mut HashMap<String, f32>,
    key: String,
    value: f32,
    update_repeat: bool,
) {
    if update_repeat {
        ledger.insert(key, value);
    } else {
        ledger.entry(key).or_insert(value);
    }
}
