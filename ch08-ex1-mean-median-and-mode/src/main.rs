// Given a list of integers, use a vector and return the mean (the average
// value), median (when sorted, the value in the middle position), and mode (the
// value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let usage = format!("Usage: {} <n0> <n1> <n2> ...", args[0]);

    let count = args.len() - 1;
    if count == 0 {
        println!("Missing argument");
        println!("{}", usage);
        return;
    }

    let mut vector: Vec<u32> = Vec::new();
    let mut map: HashMap<u32, u32> = HashMap::new();
    let mut sum: u32 = 0;

    for arg in args[1..].iter() {
        let num: u32 = match arg.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong argument: {} is not an u32.", arg);
                println!("{}", usage);
                return;
            }
        };
        vector.push(num);
        let count = map.entry(num).or_insert(0);
        *count += 1;
        sum += num;
    }
    println!("as vector: {:?}", vector);
    println!("as hash map: {:?}", map);

    let mean = sum / count as u32;
    println!("mean: {}", mean);

    vector.sort();
    let median = vector[count / 2];
    println!("median: {}", median);

    let mut modes: Vec<u32> = Vec::new(); // Can be multimodal
    let mut highest: u32 = 0;
    for (key, value) in map {
        if value > highest {
            modes.clear();
            modes.push( key );
            highest = value;
        } else if value == highest {
            modes.push( key );
        }
    }
    println!("mode(s): {:?}", modes);
}
