//! Collatz Conjecture
//! Pick a number
//! if it is EVEN divide by two
//! if it is ODD multiply by three and add one
//!
//! Start with any positive integer n. Then each term is obtained from the
//! previous term as follows: if the previous term is even, the next term is
//! one half of the previous term. If the previous term is odd, the next term
//! is 3 times the previous term plus 1. The conjecture is that no matter what
//! value of n, the sequence will always reach 1.

use std::env;

const A: usize = 2;
const B: usize = 1000000;

/// Collatz conjecture
fn collatz(starting_value: usize) -> usize {
    let mut steps = 0;
    let mut n = starting_value;

    while n != 1 {
        steps += 1;
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
    }
    steps
}

/// Get a range of numbers to compute the Collaz sequence from the
/// command line.
/// Returns a tuple of (min, max)
fn get_range() -> (usize, usize) {
    let mut a = 0;
    let mut b = 0;

    let mut args = env::args();
    let _ = args.next(); // discard program name
    if let Some(s) = args.next() {
        a = match s.parse::<usize>() {
            Ok(n) => n,
            Err(_) => 0,
        }
    };
    if let Some(s) = args.next() {
        b = match s.parse::<usize>() {
            Ok(n) => n,
            Err(_) => 0,
        }
    };

    if a == 0 && b == 0 {
        // neither supplied, use defaults
        (A, B)
    } else if b == 0 {
        // only one, use as upper bound
        if a < A {
            a = A;
        }
        (A, a)
    } else {
        // use both
        if a < A {
            a = A;
        }
        if b < a {
            b = a;
        }
        (a, b)
    }
}

/// Print the number of steps to 1 for each number in the range.
fn main() {
    let (a, b) = get_range();
    println!("    Number -> Steps to one");
    println!("    ----------------------");
    for n in a..b + 1 {
        let steps = collatz(n);
        println!("{:10} -> {}", n, steps);
    }
}
