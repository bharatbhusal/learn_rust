fn main() {
    let s = String::from("नेपाली");
    println!("{:#?}", s.bytes());

    for i in s.chars() {
        println!("{}", i);
    }
    for i in s.bytes() {
        println!("{}", i);
    }
}
