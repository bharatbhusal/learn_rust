fn main() {
    //Box basics
    let data1 = Box::new(5);
    println!("{}", data1);

    // linked list
    let my_list = next(4, Box::new(5, Box::new(x)));
}

enum List {
    data(i32),
    next(Box<List>),
}
