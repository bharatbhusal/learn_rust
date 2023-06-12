use std::fmt;

fn main() {
    let my_circle = Circle::new(5, Point::new(5, 1.0));
    let my_square = Square::new(54, Point::new(5.5, 4));
    my_circle.draw();
    my_square.draw();
}

trait Drawable {
    fn draw(&self) {
        println!("Default drawing");
    }
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

struct Circle<T, U, V> {
    radius: V,
    center: Point<T, U>,
}

impl<T, U, V> Circle<T, U, V> {
    fn new(radius: V, center: Point<T, U>) -> Circle<T, U, V> {
        Circle { radius, center }
    }
}
impl<T: fmt::Debug, U: fmt::Debug, V: fmt::Debug> Drawable for Circle<T, U, V> {
    fn draw(&self) {
        println!(
            "Drawing a circle of radius {:?} and center at ({:?}, {:?}).",
            self.radius, self.center.x, self.center.y
        );
    }
}
struct Square<T, U, V> {
    length: V,
    top_right: Point<T, U>,
}

impl<T, U, V> Square<T, U, V> {
    fn new(length: V, top_right: Point<T, U>) -> Square<T, U, V> {
        Square { length, top_right }
    }
}

impl<T: fmt::Display, U: fmt::Display, V: fmt::Display> Drawable for Square<T, U, V> {
    fn draw(&self) {
        println!(
            "Drawing a Square of lenght {} and top right corner at ({}, {}).",
            self.length, self.top_right.x, self.top_right.y
        )
    }
}
