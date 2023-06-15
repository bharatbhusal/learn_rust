pub fn swap<T: Copy>(x: &mut T, y: &mut T) {
    let temp = *x;
    *x = *y;
    *y = temp;
}
