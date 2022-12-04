// Data Types
// https://doc.rust-lang.org/book/ch03-02-data-types.html

#![allow(unused_variables)]

fn main() {
    // Signed integers
    let i : i8 = 0;
    let i : i16 = 0;
    let i : i32 = 0;
    let i : i64 = 0;
    let i : i128 = 0;
    let i : isize = 0;

    // Unsigned integers
    let u : u8 = 0;
    let u : u16 = 0;
    let u : u32 = 0;
    let u : u64 = 0;
    let u : u128 = 0;
    let u : usize = 0;

    // Integer literals
    let decimal = 98_222; // i32
    let hex = 0xff; // i32
    let octal = 0o77; // i32
    let binary = 0b1111_0000; // i32
    let byte = b'A'; // u8

    // Integer literals with type suffix
    let suffixed = 1_234u16; // u16 instead of i32
    // let nope = b'A'i8; not allowed

    // Floating point
    let f = 2.0; // f64
    let f: f32 = 3.0; // f32

    // Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // The Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation

    // The Character Type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1); // With type annotation
    let tup = (500, 6.4, 1); // Same without type annotation
    let (x, y, z) = tup; // Destructure into 3 separate variables: _x, _y and _z
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // The Array Type
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // With type annotation
    let a = [1, 2, 3, 4, 5]; // Same without type annotation
    let a3 = [3; 5]; // [3, 3, 3, 3, 3];
    let first = a[0];
    let second = a[1];

    // Error detected at compile time
    // let element = a[10];

    // Error detected at runtime
    //let index = 10;
    //let element = a[index]; // -> panic
}
