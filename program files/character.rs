fn main() {
    let sent = "hello there".to_string();
    println!("{:?}", find(&sent, b't'));
}

fn find(data: &String, key: u8) -> Option<&str> {
    let b_data = data.as_bytes();
    for (ind0, &each0) in b_data.iter().enumerate() {
        println!("{}, {}, {}", ind0, each0, key);
        if each0 == key {
            let init_ind = ind0;
            for (ind1, &each1) in b_data[ind0..].iter().enumerate() {
                if each1 == b' ' {
                    let final_ind = ind1;
                    return Some(&data[init_ind..final_ind]);
                }
            }
        }
    }
    None
}
