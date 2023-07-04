// Write a function that takes a nested vector of integers and returns a new vector containing all the elements from the nested list in a flattened form.
pub fn nested(vc: Vec<Vec<i32>>) -> Vec<i32> {
    let mut my_vec = Vec::new();
    for i in vc {
        for j in i {
            my_vec.push(j);
        }
    }
    my_vec
}
