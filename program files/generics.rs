fn main() {
    let num = vec![4, 30, 2, 1, 9, 10, 28, 5];
    println!("Largest number: {}", largest(num));
    let c = vec!['a', 'b', 'e', 'f', 'w'];
    println!("Largest character: {}", largest(c));

    let p1 = Point { x: 5, y: 4 };
    let p2 = Point { x: 5.0, y: 4.0 };
    let p3 = Point { x: 5, y: 4.4 };
    let p4 = Point {
        x: 5.5,
        y: "Bharat",
    };
    println!("X = {}", p1.get_x());
    println!("Y = {}", p3.get_y());
    println!("p1 = {:#?}", p1);
    println!("p4 = {:#?}", p4);
    println!("new = {:#?}", p1.mixup(p4));
}

fn largest<T: PartialOrd + Copy>(data: Vec<T>) -> T {
    let mut ln = data[0];
    for each in data {
        if each > ln {
            ln = each;
        }
    }
    ln
}
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }
}
impl Point<i32, f64> {
    fn get_y(&self) -> &f64 {
        &self.y
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
enum Option<T> {
    Some(T),
    None,
}
