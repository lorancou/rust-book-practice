// Concise Control Flow with if let
// https://doc.rust-lang.org/book/ch06-03-if-let.html

#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

fn main() {

    {
        let some_u8_value = Some(3u8);

        // Match with only one pattern, ignores the rest
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }

        // Concise version with if let
        if let Some(3) = some_u8_value {
            println!("three");
        }

        // Yes but? Regular if works fine too
        if Some(3) == some_u8_value {
            println!("three");
        }
    }

    {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
            // --snip--
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        let coin = Coin::Quarter(UsState::Alabama);
        let mut count = 0;
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,
        }

        // Here if let lets us extract the inner state value, couldn't do this with a regular if
        let coin = Coin::Quarter(UsState::Alaska);
        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }
    }
}