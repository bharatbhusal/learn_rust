// Problem 1:
// Create a struct called Person that represents a person's information, including their name, age, and email address.
// Define an enum called Gender with two variants: Male and Female.
// Add a method to the Person struct that returns a formatted string containing the person's information.

pub struct Person {
    pub name: String,
    pub age: u8,
    pub email: String,
    pub sex: Gender,
}

#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}

impl Person {
    pub fn details(&self) -> String {
        format!(
            "Name: {}\nAge: {}\nSex: {:?}\nEmail: {}",
            self.name, self.age, self.sex, self.email
        )
    }
}
