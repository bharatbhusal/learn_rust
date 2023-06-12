fn main() {
    let mut sent = String::from("India is my nation.");
    let words: Vec<&str> = word_count(&sent);
    for each in words {
        println!("{}", each);
    }
}

fn word_count(s: &str) -> Vec<&str> {
    let b = s.as_bytes();
    let mut ele: Vec<&str> = vec![];
    let mut prev_ind: usize = 0;
    for (i, &each) in b.iter().enumerate() {
        if each == b' ' {
            ele.push(&s[prev_ind..i]);
            prev_ind = i + 1;
        }
    }
    ele.push(&s[prev_ind..]);
    ele
}
