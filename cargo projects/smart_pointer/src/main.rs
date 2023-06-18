fn main() {
    //Box basics
    let data1 = Box::new(5);
    println!("{}", data1);

    // linked list
}

enum List {
    data(i32),
    next(Box<List>),
}
