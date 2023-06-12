use std::collections::HashMap;

fn main() {
    // let mut map: HashMap<String, i32> = HashMap::with_capacity(10);
    // map.insert(String::from("Bharat"), 20);
    // map.insert(String::from("Geetu"), 25);
    // map.insert(String::from("Neeta"), 19);
    // for (key, value) in map.iter() {
    //     println!("Key: {}\nValue: {}\n", key, value);
    // }
    // println!("Capacity: {}", map.capacity());
    // map.shrink_to(2);
    // println!("Capacity: {}", map.capacity());
    // map.entry(String::from("Bharati")).or_insert(25);
    // for (key, value) in map.iter() {
    //     println!("Key: {}\nValue: {}\n", key, value);
    // }
    // println!("Capacity: {}", map.capacity());
    // println!(
    //     "Bharat is {} years old",
    //     map.get(&String::from("Bharat")).unwrap_or(&-1)
    // );

    // println!("map = {:#?}", map);
    let sent = "Hello world Wonderful World";
    let mut freq: HashMap<String, i32> = HashMap::new();
    let words = words(sent);

    for each in words {
        let count = freq
            .entry(each.to_ascii_lowercase().to_string())
            .or_insert(0);
        *count += 1;
    }
    println!("Freq = {:#?}", freq);
}

fn words(data: &str) -> Vec<&str> {
    let mut vec: Vec<&str> = Vec::new();
    let byte_data = data.as_bytes();
    let mut prev_ind = 0;
    for (ind, &each) in byte_data.iter().enumerate() {
        if each == b' ' {
            vec.push(&data[prev_ind..ind]);
            prev_ind = ind + 1;
        }
    }
    vec.push(&data[prev_ind..]);
    vec
}
