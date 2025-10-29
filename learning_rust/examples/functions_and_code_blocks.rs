// Functions and code blocks examples in Rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn print_message() {
    println!("Hello from a function!");
}

fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

fn block_expression() -> i32 {
    let x = 5;
    let y = {
        let z = 3;
        x + z // block returns value
    };
    y * 2
}

fn main() {
    print_message();
    println!("Add: {}", add(2, 3));
    println!("Multiply: {}", multiply(4, 5));
    let result = block_expression();
    println!("Block expression result: {}", result);
    // Inline code block
    let square = |n: i32| { n * n };
    println!("Square of 6: {}", square(6));
}
