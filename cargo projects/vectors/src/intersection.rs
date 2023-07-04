// Write a function that takes two vectors of integers and returns a new vector containing the intersection of the two input vectors (i.e., elements that appear in both vectors).

pub fn insertion(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    let mut new_lst = Vec::new();
    for i in vec2 {
        if vec1.contains(&i) {
            new_lst.push(i);
        }
    }
    new_lst
}
