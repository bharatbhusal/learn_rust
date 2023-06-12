// Create a struct called "Contact" with fields like "name," "phone," and "email."
// Implement a program that stores multiple contact objects in a HashMap, using the name as the key.
// Allow the user to add, remove, and search for contacts. Display appropriate messages based on the actions performed.

use std::collections::HashMap;

// #[derive(Debug)]
struct Contact {
    name: String,
    phone: u64,
    email: String,
}

fn main() {
    let mut phonebook: HashMap<String, Contact> = HashMap::new();

    phonebook.insert(
        "Bharat".to_string(),
        Contact {
            name: "Bharat".to_string(),
            phone: 9848041769,
            email: "bharatbhusal78@gmail.com".to_string(),
        },
    );

    add(
        &mut phonebook,
        Contact {
            name: "Neeta".to_string(),
            phone: 9816576508,
            email: "denuzabhusal@gmail.com".to_string(),
        },
    );
    search(&phonebook, "Bharat");
    search(&phonebook, "Neeta");
    remove(&mut phonebook, "Neeta");
    search(&phonebook, "Neeta");
}
fn remove(book: &mut HashMap<String, Contact>, name: &str) {
    if let Some(..) = book.get(&name.to_string()) {
        book.remove(name);
    } else {
        println!("Key({}) not found.", name);
    }
}

fn add(book: &mut HashMap<String, Contact>, record: Contact) {
    book.insert(record.name.clone(), record);
}

fn search(book: &HashMap<String, Contact>, name: &str) {
    // let res = book.get(&"Bharat");
    if let Some(res) = book.get(name) {
        println!(
            "Name: {}\nPhone: {}\nEmail: {}",
            res.name, res.phone, res.email
        );
    } else {
        println!("Key({}) not found.", name);
    }
}
