// Implement a function that calculates the factorial of a given number using recursion.
// Handle edge cases and display appropriate error messages.

fn main() {
    let num = 10;
    let res = factorial(num.clone());
    println!("Factorial of {} is {}", num, res);
}

fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
