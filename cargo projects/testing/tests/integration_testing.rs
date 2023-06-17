use testing::rectangle::*;

use crate::supporting_code::area;
mod supporting_code;
#[test]
fn passes_area() {
    assert_eq!(Rectanlge::new(5, 5).area(), 25);
    assert_ne!(Rectanlge::new(4, 5).area(), 25);
}

#[test]
fn passes_area_from_supporting_code() {
    assert_eq!(area(5, 5), 25);
    assert_ne!(area(5, 4), 25);
}
