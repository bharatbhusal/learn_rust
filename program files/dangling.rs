fn main() {
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

// solution: return the string. take ownership instead of reference.
// fn dangle() -> String {
//     let s = String::from("hello");

//     s
// }

fn dangle() -> &String {
    let s = String::from("hello");

    *s
}
