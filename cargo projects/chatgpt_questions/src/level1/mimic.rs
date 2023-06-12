// Problem 3:
// Define an enum called Result<T, E> that mimics the Rust standard library's Result type.
// It should have two variants: Ok that holds a value of type T, and Err that holds a value of type E.
// Implement a function called safe_division that takes two u32 numbers as arguments and returns a Result type.
// The function should perform division and return Ok(result) if the division is successful, and Err("Division by zero")
// if the divisor is zero.

#[derive(Debug)]
pub enum Results<T, E> {
    Ok(T),
    Err(E),
}

pub fn safe_division<'a>(a: u32, b: u32) -> Results<f64, &'a str> {
    if b != 0 {
        Results::Ok((a / b).into())
    } else {
        Results::Err("Division by zero")
    }
}
