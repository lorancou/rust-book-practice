// Convert strings to pig latin. The first consonant of each word_chunk is moved to
// the end of the word_chunk and “ay” is added, so “first” becomes “irst-fay.” Words
// that start with a vowel have “hay” added to the end instead (“apple” becomes
// “apple-hay”). Keep in mind the details about UTF-8 encoding!

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!("Usage: {} <input>", args[0]);

    if args.len() != 2 {
        println!("Wrong argument count.");
        println!("{}", usage);
        return;
    }

    let input: String = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a String.");
            println!("{}", usage);
            return;
        }
    };

    // My first attempt, lots of bools and ifs and elses
    // "The first apple of Brian."
    // -> He-tay irst-fay apple-hay of-hay Rian-bay.
    let mut output = String::new();
    let mut word_chunk = String::new();
    let mut first_consonant: Option<char> = None;
    let mut next_uppercase = false;
    let mut found_letter = false;
    for c in input.chars() {
        if c.is_alphabetic() {
            if !found_letter {
                let lowercase_c = c.to_ascii_lowercase();
                if let 'a' | 'e' | 'i' | 'o' | 'u' = lowercase_c {
                    first_consonant = None;
                    word_chunk.push(c);
                } else {
                    if c == lowercase_c {
                        first_consonant = Some(c);
                    } else {
                        next_uppercase = true;
                        first_consonant = Some(lowercase_c);
                    }
                }
                found_letter = true;
            } else if next_uppercase {
                word_chunk.push(c.to_ascii_uppercase());
                next_uppercase = false;
            } else {
                word_chunk.push(c);
            }
        } else {
            if found_letter {
                if let Some(f) = first_consonant {
                    let new_word = format!("{}-{}ay", word_chunk, f);
                    output += &new_word;
                } else {
                    let new_word = format!("{}-hay", word_chunk);
                    output += &new_word;
                }
            }
            output.push(c);
            word_chunk.clear();
            first_consonant = None;
            next_uppercase = false;
            found_letter = false;
        }
    }
    println!("{}", output);

    // Found online, way shorter and cleaner, but warns at compilation and doesn't handle case
    // https://codereview.stackexchange.com/questions/172866/pig-latin-exercise-in-rust/172910
    // "The first apple of Brian."
    // -> he-Tay irst-fay apple-hay of-hay rian-Bay.
    let mut chars = input.chars().peekable();
    let mut output = String::new();
    while let Some(c) = chars.next() {
        let suffix = match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                output.push(c);
                String::from("-hay")
            }
            'a'...'z' | 'A'...'Z' => {
                format!("-{}ay", c)
            }
            _ => {
                output.push(c);
                continue;
            }
        };

        while let Some(&c) = chars.peek() {
            match c {
                'a'...'z' | 'A'...'Z' => {
                    chars.next();
                    output.push(c);
                }
                _ => break,
            }
        }

        output += &suffix;
    }
    println!("{}", output);
}


