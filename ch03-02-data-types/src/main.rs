fn main() {
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
