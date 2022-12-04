// Functions
// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

fn main() {
    println!("Hello, world!");

    another_function();

    print_number(5);
    print_numbers(5, 6);

    let x = five();
    println!("The value of x is: {}", x);

    let y = plus_one(5);
    println!("The value of y is: {}", y);
}

fn another_function() {
    println!("Another function.");
}

fn print_number(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_numbers(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
