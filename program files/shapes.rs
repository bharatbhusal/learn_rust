// Create a module named shapes and define an enum Shape inside it.
// The enum should have variants for different shapes such as Circle, Rectangle, and Triangle.
// Each variant should have fields representing the necessary properties of the shape
// (e.g., radius for Circle, width and height for Rectangle).
// Implement a method area for the Shape enum that calculates and returns the area of the shape.

mod shapes {
    use std::f64::consts::PI;
    #[derive(Debug)]
    pub enum Shape {
        Circle(f64),
        Rectangle(f64, f64),
        Triangle(f64, f64, f64),
    }

    impl Shape {
        pub fn area(&self) -> f64 {
            match self {
                Shape::Circle(radius) => PI * radius * radius,
                Shape::Rectangle(length, breadth) => length * breadth,
                Shape::Triangle(a, b, c) => {
                    let s = self.perimeter() / 2.0;
                    let temp = (s * (s - a) * (s - b) * (s - c)).sqrt();
                    temp.round()
                }
            }
        }
        pub fn perimeter(&self) -> f64 {
            match self {
                Shape::Circle(radius) => 2.0 * 3.14 * radius,
                Shape::Rectangle(l, b) => 2.0 * (l + b),
                Shape::Triangle(a, b, c) => a + b + c,
            }
        }
    }
}

use shapes::Shape;

fn main() {
    let s1 = Shape::Circle(5.5);
    let s2 = Shape::Rectangle(5.5, 6.0);
    let s3 = Shape::Triangle(3.0, 3.0, 3.0);
    println!("Area of Triangle: {}", s3.area());
    println!("Area of Rectangle: {}", s2.area());
    println!("Area of Circle: {}", s1.area().round());
    println!("Perimeter of Triangle: {}", s3.perimeter());
    println!("Perimeter of Rectangle: {}", s2.perimeter());
    println!("Perimeter of Circle: {}", s1.perimeter());
}
