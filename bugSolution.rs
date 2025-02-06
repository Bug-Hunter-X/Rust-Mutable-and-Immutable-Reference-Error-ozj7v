fn main() {
    let mut x = 5;
    { //creating a new scope
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modify x through y
    }
    let z = &x; // Now it's safe to create an immutable reference
    println!("x = {}", *z); // Output: x = 6
} 