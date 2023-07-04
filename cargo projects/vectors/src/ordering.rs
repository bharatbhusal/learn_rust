// Write a function that takes a vector of integers and returns a new vector with the elements sorted in decreasing order of their frequency of occurrence. If multiple elements have the same frequency, they should be sorted in ascending order.

pub fn order(vc: Vec<i32>) -> Vec<i32> {
    let mut new_vec = vc.clone();
    new_vec.sort_by(|a, b| b.cmp(a));

    new_vec
}
