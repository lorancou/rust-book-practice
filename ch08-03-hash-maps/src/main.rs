#![allow(unused_variables)]

use std::collections::HashMap;

fn main() {
    // Creating a New Hash Map

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Creating a hash map from a list of teams and a list of scores
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); // HashMap<_, _> needed since the collection type could be another one

    // Hash Maps and Ownership

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    //println!("{} is {}", field_name, field_value); // error[E0382]: borrow of moved value: `field_name` & `field_value`

    // Accessing Values in a Hash Map

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{}'s score is {:?}", team_name, score);
    for (key, value) in &scores {
        println!("{}'s score is {:?}", key, value);
    }

    // Updating a Hash Map

    // Overwriting a Value

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Only Inserting a Value If the Key Has No Value

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Updating a Value Based on the Old Value

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert retunrs a mutable reference to the value
        *count += 1; // The value is incremented here
    }
    println!("{:?}", map);
}
