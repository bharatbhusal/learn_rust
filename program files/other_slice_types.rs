fn main() {
    let a = [4, 5, 6, 7, 8, 3, 2];
    let b = &a[..3];
    let c = &a[3..];
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
