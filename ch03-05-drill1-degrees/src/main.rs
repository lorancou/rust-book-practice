use std::io;

// https://www.rapidtables.com/convert/temperature/fahrenheit-to-celsius.html
fn to_celsius(fahrenheit : f64) -> f64 {
    (fahrenheit - 32.0) * (5.0/9.0)
}

fn main() {
    let mut fahrenheit = String::new();

    println!("Enter a temperature in Fahrenheit:");

    io::stdin().read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f64 = fahrenheit.trim().parse()
    	.expect("Not a number");

    println!("In Celsius: {}", to_celsius(fahrenheit));
}
