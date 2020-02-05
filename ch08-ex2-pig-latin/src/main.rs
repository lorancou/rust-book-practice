// Convert strings to pig latin. The first consonant of each word_chunk is moved to
// the end of the word_chunk and “ay” is added, so “first” becomes “irst-fay.” Words
// that start with a vowel have “hay” added to the end instead (“apple” becomes
// “apple-hay”). Keep in mind the details about UTF-8 encoding!

// cargo run -- "Enjoy, the first apple from a God!"
// Enjoy-hay, he-tay irst-fay apple-hay rom-fay a-hay Od-gay!

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

    let mut output = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        // Copy anything that's not a letter.
        if !c.is_ascii_alphabetic() {
            output.push(c);
            continue;
        }

        // Build a suffix depending of if we found a vowel or a consonant. If
        // that's a vowel, copy it. For word beginning with a capital consonant,
        // capitalizes the beginning of the "pig-latin word" instead.
        let lowercase_c = c.to_ascii_lowercase();
        let mut uppercase_consonant = false;
        let suffix = if let 'a' | 'e' | 'i' | 'o' | 'u' = lowercase_c {
            output.push(c);
            String::from("-hay")
        } else {
            uppercase_consonant = c != lowercase_c;
            format!("-{}ay", lowercase_c)
        };

        // Copy the rest of the word and apply the suffix when the next
        // character is not a letter.
        while let Some(&c) = chars.peek() {
            if !c.is_ascii_alphabetic() {
                break;
            }
            chars.next();
            if uppercase_consonant {
                output.push(c.to_ascii_uppercase());
                uppercase_consonant = false;
            } else {
                output.push(c);
            }
        }
        output += &suffix;
    }
    println!("{}", output);
}
