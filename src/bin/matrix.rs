//
// An exercise in creating fixed size two-dimensional arrays.
//
// See also: (crates) ndarray, multiarray
//

use std::convert::TryInto;

fn main() {
    matrix_ops();
}

fn matrix_ops() {
    let mut a = [[0u8; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let v = (i + 1) * (j + 1);
            a[i][j] = v.try_into().unwrap();
        }
    }
    mprint("A", &a);

    let mut b = [[0u8; 3]; 3];
    mtimes(12, &a, &mut b);
    mprint("12 x A", &b);
}

// Print a 3x3 matrix
fn mprint(name: &str, m: &[[u8; 3]; 3]) {
    println!("Matrix {} = ", name);
    for i in 0..3 {
        for j in 0..3 {
            print!("\t{}", m[i][j]);
        }
        print!("\n");
    }
}

// Muliply a matrix by a scalar
// n * M -> B
fn mtimes(n: u8, m: &[[u8; 3]; 3], b: &mut [[u8; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            b[i][j] = n * m[i][j];
        }
    }
}
