// Borrowing in functions examples
fn main() {
    // Example 1: Immutable borrow in function
    let s = String::from("hello");
    print_length(&s);

    // Example 2: Mutable borrow in function
    let mut s2 = String::from("rust");
    append_world(&mut s2);
    println!("After append: {}", s2);

    // Example 3: Borrowing array in function
    let arr = [1, 2, 3, 4];
    print_sum(&arr);

    // Example 4: Borrowing slice in function
    let slice = &arr[1..3];
    print_slice(slice);

    // Example 5: Borrowing struct field
    let user = User { name: String::from("Alice") };
    print_name(&user.name);
}

fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn append_world(s: &mut String) {
    s.push_str(" world");
}

fn print_sum(a: &[i32; 4]) {
    let sum: i32 = a.iter().sum();
    println!("Sum: {}", sum);
}

fn print_slice(slice: &[i32]) {
    println!("Slice: {:?}", slice);
}

struct User {
    name: String,
}

fn print_name(name: &String) {
    println!("User name: {}", name);
}
