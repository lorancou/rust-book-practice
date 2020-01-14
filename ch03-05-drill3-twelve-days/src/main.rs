fn main() {
    // Lyrics taken from https://en.wikipedia.org/wiki/The_Twelve_Days_of_Christmas_(song)

    const DAY_COUNT : usize = 12;

    const ORDINALS : [&str; DAY_COUNT] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eigth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    const PRESENTS : [&str; DAY_COUNT] = [
        "And a partridge in a pear tree.",
        "Two turtle doves,",
        "Three french hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Sever swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ];

    for i in 0..DAY_COUNT {
        println!("On the {} day of Christmas my true love sent to me", ORDINALS[i]);
        if i == 0 {
            // Not sure if there's a more elegant way to handle that exception
            println!("A partridge in a pear tree.");
        } else {
            for j in (0..i+1).rev() {
                println!("{}", PRESENTS[j]);
            }
        }
        println!("");
    }
}
