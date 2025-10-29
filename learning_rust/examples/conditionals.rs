// Conditionals examples in Rust
fn main() {
    let x = 10;
    if x > 5 {
        println!("x is greater than 5");
    }

    let y = 3;
    if y == 0 {
        println!("y is zero");
    } else {
        println!("y is not zero");
    }

    let z = 7;
    if z < 5 {
        println!("z is less than 5");
    } else if z == 5 {
        println!("z is 5");
    } else {
        println!("z is greater than 5");
    }

    let is_even = if x % 2 == 0 { true } else { false };
    println!("x is even: {}", is_even);

    let grade = 85;
    let result = if grade >= 50 { "Pass" } else { "Fail" };
    println!("Result: {}", result);
}
