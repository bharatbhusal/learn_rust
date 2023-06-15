pub fn max_ele<T: std::cmp::PartialOrd + Copy>(arr: &[T]) -> Option<T> {
    if arr.len() == 0 {
        return None;
    }
    let mut res = &arr[0];
    for each in arr {
        if each > res {
            res = each;
        }
    }
    Some(*res)
}
