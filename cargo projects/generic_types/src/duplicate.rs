// Write a generic function that takes a list of elements and returns a new list with all duplicate elements removed.
pub fn remove_duplicate<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> Vec<T> {
    let mut new_list: Vec<T> = Vec::new();
    for each in list {
        if !new_list.contains(each) {
            new_list.push(*each);
        }
    }
    new_list
}
