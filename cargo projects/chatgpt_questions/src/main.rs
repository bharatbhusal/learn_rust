pub mod level1;

// use level1::{
//     genre::{Genre::Rock, Song},
//     mimic::*,
//     person::{Gender::Male, Person},
// };
use level1::colors::*;

fn main() {
    // println!("Hello, world!");
    // let ram = Person {
    //     name: String::from("Bharat Bhusal"),
    //     age: 21,
    //     email: String::from("bharatbhusal78@gmail.com"),
    //     sex: Male,
    // };
    // let khamos = Song {
    //     title: String::from("Khamosh"),
    //     artist: String::from("Neeta Bhusal"),
    //     duration: 1500,
    //     genre: Rock,
    // };
    // let res = safe_division(5, 2);
    // println!("{:?}", res);
    // println!("{}", ram.details());
    // println!("{}", khamos.details());
    let col1 = Color(0, 40, 53);
    let col2 = Color(256, 45, 190);
    let new_col = Color::mix(&col1, &col2);
    println!("New color: {:#?}", new_col);
    println!("Col1 color: {:#?}", col1);
    println!("Col2 color: {:#?}", col2);
}
