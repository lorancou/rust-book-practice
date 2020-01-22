fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    // Mutable References

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}",s);

    let _r1 = &mut s;
    let _r2 = &mut s; 
    //println!("{}, {}", r1, r2); // [E0499]: cannot borrow `s` as mutable more than once at a time

    let _r1 = &s; // no problem
    let _r2 = &s; // no problem
    let _r3 = &mut s; // BIG PROBLEM
    //println!("{}, {}, and {}", r1, r2, r3); // [E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Dangling References

    //let reference_to_nothing = dangle();
    let reference_to_something = no_dangle();
    println!("{}", reference_to_something);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // Needs a mutable reference to be allowed
}

/*
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}