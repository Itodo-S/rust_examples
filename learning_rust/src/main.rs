fn main() {
    //    variables definition
    let x: i32 = 5;
    println!("The value of x is: {x}");

    //   mutablie variable
    let mut y: i32 = 10;
    y += 5;
    println!("The updated value of y is: {y}");

    // scopes example
    {
        let z: i32 = 15;
        println!("The value of z in the inner scope is: {z}");
    }
    // z is not accessible here

    // shadowing
    let a: i32 = 20;
    let a: i32 = a + 5; // shadowing the previous 'a'
    println!("The value of a after shadowing is: {a}");
}
