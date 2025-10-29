// Dereferencing examples in Rust
fn main() {
    // Example 1: Dereferencing a reference
    let x = 5;
    let y = &x;
    println!("Dereferenced y: {}", *y);

    // Example 2: Dereferencing a Box pointer
    let b = Box::new(10);
    println!("Dereferenced Box: {}", *b);

    // Example 3: Mutable dereferencing
    let mut z = 7;
    let p = &mut z;
    *p += 1;
    println!("After mut deref: {}", z);

    // Example 4: Dereferencing in function
    let a = 20;
    print_deref(&a);

    // Example 5: Dereferencing a struct field
    let s = String::from("hello");
    let s_ref = &s;
    println!("First char: {}", s_ref.chars().next().unwrap());
}

fn print_deref(val: &i32) {
    println!("Dereferenced in function: {}", *val);
}
