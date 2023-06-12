fn main() {
    let c1 = Shape::Circle { radius: 5.0 };
    let s1 = Shape::Square { length: 5.0 };
    let r1 = Shape::Rectangle {
        length: 5.0,
        breadth: 5.0,
    };

    println!("Area of Rectangle: {}", r1.area());
    println!("Area of Square: {}", s1.area());
    println!("Area of Circle: {}", c1.area());
    {
        println!("New block start");
        let r2 = Shape::Rectangle {
            length: 55.0,
            breadth: 44.3,
        };
        println!("Area of new rectangle: {}", r2.area());
        println!("New block ending");
    }
    println!("New block ended");
    println!("Out of the block");
}

enum Shape {
    Rectangle { length: f64, breadth: f64 },
    Square { length: f64 },
    Circle { radius: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { length, breadth } => length * breadth,
            Shape::Square { length } => length * length,
            Shape::Circle { radius } => 3.14 * radius * radius,
        }
    }
}

impl Drop for Shape {
    fn drop(&mut self) {
        match self {
            Shape::Rectangle {
                length: _,
                breadth: _,
            } => println!("Rectangle dropped."),
            Shape::Square { length: _ } => println!("Square dropped."),
            Shape::Circle { radius: _ } => println!("Circle dropped."),
        }
    }
}
