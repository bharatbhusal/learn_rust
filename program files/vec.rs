use NewType::*;
fn main() {
    // let mut v1: Vec<i32> = vec![1, 2, 40];
    // v1[0] = 100;
    // println!("{:#?}", v1);
    // v1.push(44);
    // println!("{:#?}", v1);
    // v1.insert(0, 9);
    // println!("{:#?}", v1);
    // v1.pop();
    // println!("{:#?}", v1);
    // v1.remove(1);
    // println!("{:#?}", v1);
    // let first = v1.get(0);
    // println!("{:#?}", first);
    // let invalid = v1.get(10);
    // println!("{:#?}", invalid);
    // v1.sort();
    // println!("{:#?}", v1);
    let v = vec![Int(32), Float(3.2), Name(String::from("Bharat"))];
    // println!("{:#?}", v);

    for i in &v {
        println!("{:?}", i);
    }
}
#[derive(Debug)]
enum NewType {
    Int(i32),
    Float(f32),
    Name(String),
}
