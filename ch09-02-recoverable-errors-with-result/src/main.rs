// Recoverable Errors with Result
// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::error::Error;
use std::fs::{self,File};
use std::io::{self,ErrorKind,Read};

fn main() {

    let f = File::open("hello.txt");

    // let f: u32 = File::open("hello.txt");
    // error[E0308]: mismatched types
    // (...)
    //   = note: expected type `u32`
    //          found type `std::result::Result<std::fs::File, std::io::Error>`

    // Matching on different errors

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(create_error) => panic!("Problem creating the file: {:?}", create_error),
            },
            open_error => panic!("Problem opening the file: {:?}", open_error),
        },
    };

    // More readable version with closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Shortcuts for Panic on Error: unwrap and expect

    //let f = File::open("hello_missing.txt").unwrap();
    //let f = File::open("hello_missing.txt").expect("Failed to open hello_missing.txt");

    // Propagating Errors

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    // A Shortcut for Propagating Errors: the ? Operator

    fn read_username_from_file_2() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // Even shorter, chaining the two functoin calls
    fn read_username_from_file_3() -> Result<String, io::Error> {
       let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    // Even shorter, using std::fs
    fn read_username_from_file_4() -> Result<String, io::Error> {
       fs::read_to_string("hello.txt")
    }

    // The ? Operator Can Be Used in Functions That Return Result

    //let f = File::open("hello.txt")?;
    // error[E0277]: the `?` operator can only be used in a function that
    // returns `Result` or `Option` (or another type that implements `std::ops::Try`)
}

// The main function can *also* return a Result, in this case the `?` operator
// can be used
/*fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}*/
