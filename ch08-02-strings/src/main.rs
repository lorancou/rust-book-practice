// Storing UTF-8 Encoded Text with Strings
// https://doc.rust-lang.org/book/ch08-02-strings.html

#![allow(unused_mut)]
#![allow(unused_variables)]

fn main() {
    // Creating a New String

    let mut s = String::new();

    let data = "initial contents"; // String literal
    let s = data.to_string(); // Any type that implements the Display trait has a to_string method, including string literals

    let s = "initial contents".to_string(); // The method also works on a literal directly

    let s = String::from("initial contents"); // Same thing as with to_string

    let mut hellos = Vec::new();
    hellos.push(String::from("السلام عليكم"));
    hellos.push(String::from("Dobrý den"));
    hellos.push(String::from("Hello"));
    hellos.push(String::from("שָׁלוֹם"));
    hellos.push(String::from("नमस्ते"));
    hellos.push(String::from("こんにちは"));
    hellos.push(String::from("안녕하세요"));
    hellos.push(String::from("你好"));
    hellos.push(String::from("Olá"));
    hellos.push(String::from("Здравствуйте"));
    hellos.push(String::from("Hola"));
    for hello in hellos {
        println!("{}", hello);
    }

    // Updating a String

    // Appending to a String with push_str and push

    let mut s = String::from("foo");
    s.push_str("bar"); // Push a string (slice)
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // Doesn't take ownership of s2
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l'); // Push a single character
    println!("{}", s);

    // Concatenation with the + Operator or the format! Macro

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // println!("{}", s1); // error[E0382]: borrow of moved value: `s1`
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3; // A little unwieldy, what's moved already?
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // OK, that's better. No ownership taken.
    println!("{}", s);

    // Indexing into Strings

    let s1 = String::from("hello");
    // let h = s1[0]; // error[E0277]: the type `std::string::String` cannot be indexed by `{integer}`

    // Internal Representation

    let len = String::from("Hola").len();
    println!("len is {}", len); // 4

    let len = String::from("Здравствуйте").len();
    println!("len is {}", len); // 24: the length in bytes, not the character count

    // Slicing Strings

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // let s = &hello[0..1]; // thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`'

    // Methods for Iterating Over Strings

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
