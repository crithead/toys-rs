//! Print a histogram of dX+dY results.
use std::env;
use std::process;

fn main() {
    let (x, y) = get_pair();

    let mut sums = vec![0; x + y + 1];
    for i in 1..=x {
        for j in 1..=y {
            sums[i + j] += 1;
        }
    }

    println!("d{x} + d{y}");
    sums.iter()
        .enumerate()
        .skip(2)
        .for_each(|(i, x)| println!("{i}: {x}"));
}

fn get_pair() -> (usize, usize) {
    let x;
    let y;
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        y = args.pop().unwrap().parse().unwrap();
        x = args.pop().unwrap().parse().unwrap();
    } else {
        println!("Error: Two arguments requied");
        println!("Usage: {} X Y", args[1]);
        process::exit(1);
    }
    (x, y)
}
