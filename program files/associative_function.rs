fn main() {
    let breadth = 5.0;
    let length = 4.0;
    let mut r1 = Rectangle { length, breadth };

    // before change
    println!("Area: {}", r1.area());
    println!("Perimeter: {}", r1.perimeter());
    println!("Length: {}\nBreadth: {}", r1.get_len(), r1.get_bred());
    println!("{:#?}", r1);

    // changing values
    let new_length = 10.0;
    let new_breadth = 20.0;
    r1.set_len(new_length);
    r1.set_bred(new_breadth);

    // after change
    println!("Area: {}", r1.area());
    println!("Perimeter: {}", r1.perimeter());
    println!("Length: {}\nBreadth: {}", r1.get_len(), r1.get_bred());
    println!("{:#?}", r1);

    // New instance with values squared
    let r2 = r1.square_of_instance();
    println!("Area: {}", r2.area());
    println!("Perimeter: {}", r2.perimeter());
    println!("Length: {}\nBreadth: {}", r2.get_len(), r2.get_bred());
    println!("{:#?}", r2);

    // New instance using associative method.
    let r3 = Rectangle::create(5.0, 2.0);
    println!("Area: {}", r3.area());
    println!("Perimeter: {}", r3.perimeter());
    println!("Length: {}\nBreadth: {}", r3.get_len(), r3.get_bred());
    println!("{:#?}", r3);
}

#[derive(Debug)]
struct Rectangle {
    length: f32,
    breadth: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.breadth * self.length
    }
    fn perimeter(&self) -> f32 {
        2.0 * (self.length + self.breadth)
    }
    fn get_len(&self) -> f32 {
        self.length
    }

    fn get_bred(&self) -> f32 {
        self.breadth
    }

    fn set_len(&mut self, new_len: f32) {
        self.length = new_len;
    }

    fn set_bred(&mut self, new_bred: f32) {
        self.breadth = new_bred;
    }

    fn square_of_instance(&self) -> Rectangle {
        Rectangle {
            length: self.get_len() * 2.0,
            breadth: self.get_bred() * 2.0,
        }
    }
}
impl Rectangle {
    fn create(len: f32, bred: f32) -> Rectangle {
        Rectangle {
            length: len,
            breadth: bred,
        }
    }
}

impl Drop for Rectangle {
    fn drop(&mut self) {
        println!("Object dropped.");
    }
}
