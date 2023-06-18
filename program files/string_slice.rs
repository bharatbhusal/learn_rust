fn main() {
    let slice = "My name\n is Bharat\n Bhusal.";
    let chars = slice.chars().collect::<Vec<char>>();
    let lines = slice.lines().collect::<Vec<&str>>();
    println!("{:?}", chars);
    println!("Slice len: {}", slice.len());
    println!("Lines count: {}", lines.len());
    println!("Characters count: {}", chars.len());

    let string = String::from("Hi Bharat! How are you?");
    let mut slice1: &str = &string;
    let slice2 = &mut slice1;
    // let slice3 = &mut slice1;
    // println!("Slice1: {}\nSlice2: {}\nSlice3: {}", slice1, slice2, slice3);
    // println!("Slice1: {}\nSlice2: {}", slice1, slice2);
    println!("Slice2: {}", slice2);
    println!("Slice1: {}", slice1);

    // &str and &String is same. Both are reference to string. string slice(&str) doesn't own data. it take the reference of string.
    let string1 = String::from("fuzz");
    let string2 = "fuzz";
    println!("Characters in string1: {}", count_chars(&string1));
    println!("Characters in string2: {}", count_chars(string2));
}

fn count_chars(a: &str) -> u32 {
    a.len() as u32
}
