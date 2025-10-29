// Control flow examples in Rust
fn main() {
    // Loop with break
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("Loop break at count = {}", count);
            break;
        }
    }

    // While loop
    let mut n = 5;
    while n > 0 {
        println!("n = {}", n);
        n -= 1;
    }

    // For loop over range
    for i in 0..4 {
        println!("For loop i = {}", i);
    }

    // For loop over array
    let arr = [10, 20, 30];
    for val in arr.iter() {
        println!("Array value: {}", val);
    }

    // Continue in loop
    for i in 0..5 {
        if i % 2 == 0 {
            continue;
        }
        println!("Odd number: {}", i);
    }
}
