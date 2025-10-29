// Mutability in function parameters examples in Rust
fn print_value(val: i32) {
    println!("Value: {}", val);
}

fn increment(mut num: i32) {
    num += 1;
    println!("Incremented: {}", num);
}

fn change_string(mut s: String) {
    s.push_str(" world");
    println!("Changed string: {}", s);
}

fn modify_array(mut arr: [i32; 3]) {
    arr[0] = 99;
    println!("Modified array: {:?}", arr);
}

fn set_true(flag: &mut bool) {
    *flag = true;
}

fn main() {
    print_value(10);
    increment(5);
    change_string(String::from("Hello"));
    modify_array([1, 2, 3]);
    let mut flag = false;
    set_true(&mut flag);
    println!("Flag after set_true: {}", flag);
}
