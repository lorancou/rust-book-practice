// Closures: Anonymous Functions that Capture Their Environment
// https://doc.rust-lang.org/book/ch13-01-closures.html

use std::thread;
use std::time::Duration;

fn main() {

    // Capturing the Environment with Closures

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue,
    }
    
    struct Inventory {
        shirts: Vec<ShirtColor>,
    }
    
    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }
    
        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;
    
            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Closure Type Inference and Annotation

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    println!("result: {:?}", expensive_closure(3));

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    println!("add_one_v1: {:?}", add_one_v1(1));
    println!("add_one_v2: {:?}", add_one_v2(1));
    println!("add_one_v3: {:?}", add_one_v3(1));
    println!("add_one_v4: {:?}", add_one_v4(1));

    let example_closure = |x| x;

    let _s = example_closure(String::from("hello"));
    //let n = example_closure(5); // error[E0308]: mismatched types (return type is locked to String)

    // Capturing References or Moving Ownership

    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
    
        let only_borrows = || println!("From closure: {:?}", list);
    
        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }

    {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let mut borrows_mutably = || list.push(7);

        //println!("Before calling closure: {:?}", list); // error[E0502]: cannot borrow `list` as immutable because it is also borrowed as mutable
        borrows_mutably();
        println!("After calling closure: {:?}", list);    
    }

    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
    
        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();
        //println!("After calling closure: {:?}", list); // error[E0382]: borrow of moved value: `list`

    }

    // Moving Captured Values Out of Closures and the Fn Traits

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        _height: u32,
    }

    let mut list = [
        Rectangle { width: 10, _height: 1 },
        Rectangle { width: 3, _height: 5 },
        Rectangle { width: 7, _height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    /*let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(value); // error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
        r.width
    });
    println!("{:#?}", list);*/

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);    

}
