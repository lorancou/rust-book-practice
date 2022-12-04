// Traits: Defining Shared Behavior
// https://doc.rust-lang.org/book/ch10-02-traits.html

#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Debug;
use std::fmt::Display;

fn main() {

    // Implementing a Trait on a Type

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    {
        // Defining a Trait

        pub trait Summary {
            fn summarize(&self) -> String;
            // -- possibly, more method signatures --
        }

        // Implementing a Trait on a Type

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        println!("1 new tweet: {}", tweet.summarize());
        println!("1 new article: {}", article.summarize());
    }

    // Default Implementations
    {
        pub trait Summary {
            fn summarize(&self) -> String {
                String::from("(Read more...)")
            }
        }

        impl Summary for NewsArticle {}

        println!("1 new article: {}", article.summarize());
    }
    {
        pub trait Summary {
            fn summarize_author(&self) -> String;

            fn summarize(&self) -> String {
                format!("(Read more from {}...)", self.summarize_author())
            }
        }

        impl Summary for Tweet {
            fn summarize_author(&self) -> String {
                format!("@{}", self.username)
            }
        }

        println!("1 new tweet: {}", tweet.summarize());

        // Traits as Parameters

        impl Summary for NewsArticle {
            fn summarize_author(&self) -> String {
                format!("{}", self.author)
            }
        }

        pub fn notify(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }

        notify(&article);

        // Trait Bound Syntax

        pub fn notify_2<T: Summary>(item: &T) {
            println!("Breaking news! {}", item.summarize());
        }

        notify_2(&article);

        // Here item1 and item2 can be of different types
        pub fn notify_3(item1: impl Summary, item2: impl Summary) {}

        // Trait bound allows to constrain them to be of the same type
        pub fn notify_4<T: Summary>(item1: T, item2: T) {}

        // Specifying Multiple Trait Bounds with the + Syntax

        pub fn notify_5(item: impl Summary + Display) {}

        // Same with trait Bounds
        pub fn notify_6<T: Summary + Display>(item: T) {}

        // Clearer Trait Bounds with where Clauses

        fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
            // ...
            let result = 0;
            // ...
            result
        }

        fn some_function_2<T, U>(t: T, u: U) -> i32
            where T: Display + Clone,
                  U: Clone + Debug
        {
            let result = 0;
            // ...
            result
        }

        // Returning Types that Implement Traits

        fn returns_summarizable() -> impl Summary {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }

        // Can't return different types
        /*fn returns_summarizable_2(switch: bool) -> impl Summary {
            if switch {
                NewsArticle {
                    headline: String::from("Penguins win the Stanley Cup Championship!"),
                    location: String::from("Pittsburgh, PA, USA"),
                    author: String::from("Iceburgh"),
                    content: String::from("The Pittsburgh Penguins once again are the best
                    hockey team in the NHL."),
                }
            } else {
                Tweet {
                    username: String::from("horse_ebooks"),
                    content: String::from("of course, as you probably already know, people"),
                    reply: false,
                    retweet: false,
                }
            }
        }*/
        // error[E0308]: if and else have incompatible types
    }

    // Fixing the largest Function with Trait Bounds

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // Clone variant
    fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
        let mut largest = list[0].clone();

        for item in list.iter() {
            if item > &largest {
                largest = item.clone();
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_clone(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_clone(&char_list);
    println!("The largest char is {}", result);

    // Returning a reference variant
    fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_ref(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_ref(&char_list);
    println!("The largest char is {}", result);

    // Using Trait Bounds to Conditionally Implement Methods

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    let pair = Pair{ x: 49, y: 3 };
    pair.cmp_display();
}
