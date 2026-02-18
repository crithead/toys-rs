//! 'Range' examples
//! See: std::ops::Range et al.

fn main() {
    example1();
    example2();
}

/// Count up.
fn example1() {
    print!("0..10:\t");
    for i in 0..10 {
        print!("{i:4}");
    }
    println!("");

    print!("0..=10:\t");
    for i in 0..=10 {
        print!("{i:4}");
    }
    println!("");

    print!("-10..=-1:\t");
    for i in -10..=-1 {
        print!("{i:4}");
    }
    println!("");

    print!("-5..=5:\t");
    for i in -5..=5 {
        print!("{i:4}");
    }
    println!("");
}

/// Conut down.
/// Range expressions only count up.
/// To count down, use '.rev()' to reverse the order.
fn example2() {
    print!("(0..10).rev():\t");
    for i in (0..10).rev() {
        print!("{i:4}");
    }
    println!("");

    print!("(0..=10).rev():\t");
    for i in (0..=10).rev() {
        print!("{i:4}");
    }
    println!("");

    print!("(-5..=5).rev():\t");
    for i in (-5..=5).rev() {
        print!("{i:4}");
    }
    println!("");
}
