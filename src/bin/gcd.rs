// GCD example from Programming Rust, p.12-13

use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        // expect() -- Unwraps a result, yielding the content of an Ok.
        // Panics if the value is an Err, with a panic message including
        // the passed message, and the content of the Err.
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        // If a returned Result is Ok, unwrap() returns the value.
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd_1() {
    assert_eq!(gcd(14, 15), 1);
}

#[test]
fn test_gcd_2() {
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

#[test]
#[should_panic]
fn test_gcd_3() {
    assert_eq!(gcd(0, 15), 1);
}

#[test]
#[should_panic]
fn test_gcd_4() {
    assert_eq!(gcd(10, 0), 1);
}
