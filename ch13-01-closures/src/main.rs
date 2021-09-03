#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

// Creating an Abstraction of Behavior with Closures

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

// Refactoring Using Functions

fn generate_workout_2(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

// Refactoring with Closures to Store Code

fn generate_workout_3(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

// Closure Type Inference and Annotation

// let example_closure = |x| x;

// let s = example_closure(String::from("hello"));
// let n = example_closure(5); // error[E0308]: mismatched types

// Storing Closures Using Generic Parameters and the Fn Traits

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout_4(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10; // 50
    let simulated_random_number = 7; // 3

    println!("==== generate_workout ====");
    generate_workout(simulated_user_specified_value, simulated_random_number);

    println!("=== generate_workout_2 ===");
    generate_workout_2(simulated_user_specified_value, simulated_random_number);

    println!("=== generate_workout_3 ===");
    generate_workout_3(simulated_user_specified_value, simulated_random_number);

    println!("=== generate_workout_4 ===");
    generate_workout_4(simulated_user_specified_value, simulated_random_number);
}

// Limitations of the Cacher Implementation

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

// Try modifying Cacher to hold a hash map rather than a single value. The keys of the hash map
// will be the arg values that are passed in, and the values of the hash map will be the result of
// calling the closure on that key. Instead of looking at whether self.value directly has a Some or
// a None value, the value function will look up the arg in the hash map and return the value if
// it’s present. If it’s not present, the Cacher will call the closure and save the resulting value
// in the hash map associated with its arg value.

struct Cacher2<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher2<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher2<T> {
        Cacher2 {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        *self.values.entry(arg).or_insert((self.calculation)(arg))
    }
}

#[test]
fn call_with_different_values_2() {
    let mut c = Cacher2::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

// The second problem with the current Cacher implementation is that it only accepts closures that
// take one parameter of type u32 and return a u32. We might want to cache the results of closures
// that take a string slice and return usize values, for example. To fix this issue, try
// introducing more generic parameters to increase the flexibility of the Cacher functionality.

struct Cacher3<Calc, In, Out>
where
    Calc: Fn(&In) -> Out,
{
    calculation: Calc,
    values: HashMap<In, Out>,
}

impl<Calc, In, Out> Cacher3<Calc, In, Out>
where
    Calc: Fn(&In) -> Out,
    In: Eq + Hash,
    Out: Copy,
{
    fn new(calculation: Calc) -> Cacher3<Calc, In, Out> {
        Cacher3 {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: In) -> Out {
        // Not sure why this doesn't compile...
        // *self.values.entry(arg).or_insert((self.calculation)(&arg))

        // ... but this does? It's more readable anyway
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(&arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values_3() {
    let mut c = Cacher3::new(|a: &String| a.len());

    let v1 = c.value(String::from("foo"));
    let v2 = c.value(String::from("bar_baz"));

    assert_eq!(v2, 7);

    // Doesn't work with string slices... I'm not sure why as well :/
    // let mut c = Cacher3::new(|a: &str| a.len());
}

// Capturing the Environment with Closures

#[test]
fn example_closure_capture() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}

/*
#[test]
fn example_function_capture() { // Does not compile: "error[E0434]: can't capture dynamic environment in a fn item"
    let x = 4;

    fn equal_to_x(z: i32) -> bool {
        z == x
    }

    let y = 4;

    assert!(equal_to_x(y));
}
*/

#[test]
fn example_closure_move() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x); // Does not compile, x was moved

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
