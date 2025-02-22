fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y += 1; // Modify x through y
    println!("x = {}", x); // x is now 6

    // This line will compile but leads to undefined behavior at runtime!
    println!("x = {}", *z); // Dereferencing z after x was modified via y
}