fn main() {
    // For data types with known size in runtime, stack is applied. hence they "y" can just copy the value "5" rather than invalidating x and taking the ownership.
    let x = 5;
    let y = x;
    println!("X: {}\tY: {}", x, y);

    // Since the size of String data type is not known in runtime, heap is applied, hence "x" is droped first when y takes the ownership of the string "hi".
    let x = String::from("hi");
    let y = x;
    println!("X: {}\tY: {}", x, y);
}
