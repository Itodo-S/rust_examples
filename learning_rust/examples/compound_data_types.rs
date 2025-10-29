// Compound data types examples in Rust
fn main() {
    // Tuple
    let tup: (i32, f64, char) = (500, 6.4, 'z');
    println!("Tuple: {:?}", tup);

    // Accessing tuple elements
    let (x, y, z) = tup;
    println!("Destructured tuple: x={}, y={}, z={}", x, y, z);

    // Array
    let arr: [i32; 4] = [1, 2, 3, 4];
    println!("Array: {:?}", arr);

    // Accessing array elements
    println!("First element: {}", arr[0]);

    // Slices
    let slice = &arr[1..3];
    println!("Slice: {:?}", slice);
}
