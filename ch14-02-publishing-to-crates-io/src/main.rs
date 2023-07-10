// Publishing a Crate to Crates.io
// https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html

//use rust_book_publish::kinds::PrimaryColor;
//use rust_book_publish::utils::mix;

// Using re-exports:
use rust_book_publish::mix;
use rust_book_publish::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
