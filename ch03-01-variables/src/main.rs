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

    // Tuple
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup; // Makes 3 separate variables
    println!("The value of y is: {}", y);

    // Array    
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index]; // -> panic
    println!("The value of element is: {}", element);

    //const INDEX : usize = 10;
    //let element = a[INDEX];
    // => error on cargo build (*NOT* cargo check?!)
}
