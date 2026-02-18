//  An exercise in Rust String objects.
//
//  Some examples from:
//      https://doc.rust-lang.org/std/string/struct.String.html

use std::fmt::Write;

/// Build strings by appending new letters to the back.
fn example1() {
    // Create a mutable String object from a literal string.
    let mut s = String::from("cheeseburger");
    // and append characters to it
    s.push(' ');
    s.push('a');
    s.push('n');
    s.push('d');
    s.push(' ');
    // Append a &str
    s.push_str("fries");
    println!("{}", s);

    // Create another String object from a literal string.
    let mut s = "hamburger".to_string();
    s += " ";
    s += "with";
    s += " ";
    s += "cheese";
    println!("{}", s);
}

/// Strings are UTF-8
fn example2() {
    // Build a String from a vector of bytes.
    // Here are some bytes, in a vector
    let sparkle_heart = vec![240, 159, 146, 150];

    // We know these bytes are valid, so we'll use `unwrap()`.
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

    assert_eq!("💖", sparkle_heart);
    println!("sparkle_heart {}", sparkle_heart);

    // A second example (this is just ASCII)
    let vector = vec![83, 112, 97, 114, 107, 108, 101, 121, 33];
    let string = String::from_utf8(vector).unwrap();
    println!("string '{}'", string);

    // Strings are always valid UTF-8.
    // Strings cannot be indexed into, but can be iterated over.
    // The opposite of from_utf8 is as_bytes.
}

/// Split a string.
fn example3() {
    let csv = String::from("12\t34\t56");
    println!("csv '{}'", csv);
    // String::split(ch) returns an iterator
    for i in csv.split('\t') {
        println!("{}", i);
    }
}

/// Pass a String to a function and return a String from a function.
fn example4() {
    let s1 = "This is a String.".to_string();
    let s2 = reverse(&s1);
    println!("{}\n{}", s1, s2);
}

/// Returns a new String with the letters in the reverse order.
fn reverse(s: &String) -> String {
    let mut r = String::with_capacity(s.len());
    // Get an iterator over the characters in the String and reverse it.
    for c in s.chars().rev() {
        r.push(c);
    }
    r
}

/// String implements the Write trait.
fn example5() {
    let mut s = String::new();
    // Call unwrap the Result since String::Write won't fail
    // so there's no risk of panic
    write!(s, "{} + {} = {}", 1, 2, 5).unwrap();
    println!("String '{}'", s);
}

fn main() {
    println!("--- Rust Strings");
    example1();
    example2();
    example3();
    example4();
    example5();
}
