fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;    // z is an immutable reference to x

    *y += 1; // Modify x through y
    println!("x = {}", x); // Output: x = 6

    // This will not compile as z is immutable
    // *z += 1;
} 