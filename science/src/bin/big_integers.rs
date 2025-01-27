// Calculation for integers exceeding 128 bits are possible with BigInt.
use num::{BigInt, One};

fn factorial(x: u32) -> BigInt {
    (1..=x).fold(BigInt::one(), |factorial, i| factorial * i)
}

fn main() {
    println!("{}! equals {}", 100, factorial(100));
}
