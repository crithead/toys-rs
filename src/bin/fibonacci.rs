// Print the fibonacci sequence

/// Print the first 'n' term, if specified, otherwise print all terms that
/// fit into a u64.
use toys::str_to_int;

/// The maximum number of terms where the value fits in a u64
const MAX_TERMS: usize = 94;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        fib(MAX_TERMS);
    } else {
        let n = str_to_int(&args[1]) as usize;
        fib(n);
    }
}

/// Print the first 'n' terms of the fibonaccci sequence
fn fib(n_terms: usize) {
    let mut a0 = 0;
    let mut a1 = 1;
    let mut a2;
    let mut n = n_terms;

    println!("{}", a0);
    println!("{}", a1);

    if n > MAX_TERMS {
        n = MAX_TERMS;
    }

    for _i in 0..n - 2 {
        a2 = next_term(a1, a0);
        println!("{}", a2);
        a0 = a1;
        a1 = a2
    }
}

/// Given terms Xn-1 and Xn-2, return Xn
fn next_term(x0: u64, x1: u64) -> u64 {
    x1 + x0
}
