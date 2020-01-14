fn main() {
    // Signed integers
    let _i : i8 = 0;
    let _i : i16 = 0;
    let _i : i32 = 0;
    let _i : i64 = 0;
    let _i : i128 = 0;
    let _i : isize = 0;

    // Unsigned integers
    let _u : u8 = 0;
    let _u : u16 = 0;
    let _u : u32 = 0;
    let _u : u64 = 0;
    let _u : u128 = 0;
    let _u : usize = 0;

    // Integer literals
    let _decimal = 98_222; // i32
    let _hex = 0xff; // i32
    let _octal = 0o77; // i32
    let _binary = 0b1111_0000; // i32
    let _byte = b'A'; // u8

    // Integer literals with type suffix
    let _suffixed = 1_234u16; // u16 instead of i32
    // let _nope = b'A'i8; not allowed

    // Floating point
    let _f = 2.0; // f64
    let _f: f32 = 3.0; // f32

    // Numeric Operations
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    // The Boolean Type
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // The Character Type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // The Tuple Type
    let _tup: (i32, f64, u8) = (500, 6.4, 1); // With type annotation
    let tup = (500, 6.4, 1); // Same without type annotation
    let (_x, _y, _z) = tup; // Destructure into 3 separate variables: _x, _y and _z
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    // The Array Type
    let _a: [i32; 5] = [1, 2, 3, 4, 5]; // With type annotation
    let a = [1, 2, 3, 4, 5]; // Same without type annotation
    let _a3 = [3; 5]; // [3, 3, 3, 3, 3];
    let _first = a[0];
    let _second = a[1];

    // Error detected at compile time
    // let _element = a[10];

    // Error detected at runtime
    //let index = 10;
    //let _element = a[index]; // -> panic
}
