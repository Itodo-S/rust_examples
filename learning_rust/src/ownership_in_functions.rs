// Ownership in functions examples 
fn main() {
    // Example 1: Passing ownership to function
    let s = String::from("hello");
    takes_ownership(s);
    // println!("s: {}", s); // Error: s moved

    // Example 2: Function returns ownership
    let s2 = gives_ownership();
    println!("s2: {}", s2);

    // Example 3: Passing and returning ownership
    let s3 = String::from("rust");
    let s4 = takes_and_returns(s3);
    println!("s4: {}", s4);

    // Example 4: Ownership with non-Copy type
    let v = vec![1, 2, 3];
    takes_vector(v);
    // println!("v: {:?}", v); // Error: v moved

    // Example 5: Ownership with Copy type
    let n = 42;
    takes_integer(n);
    println!("n: {}", n); // n is still valid
}

fn takes_ownership(s: String) {
    println!("Took ownership: {}", s);
}

fn gives_ownership() -> String {
    String::from("from function")
}

fn takes_and_returns(s: String) -> String {
    println!("Got and returning: {}", s);
    s
}

fn takes_vector(v: Vec<i32>) {
    println!("Vector: {:?}", v);
}

fn takes_integer(n: i32) {
    println!("Integer: {}", n);
}
