/// Examples of how to use Result return types.
///

/// Use functions that return a result or a static error string.
fn example_1() {
    let a = 6789;
    let b = 45;

    match multiply(a, b) {
        Ok(c) => println!("{a} * {b} = {c}"),
        Err(msg) => println!("Error: {a} × {b} = {msg}"),
    }

    let a = (isize::MAX / 3) * 2;
    let b = (isize::MAX / 4) * 3;

    match multiply(a, b) {
        Ok(c) => println!("{a} * {b} = {c}"),
        Err(msg) => println!("Error: {a} × {b} = {msg}"),
    }

    let a = 6789;
    let b = 45;

    match divide(a, b) {
        Ok(c) => println!("{a} / {b} = {c}"),
        Err(msg) => println!("Error: {a} ÷ {b} = {msg}"),
    }

    let b = 0;
    match divide(a, b) {
        Ok(c) => println!("{a} / {b} = {c}"),
        Err(msg) => println!("Error: {a} ÷ {b} = {msg}"),
    }

    let b = -13;
    match divide(a, b) {
        Ok(c) => println!("{a} / {b} = {c}"),
        Err(msg) => println!("Error: {a} ÷ {b} = {msg}"),
    }
}

fn multiply(a: isize, b: isize) -> Result<isize, &'static str> {
    if a > isize::MAX / b {
        Err("overflow")
    } else {
        Ok(a * b)
    }
}

fn divide(a: isize, b: isize) -> Result<isize, &'static str> {
    if b == 0 {
        Err("divide by zero")
    } else {
        Ok(a / b)
    }
}

fn main() {
    example_1();
}
