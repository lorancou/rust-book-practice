// How to Write Tests
// https://doc.rust-lang.org/book/ch11-01-writing-tests.html

#![allow(dead_code)]

#[cfg(test)]
mod tests {

    // The Anatomy of a Test Function

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    /*#[test]
    fn another() {
        panic!("Make this test fail");
    }*/

    // Checking Results with the assert! Macro

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(!smaller.can_hold(&larger));
    }

    // Testing Equality with the assert_eq! and assert_ne! Macros

    fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // Adding Custom Failure Messages

    fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    // Checking for Panics with should_panic

    struct Guess {
        value: i32,
    }

    impl Guess {
        fn new(value: i32) -> Guess {
            if value < 1 {
                panic!("Guess value must be greater than or equal to 1, got {}.",
                       value);
            } else if value > 100 {
                panic!("Guess value must be less than or equal to 100, got {}.",
                       value);
            }
            Guess {
                value
            }
        }
    }

    // Chech for *any* panic
    #[test]
    #[should_panic]
    fn lower_than_1() {
        Guess::new(0);
    }

    // Chech for panic including a specific string
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Using Result<T, E> in Tests

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
