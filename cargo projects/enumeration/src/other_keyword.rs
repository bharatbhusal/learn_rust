pub fn other_keyword() {
    let a = 50;
    match a {
        1 => println!("important"),
        3 => println!("important"),
        5 => println!("important"),
        //if you want to use the value.
        other => println!("{}", other),
        //if you don't want to use the value.
        // _ => println!("None of our business"),
    }
}
