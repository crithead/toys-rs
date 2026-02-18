//! Miscellaneous mathementical computations

fn main() {
    println!("Mathematical computations");
    sums_and_squares();
    sums_and_squares_v2();
}

/// In which sum of squares and square of sums is explored.
fn sums_and_squares() {
    let a = [1, 2, 3, 4, 5, 6];
    let n = 6;

    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in &a {
        sum1 += i;
        sum2 += i * i;
    }

    println!("A = {:?}", a);
    println!("Square of Sum of A = {}", sum1 + sum1);
    println!("Sum of A^2 = {}", sum2);

    let mut sum3 = 0f32;
    let mut sum4 = 0f32;
    for i in &a {
        sum3 += *i as f32 / n as f32;
        sum4 += (*i as f32 / n as f32) * (*i as f32 / n as f32);
    }

    println!("Square of Sum of 1/A = {}", sum3 + sum3);
    println!("Sum of A^-2 = {}", sum4);
}

/// In which sum of squares and square of sums is explored.
fn sums_and_squares_v2() {
    let a = [1, 2, 3, 4, 5, 6];
    // B is an array of "A[i]^2" -> [1, 4, 9, 16, 25, 36]
    let mut b: [i8; 6] = [0; 6]; // = a.iter().map(|x| x * x);
    for i in 0..a.len() {
        b[i] = a[i] * a[i];
    }
    println!("A = {:?}", a);
    println!("B = {:?}", b);

    let sum_a: i8 = a.iter().sum();
    let sum_b: i8 = b.iter().sum();
    println!("Sum of A = {}", sum_a);
    println!("Sum of B = {}", sum_b);
}
