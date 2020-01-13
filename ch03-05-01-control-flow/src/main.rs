use std::io;

fn main() {
    // if Expressions

    println!("Enter a number");

    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = number.trim().parse()
        .expect("Please type a number!");

    if number < 5 {
        println!("The number is lower than five");
    } else {
        println!("The number is greater or equal five");
    }

    if number != 0 {
        println!("The number is something other than zero");
    }    

    if number % 4 == 0 {
        println!("The number is divisible by 4");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 4, 3, or 2");
    }

    let message = if number < 0 {
        "strictly negative"
    } else if number > 0 {
        "strictly positive"
    } else {
        "zero"
    };
    println!("The number is {}", message);

    // Repetition with Loops

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The loop result is: {}", result);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value at index {} is: {}", index, a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
