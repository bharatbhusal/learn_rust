// Create a struct called "Rectangle" with fields like "width" and "height."
// Implement an enum called "Unit" with variants like "Centimeters," "Inches," and "Feet."
// Write a function that calculates the area of a rectangle based on the provided dimensions and unit of measurement.
// Display the result in a user-friendly format.

struct Rectangle {
    width: f32,
    height: f32,
}

enum Unit {
    Centimeters,
    Inches,
    Feet,
}
impl Rectangle {
    fn area(&self, u: Unit) -> String {
        format!(
            "{} {}",
            self.width * self.height,
            match u {
                Unit::Centimeters => "centimeter square",
                Unit::Inches => "inch square",
                Unit::Feet => "feet square",
            }
        )
    }
}

fn main() {
    let mybox = Rectangle {
        width: 5.5,
        height: 2.5,
    };
    println!("The area of mybox is {}", mybox.area(Unit::Inches));
}
