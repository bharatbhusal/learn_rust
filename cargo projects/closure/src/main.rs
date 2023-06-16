fn main() {
    // Immutable closure
    let name1 = "Bharat";
    println!("Name before closure: {}", name1);
    let immutable_closure = |x| println!("Hi {}", x);
    immutable_closure(name1);
    println!("Name after closure: {}\n", name1);

    //Mutable closure
    let mut name2 = "Bharat".to_string();
    println!("Name before closure: {}", name2);
    let immutable_closure = |x: &mut String| {
        *x = "Geeta".to_string();
        println!("Hi {}", x)
    };
    immutable_closure(&mut name2);
    println!("Name after closure: {}\n", name2);

    //Owned closure
    let name2 = "Bharat".to_string();
    println!("Name before closure: {}", name2);
    let immutable_closure = |mut x: String| {
        x = "Geeta".to_string();
        println!("Hi {}", x)
    };
    immutable_closure(name2);
    println!(
        "Name after closure: {}\n", /*name2 */
        "cannot display name"
    );
}
