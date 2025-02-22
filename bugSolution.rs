fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modify x through y
    } // y goes out of scope

    let z = &x; // z is an immutable reference to x
    println!("x = {}", x); // Prints x = 6
    println!("x = {}", *z); // This is now safe
} 