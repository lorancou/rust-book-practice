fn main() {
    // Mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // Mutability required to change the value of x
    println!("The value of x is: {}", x);

    // Constants (must be set to a constant expression)
    const X: u32 = 100_000;
    println!("The value of X is: {}", X);

    // Shadowing
    let x = 5; // *Not* mutable, shadows the previous x
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
}
