// Defining an Enum
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html

#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    // Enum Values

    {
        enum IpAddrKind {
            V4,
            V6,
        }

        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        fn route(ip_kind: IpAddrKind) {}

        route(four);
        route(six);

        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }

    {
        // Data attached to each variant directly

        enum IpAddr {
            V4(String),
            V6(String),
        }

        let home = IpAddr::V4(String::from("127.0.0.1"));

        let loopback = IpAddr::V6(String::from("::1"));
    }

    {
        // Different types of data per variant

        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IpAddr::V4(127, 0, 0, 1);

        let loopback = IpAddr::V6(String::from("::1"));
    }

    {
        // How the standard library does it

        struct Ipv4Addr {
            // --snip--
        }

        struct Ipv6Addr {
            // --snip--
        }

        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
    }

    {
        // Another example

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        impl Message {
            fn call(&self) {
                // method body would be defined here
            }
        }

        let m = Message::Write(String::from("hello"));
        m.call(); 
    }

    // The Option Enum and Its Advantages Over Null Values

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; // Can't infer type from None, has to be specified

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; error[E0277]: cannot add `std::option::Option<i8>` to `i8`
}
