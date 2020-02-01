// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
// taking advantage of the repetition in the song.

// Lyrics taken from:
// https://en.wikipedia.org/wiki/The_Twelve_Days_of_Christmas_(song)

fn main() {

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

    // Day 1
    println!("On the {} day of Christmas my true love sent to me", ORDINALS[0]);
    println!("A partridge in a pear tree."); // That one's without the "and"
    println!("");

    // Days 2+
    for i in 1..DAY_COUNT {
        println!("On the {} day of Christmas my true love sent to me", ORDINALS[i]);
        for j in (0..i+1).rev() {
            println!("{}", PRESENTS[j]);
        }
        println!("");
    }
}
