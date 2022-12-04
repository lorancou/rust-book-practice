// Defining and Instantiating Structs
// https://doc.rust-lang.org/book/ch05-01-defining-structs.html

#![allow(unused_variables)]

fn main() {
    // Defining and Instantiating Structs

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("{}: {}", user1.username, user1.email);
    println!("  signed in {} times", user1.sign_in_count);
    println!("  is active: {}", user1.active);
    // user1.email = String::from("anotheremail@example.com"); // Nope, not mutable

    let mut user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("anotheremail@example.com"); // OK, is mutable
    println!("{}: {}", user1.username, user1.email);

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    let user1 = build_user(
        "someone@example.com".to_string(),
        "someusername123".to_string());
    println!("{}: {}", user1.username, user1.email);

    // Using the Field Init Shorthand when Variables and Fields Have the Same Name

    fn build_user_shorthand(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let user1 = build_user_shorthand(
        "someone@example.com".to_string(),
        "someusername123".to_string());
    println!("{}: {}", user1.username, user1.email);

    // Creating Instances From Other Instances With Struct Update Syntax

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("{}: {}", user2.username, user2.email);

    // Using Tuple Structs without Named Fields to Create Different Types

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Ownership of Struct Data

    /*struct BadUser {
        username: &str, error[E0106]: missing lifetime specifier
        email: &str,    error[E0106]: missing lifetime specifier
        sign_in_count: u64,
        active: bool,
    }*/
}

