// Using a hash map and vectors, create a text interface to allow a user to add
// employee names to a department in a company. For example, “Add Sally to
// Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all
// people in a department or all people in the company by department, sorted
// alphabetically.

use std::collections::HashMap;
use std::io::{self, Write};

fn help() {
    println!( "AVAILABLE COMMANDS" );
    println!( "ADD <NAME> <DEPT>" );
    println!( "LIST [DEPT]" );
    println!( "HELP" );
    println!( "EXIT" );
}

fn main() {
    println!("WELCOME TO THE DUNDER MIFFLIN HR DATABASE");

    let mut employees = HashMap::new();

    loop {
        println!("PLEASE ENTER YOUR COMMAND");

        print!("DM-HRDB> ");
        io::stdout().flush()
            .expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        input.make_ascii_uppercase();

        let args: Vec<&str> = input.split_whitespace().collect();
        let argc = args.len();

        if argc != 0 {
            match args[0] {
                "ADD" => {
                    if argc == 3 {
                        let name = args[1].to_string();
                        let dept = args[2].to_string();
                        let names = employees.entry(dept).or_insert(Vec::new());
                        names.push(name);
                        names.sort();
                        println!("SUCCESS! ADDED {} TO {}", args[1], args[2]);
                    } else {
                        println!("ERROR! WRONG ARGUMENT COUNT");
                        help();
                    }
                },
                "LIST" => {
                    if argc == 2 {
                        let dept = args[1].to_string();
                        if let Some(names) = employees.get(&dept) {
                            println!("{:?}", names);
                        } else {
                            println!("ERROR! UNKNOWN DEPT");
                            help();
                        }
                    } else if argc == 1 {
                        // Not sure if that really matches the intructions
                        // (should it be a list of names sorted *globally*?).
                        // Let's say it does.
                        println!("{:?}", employees);
                    } else {
                        println!("ERROR! WRONG ARGUMENT COUNT");
                        help();
                    }
                }
                "HELP" => help(),
                "EXIT" => break,
                _ =>  {
                    println!( "ERROR! UNKWOWN COMMAND" );
                    help();
                },
            }
        } else {
            println!("ERROR! MISSING COMMAND");
            help();
        }
    }

    println!("THANKS FOR YOUR VISIT");
}
