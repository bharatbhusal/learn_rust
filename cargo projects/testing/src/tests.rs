use super::rectangle::*;

#[test]
fn passes_fit() {
    let r1 = Rectanlge::new(5, 5);
    let r2 = Rectanlge::new(4, 4);
    assert!(r1.fits(&r2));
    assert!(!r2.fits(&r1));
}

#[test]
fn passes_area() {
    assert_eq!(Rectanlge::new(5, 5).area(), 25);
    assert_ne!(Rectanlge::new(4, 5).area(), 25);
}

#[test]
// Test with false message
fn fails_fit() {
    let r1 = Rectanlge::new(5, 5);
    let r2 = Rectanlge::new(4, 4);
    assert!(
        r2.fits(&r1),
        "The rectangle is smaller than the argument rectangle."
    );
}
