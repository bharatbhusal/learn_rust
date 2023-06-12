use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (key, file) = (&args[1], &args[2]);
    // let fil = &args[2];
    println!("Key: {:?}", key);
    println!("File: {:?}", file);
    let content = fs::read_to_string(file).expect("Error");
    println!("{}", content);
}
