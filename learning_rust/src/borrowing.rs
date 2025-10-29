// Borrowing examples
fn main() {
    // Example 1: Immutable borrow
    let s = String::from("hello");
    print_str(&s);
    println!("After borrow: {}", s);

    // Example 2: Multiple immutable borrows
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);

    // Example 3: Mutable borrow
    let mut s2 = String::from("rust");
    change_str(&mut s2);
    println!("After mutable borrow: {}", s2);

    // Example 4: Cannot borrow as mutable and immutable at same time
    // let r3 = &s2;
    // let r4 = &mut s2; // Error: cannot borrow as mutable because it is also borrowed as immutable

    // Example 5: Borrowing arrays
    let arr = [1, 2, 3];
    print_array(&arr);
}

fn print_str(s: &String) {
    println!("Borrowed string: {}", s);
}

fn change_str(s: &mut String) {
    s.push_str(" language");
}

fn print_array(a: &[i32; 3]) {
    println!("Borrowed array: {:?}", a);
}
