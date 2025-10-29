// Operators examples in Rust
fn main() {
    // Arithmetic operators
    let sum = 5 + 3;
    let diff = 10 - 4;
    let product = 2 * 6;
    let quotient = 8 / 2;
    let remainder = 9 % 4;
    println!("sum={}, diff={}, product={}, quotient={}, remainder={}", sum, diff, product, quotient, remainder);

    // Comparison operators
    let a = 7;
    let b = 5;
    println!("a > b: {}", a > b);
    println!("a < b: {}", a < b);
    println!("a == b: {}", a == b);
    println!("a != b: {}", a != b);

    // Logical operators
    let t = true;
    let f = false;
    println!("t && f: {}", t && f);
    println!("t || f: {}", t || f);
    println!("!t: {}", !t);

    // Assignment operators
    let mut x = 10;
    x += 2;
    println!("x after += 2: {}", x);
    x *= 3;
    println!("x after *= 3: {}", x);
}
