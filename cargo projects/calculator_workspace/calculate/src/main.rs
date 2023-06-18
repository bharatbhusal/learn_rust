use adder::*;
use multiplier::*;
use subtractor::*;

fn main() {
    println!("Hello, world!");
    println!("Sum: {}", adder(5, 4));
    println!("Product: {}", multiplier(5, 4));
    println!("Difference: {}", subtractor(5, 4));
}
