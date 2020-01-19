fn main() {
    { // s is not valid here, it’s not yet declared
        let s = "hello, world!"; // s is valid from this point forward
        println!("{}", s); // This will print `hello, world!`
        // do more stuff with s
    } // this scope is now over, and s is no longer valid


    {
        let mut s = String::from("hello");
        s.push_str(", world!"); // push_str() appends a literal to a String
        println!("{}", s); // This will print `hello, world!`
        // do more stuff with s
    } // this scope is now over, and s is no longer valid (drop occured)

    {
        //let s1 = String::from("hello");
        //let s2 = s1; // = shallow copy
        //println!("{}, world!", s1); // won't compile, s1 is invalidated

        let s1 = String::from("hello");
        let _s2 = s1.clone(); // clone = deep copy
        println!("{}, world!", s1); // OK, s1 still valid, will print `hello, world!`
    }

    {
        let s = String::from("hello"); // s comes into scope
        takes_ownership(s); // s's value moves into the function...
        // ... and so is no longer valid here

        let x = 42; // x comes into scope
        makes_copy(x); // x would move into the function,
        // but i32 is Copy, so it’s okay to still use x afterward

    } // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

    {
        let _s1 = gives_ownership(); // gives_ownership moves its return value into s1
        let _s2 = String::from("hello"); // s2 comes into scope
        let _s3 = takes_and_gives_back(_s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    } // Here, _s3 goes out of scope and is dropped. _s2 goes out of scope but was moved, so nothing happens. _s1 goes out of scope and is dropped.

    {
        // What we'd have to do if there was no references in Rust
        let s1 = String::from("hello, world!");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}, world!", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}