//! Convert decimal number into Roman Numerals.

use std::env;

fn main() {
    for arg in env::args().skip(1) {
        let d = arg.parse::<u32>().unwrap_or(0);
        let r = to_roman(d);
        println!("{}\t{}", d, r);
    }
}

fn to_roman(number: u32) -> String {
    let mut string: String = String::new();
    let mut number = number;
    while number > 0 {
        if number >= 1000 {
            number -= 1000;
            string.push('M');
        } else if number >= 900 {
            number -= 900;
            string.push('C');
            string.push('M');
        } else if number >= 500 {
            number -= 500;
            string.push('D');
        } else if number >= 400 {
            number -= 400;
            string.push('C');
            string.push('D');
        } else if number >= 100 {
            number -= 100;
            string.push('C');
        } else if number >= 50 {
            number -= 50;
            string.push('L');
        } else if number >= 40 {
            number -= 40;
            string.push('X');
            string.push('L');
        } else if number >= 10 {
            number -= 10;
            string.push('X');
        } else if number >= 9 {
            number -= 9;
            string.push('I');
            string.push('X');
        } else if number >= 5 {
            number -= 5;
            string.push('V');
        } else if number >= 4 {
            number -= 4;
            string.push('I');
            string.push('V');
        } else if number >= 1 {
            number -= 1;
            string.push('I');
        }
    }
    string
}
