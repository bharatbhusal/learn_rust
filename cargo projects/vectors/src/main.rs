fn main() {
    //assignment
    let mut v = vec![1, 2, 3, 4, 5];

    //query-indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    //query-method
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    //iteration
    for i in &v {
        println!("{}", i);
    }
    println!("The first element is {}", &v[0]);

    //changing values.
    for i in &mut v {
        *i = *i + 1;
    }
    println!("The first element is {}", &v[0]);
}
