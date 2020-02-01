// Generate the nth Fibonacci number.

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Wrong argument count.");
        println!("Usage: fibonacci <n>");
        return;
    }

    let n: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not an u32.");
            println!("Usage: fibonacci <n>");
            return;
        }
    };

    let mut pair = (0, 1);
    for _ in 0..n {
        pair = (pair.1, pair.0+pair.1);
    }
    println!("Fibonacci: {}", pair.0);
}