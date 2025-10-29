// Ownership basics examples
fn main() {
    // Example 1: Simple ownership move
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // Error: s1 moved

    // Example 2: Copy types (i32 implements Copy)
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // Example 3: Ownership with function
    let s3 = String::from("world");
    takes_ownership(s3);
    // println!("s3: {}", s3); // Error: s3 moved

    // Example 4: Returning ownership
    let s4 = gives_ownership();
    println!("s4: {}", s4);

    // Example 5: Ownership transfer and return
    let s5 = String::from("rust");
    let s6 = takes_and_gives_back(s5);
    println!("s6: {}", s6);
}

fn takes_ownership(s: String) {
    println!("Took ownership of: {}", s);
}

fn gives_ownership() -> String {
    String::from("ownership")
}

fn takes_and_gives_back(s: String) -> String {
    s
}
