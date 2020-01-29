fn main() {
    // The match Control Flow Operator

    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }

        println!("1 penny = {} cents", value_in_cents(Coin::Penny));
        println!("1 nickel = {} cents", value_in_cents(Coin::Nickel));
        println!("1 dime = {} cents", value_in_cents(Coin::Dime));
        println!("1 quarter = {} cents", value_in_cents(Coin::Quarter));
    }

    {
        // Patterns that Bind to Values

        #[derive(Debug)] // so we can inspect the state in a minute
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

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                },
            }
        }

        println!("1 penny = {} cents", value_in_cents(Coin::Penny));
        println!("1 nickel = {} cents", value_in_cents(Coin::Nickel));
        println!("1 dime = {} cents", value_in_cents(Coin::Dime));
        println!("1 quarter = {} cents", value_in_cents(Coin::Quarter(UsState::Alabama)));
        println!("1 quarter = {} cents", value_in_cents(Coin::Quarter(UsState::Alaska)));
    }

    // Matching with Option<T>

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Some(5) plus one = {:?}", six);
    println!("None plus one = {:?}", none);

    // Matches Are Exhaustive

    /*fn faulty_plus_one(x: Option<i32>) -> Option<i32> {
        match x { // error[E0004]: non-exhaustive patterns: `None` not covered
            Some(i) => Some(i + 1),
        }
    }*/

    // The _ Placeholder

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("One"),
        3 => println!("Three"),
        5 => println!("Five"),
        7 => println!("Seven"),
        _ => println!("Any other value"),
    }
}
