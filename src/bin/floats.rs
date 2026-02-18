// Exploring IEEE 754 floating point numbers
//
// https://en.wikipedia.org/wiki/IEEE_754
//
// A floating-point format is specified by:
//
// * a base (also called radix) b, which is either 2 (binary) or 10 (decimal)
//   in IEEE 754;
// * a precision p;
// * an exponent range from emin to emax, with emin = 1 − emax for all IEEE 754
//   formats.
//
//A format comprises:
//
// * Finite numbers, which can be described by three integers:
//      s = a sign (zero or one),
//      c = a significand (or coefficient) having no more than p digits when
//          written in base b (i.e., an integer in the range through 0 to
//          bp−1), and
//      q = an exponent such that emin ≤ q+p−1 ≤ emax. The numerical value of
//          such a finite number is (−1)s × c × bq.[a] Moreover, there are two
//          zero values, called signed zeros: the sign bit specifies whether a
//          zero is +0 (positive zero) or −0 (negative zero).
// * Two infinities: +∞ and −∞.
// * Two kinds of NaN (not-a-number): a quiet NaN (qNaN) and a signaling NaN
//      (sNaN).
//
//  f32  base 2 digits 24  decimal  7  (7.22)  exp  8  38.23 E+127/E-126
//      24 bit precision + 8 bit exponent -> 1.123 456 7 E±38
//  f64  base 2 digits 53  decimal 16 (15.95)  exp 11 307.95 E+1023/E-1022
//      53 bit precision + 11 bit exponent -> 1.123 456 789 012 345 6 E±308
//

use std::f64::consts::PI;

fn main() {
    println!("Floating Point Number Experiments");

    let p1 = precision32();
    let p2 = precision64();
    let p3 = zero();

    equals_examples();

    println!("\nSUMMARY\n-------");
    println!("f32:  x == x - z -> 0 where z = 1/2^{}", p1);
    println!("f64:  x == x - z -> 0 where z = 1/2^{}", p2);
    println!("zero: {}", p3);
    // equals_examples doesn't return a value
}

// Find the smallest f32 number that when subtracted from 1.0 is equal to 1.0
// (floating point underflow)
fn precision32() -> usize {
    let x: f32 = 1.0;
    let mut z: f32 = 1.0;
    assert!(x == z);

    let mut i = 0;
    loop {
        i += 1;
        z = z / 2.0;
        let y = x - z;
        // f32 is good to 7 decimal places
        println!("{0}: {1:.8} == {2:.8} -> {3} ", i, x, y, x == y);
        if x == y {
            break;
        }
    }
    i
}

// Find the smallest f64 number that when subtracted from 1.0 is equal to 1.0
// (floating point underflow)
fn precision64() -> usize {
    let x: f64 = 1.0;
    let mut z = 1.0;
    assert!(x == z);

    let mut i = 0;
    loop {
        i += 1;
        z = z / 2.0;
        let y = x - z;
        // f64 is good to 16 decimal places
        println!("{0}: {1:.17} == {2:.17} -> {3} ", i, x, y, x == y);
        if x == y {
            break;
        }
    }
    i
}

// Tried to make -0.0
fn zero() -> bool {
    let x: f32 = 0.0;
    let y: f32 = -1.0 * x;
    println!("x {0:+.8}, y = {1:+.8}", x, y);
    x == y
}

// Compare two double precision floats for equality with the given variance
fn equals(x: f64, y: f64, e: f64) -> bool {
    x < y + e && x > y - e
}

// Examples of things what should be equal (and one case where they aren't!)
fn equals_examples() {
    let x = (PI / 2.0).sin();
    println!("sin(90°) = {0:+.16}", x);
    let y = 1.0 as f64;
    let e = 0.000000000000001 as f64;
    println!("{0:+.16} == {1:+.16} -> {2}", x, y, x == y);
    println!("{0:+.16} eq {1:+.16} -> {2}", x, y, equals(x, y, e));

    let three = 3.0 as f64;
    let x = (PI / 3.0).sin();
    let y = three.sqrt() / 2.0;

    println!("sin(60°) = {0:+.16}", x);
    println!("{0:+.16} == {1:+.16} -> {2}", x, y, x == y);

    let two = 2.0 as f64;
    let x = (PI / 4.0).sin();
    let y = 1.0 / two.sqrt();

    println!("sin(45°) = {0:+.16}", x);
    println!("{0:+.16} == {1:+.16} -> {2}", x, y, x == y);

    let x = (PI / 6.0).sin();
    let y = 0.5;

    println!("sin(30°) = {0:+.16}", x);
    println!("{0:+.16} == {1:+.16} -> {2}", x, y, x == y); // uh-oh!
    println!("{0:+.16} eq {1:+.16} -> {2}", x, y, equals(x, y, e));
}
