// Validating References with Lifetimes
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;

fn main() {

    // Preventing Dangling References with Lifetimes

    /*
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
    // -> error[E0597]: `x` does not live long enough
    */

    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
    // -> OK

    // Generic Lifetimes in Functions

    /*
    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd"); // String
    let string2 = "xyz"; // string literal

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    
    // -> error[E0106]: missing lifetime specifier
    */

    // Lifetime Annotation Syntax

    /*
    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime
    */

    // Lifetime Annotations in Function Signatures

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd"); // String
    let string2 = "xyz"; // string literal

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // ----

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // ----

    /*
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    // -> error[E0597]: `string2` does not live long enough
    */

    // Thinking in Terms of Lifetimes

    fn return_first<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }
    // -> OK, no reference to y returned
    // Allows this: https://stackoverflow.com/a/31609892/1005455

    /*
    fn return_long_string<'a>(x: &str, y: &str) -> &'a str {
        let result = String::from("really long string");
        result.as_str()
    }
    // -> error[E0515]: cannot return value referencing local variable `result`
    */

    // Lifetime Annotations in Struct Definitions

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    // Lifetime Ellision

    /*
    // Signature:
    fn first_word(s: &str) -> &str {

    // After rule 1:
    fn first_word<'a>(s: &'a str) -> &str {

    // After rule 2:
    fn first_word<'a>(s: &'a str) -> &'a str {

    // -> OK, compiles
    */

    // ----

    /*
    // Signature:
    fn longest(x: &str, y: &str) -> &str {

    // After rule 1:
    fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {

    // Rule 2 and 3 don't apply
    // -> Doesn't compile
    */

    // Lifetime Annotations in Method Definitions

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // The Static Lifetime

    let s: &'static str = "I have a static lifetime.";

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where T: Display
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd"); // String
    let string2 = "xyz"; // string literal

    let result = longest_with_an_announcement(string1.as_str(), string2, 42);
    println!("The longest string is {}", result);
}
