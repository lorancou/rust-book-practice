// Cargo Workspaces
// https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

#![allow(unused_imports)]

use add_one;
use add_two;
use rand;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
    println!("Furthermore, {num} plus two is {}!", add_two::add_two(num));
}
