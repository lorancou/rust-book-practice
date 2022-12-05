// Improving Our I/O Project
// https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html

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
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}