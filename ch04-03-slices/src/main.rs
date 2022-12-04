// The Slice Type
// https://doc.rust-lang.org/book/ch04-03-slices.html

#![allow(unused_variables)]

fn main() {
    // The Slice Type

    let mut s = String::from("hello world");

    fn first_word_as_index(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let word = first_word_as_index(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // String Slices

    let s = String::from("hello world");

    let hello = &s[0..5];
    let hello = &s[..5]; // same

    let len = s.len();
    let world = &s[6..len];
    let world = &s[6..]; // same

    println!("{} {}", hello, world);

    fn first_word_as_slice(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let word = first_word_as_slice(&s);

    println!("the first word is: {}", word);

    //s.clear(); // error!
    //println!("the first word is: {}", word); // [E0596]: cannot borrow `s` as mutable, as it is not declared as mutable

    // String Slices as Parameters

    let my_string = String::from("hello world");

    fn first_word_slice_to_slice(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    // first_word works on slices of `String`s
    let word = first_word_slice_to_slice(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word_slice_to_slice(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice_to_slice(my_string_literal);

    // Other Slices

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}
