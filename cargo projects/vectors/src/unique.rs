pub fn unique(lst: Vec<i32>) -> Vec<i32> {
    let mut new = Vec::new();
    for i in lst {
        if !new.contains(&i) {
            new.push(i);
        }
    }
    new.sort();
    new
}
