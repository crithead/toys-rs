//! A005132
//! Recamán's sequence (or Recaman's sequence):
//! a(0) = 0; for n > 0, a(n) = a(n-1) - n if nonnegative and not already
//! in the sequence, otherwise a(n) = a(n-1) + n.

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

/// If MAX_N is greater than PROGRESS_N, print a dot '.' after every
/// PROGRESS_N numbers in the sequence are calculated.
const PROGRESS_N: usize = 100000;

/// Write the sequence to this file.
const OUTPUT_FILE: &str = "A005132.txt";

/// Calculate this many term of the sequence, if not specified.
const DEFAULT_N: usize = 100;

/// Calculate N term of the Recaman Sequence, print them to the console and
/// write them to a file.
fn main() {
    let mut sequence = Vec::new();
    calculate(&mut sequence, max_n());
    print_sequence(&sequence);
    write_sequence(&sequence, OUTPUT_FILE);
}

/// Calculate N terms of the Recamán Sequence.
fn calculate(seq: &mut Vec<usize>, max_n: usize) {
    let mut a = 0;
    seq.push(a);
    for n in 1..max_n + 1 {
        if a <= n {
            // 'a - n' is not greater than zero, use 'a + n'
            a += n;
        } else if let Some(_) = seq.iter().position(|&x| x == a - n) {
            // 'a - n' is already in the sequence, use 'a + n'
            a += n;
        } else {
            // 'a - n' is positive and not already in the sequence,
            // use 'a - n'
            a -= n;
        }

        seq.push(a);

        if max_n > PROGRESS_N && n % PROGRESS_N == 0 {
            print!(".");
            io::stdout().flush().unwrap();
        }
    }
}

/// Get the number of terms to generate from the command line or the default.
fn max_n() -> usize {
    match env::args().skip(1).next() {
        Some(s) => match s.parse::<usize>() {
            Ok(n) => n,
            Err(_) => DEFAULT_N,
        },
        None => DEFAULT_N,
    }
}

/// Print the sequence
fn print_sequence(seq: &Vec<usize>) {
    for (i, n) in seq.iter().enumerate() {
        print!("{:8}", n);
        if i % 8 == 7 {
            println!();
        }
    }
    println!();
}

/// Write the sequence to a file.
fn write_sequence(seq: &Vec<usize>, file_name: &str) {
    let path = Path::new(file_name);
    let mut file = match File::create(&path) {
        Err(why) => {
            eprintln!("write_sequence: {}", why);
            return;
        }
        Ok(file) => file,
    };

    println!(
        "Writing {} terms of the Recaman sequence to '{}'",
        seq.len(),
        file_name
    );
    for n in seq {
        match writeln!(file, "{}", n) {
            Err(why) => eprintln!("write_sequence: {}", why),
            Ok(_) => {}
        }
    }
}
