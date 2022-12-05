// An I/O Project: Building a Command Line Program
// https://doc.rust-lang.org/book/ch12-00-an-io-project.html

// Test commands:
// > cargo run frog poem.txt
// > cargo run body poem.txt
// > cargo run monomorphization poem.txt
// > cargo run to poem.txt
// > cargo run to poem.txt --i

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}