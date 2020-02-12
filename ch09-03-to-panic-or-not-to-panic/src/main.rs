// https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html

#![allow(unused_variables)]

use std::io;

fn main() {
    // To panic! or Not to panic!

    // Cases in Which You Have More Information Than the Compiler

    use std::net::IpAddr;
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    // Creating Custom Types for Validation

    loop {
        // Without a custom type

        println!("Please input your guess. [logs and asks again if out of range]");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The guess must be between 1 and 100, got {}.", guess);
            continue;
        }

        println!("Ok, guess is between 1 and 100: {}", guess);

        // -- snip --
        break;
    }

    loop {
        // With a custom Guess type

        pub struct Guess {
            value: i32,
        }

        impl Guess {
            pub fn new(value: i32) -> Guess {
                if value < 1 || value > 100 {
                    panic!("Guess value must be between 1 and 100, got {}.", value);
                }

                Guess {
                    value
                }
            }

            pub fn value(&self) -> i32 {
                self.value
            }
        }

        println!("Please input your guess. [panics if out of range]");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess); // Panics if out of range

        println!("Ok, guess is between 1 and 100: {}", guess.value());

        // -- snip --
        break;
    }
}
