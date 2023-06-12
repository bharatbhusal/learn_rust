// Create a struct called Color that represents a color with red, green, and blue components.
// Implement a method that takes two colors and returns the resulting color obtained by adding their components together.

#[derive(Debug)]
pub struct Color(pub u16, pub u16, pub u16);

impl Color {
    pub fn mix(a: &Color, b: &Color) -> Color {
        Color((a.0 + b.0) % 255, (a.1 + b.1) % 255, (a.2 + b.2) % 255)
    }
}
