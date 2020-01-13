fn main() {
    // Day 1
    println!("On the first day of Christmas my true love sent to me");
    println!("A partridge in a pear tree.");
    println!("");

    const DAY_COUNT : usize = 12;

    // Not sure how to make that const (is that even possible?)
    let numerals = [
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
    assert_eq!(numerals.len(), DAY_COUNT-1);
    
    // Same
    let presents = [
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
    assert_eq!(presents.len(), DAY_COUNT);

    // Days 2+
    for i in 0..DAY_COUNT-1 {
        println!("On the {} day of Christmas my true love sent to me", numerals[i]);
        for j in (0..i+2).rev() {
            println!("{}", presents[j]);
        }
        println!("");
    }
}
