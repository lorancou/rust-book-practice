use std::env;

fn to_celsius(fahrenheit : f64) -> f64 {
    (fahrenheit - 32.0) * (5.0/9.0)
}

fn to_fahrenheit(celsius : f64) -> f64 {
    (celsius * (9.0/5.0)) + 32.0
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Wrong argument count.");
        println!("Usage: degrees <n> <°C|°F>");
        return;
    }

    let n: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("First argument must be a number.");
            println!("Usage: degrees <n> <°C|°F>");
            return;
        }
    };

    match args[2].as_str() {
        "°C" => println!("{} °C = {} °F", n, to_fahrenheit(n)),
        "°F" => println!("{} °F = {} °C", n, to_celsius(n)),
        _ => {
            println!("Second argument must be \"°C\" or \"°F\".");
            println!("Usage: degrees <n> <°C|°F>");
        },
    }
}
