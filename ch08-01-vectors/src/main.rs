#![allow(unused_mut)]
#![allow(unused_variables)]

fn main() {
    // Creating a New Vector

    let v: Vec<i32> = Vec::new(); // Needs type annotation, can't be inferred
    let v = vec![1, 2, 3]; // Type is inferred from values

    // Updating a Vector

    let mut v = Vec::new();
    v.push(5); // <- Type is inferred from this! Neat 😊
    v.push(6);
    v.push(7);
    v.push(8);

    // Dropping a Vector Drops Its Elements

    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here


    // Reading Elements of Vectors

    let v = vec![1, 2, 3, 4];

    let third: &i32 = &v[2];
    println!("The third element is {}.", third);

    match v.get(4) {
        Some(third) => println!("The fifth element is {}.", third),
        None => println!("There is no fifth element."),
    }

    //let does_not_exist = &v[100]; thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 100'
    let does_not_exist = v.get(100); // No panic

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    //v.push(6); error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("The first element is: {}", first);

    // Iterating over the Values in a Vector

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57]; // mut to change elements
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // Using an Enum to Store Multiple Types

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
