// Examples of pattern matching in Rust
//

struct Item<'a> {
    key: i32,
    value: &'a str,
}

fn main() {
    const N: i32 = 42;

    match_boolean(N);
    match_boolean(N + 1);
    match_enum(N);
    match_enum(-N);
    match_letter(N);
    match_letter(N / 2);
    match_number(0);
    match_number(2);
    match_number(8);
    match_number(16);
    match_struct(&Item {
        key: 0,
        value: "nil",
    });
    match_struct(&Item {
        key: 42,
        value: "The Answer",
    });
    match_struct(&Item {
        key: 9,
        value: "Number Nine",
    });
    match_tuple(0, 0);
    match_tuple(0, 1);
    match_tuple(1, 0);
    match_tuple(1, 1);
    match_tuple(-1, -1);
}

fn match_boolean(n: i32) {
    let is_even = if n % 2 == 0 { true } else { false };
    match is_even {
        true => println!("{} is even", n),
        false => println!("{} is not even", n),
    }
}

#[derive(Debug)]
enum Sign {
    Negative,
    Zero,
    Positive,
}

fn match_enum(n: i32) {
    let sign = if n > 0 {
        Sign::Positive
    } else if n < 0 {
        Sign::Negative
    } else {
        Sign::Zero
    };
    match sign {
        Sign::Negative => println!("{} is {:?}", n, sign),
        Sign::Zero => println!("{} is {:?}", n, sign),
        Sign::Positive => println!("{} is {:?}", n, sign),
    }
}

fn match_letter(n: i32) {
    let c = if (n % 52) >= 26 {
        ('A' as u8 + (n % 26) as u8) as char
    } else {
        ('a' as u8 + (n % 26) as u8) as char
    };

    // '..=' is inclusive range
    match c {
        'a'..='z' => println!("{} is lower case", c),
        'A'..='Z' => println!("{} is upper case", c),
        _ => panic!("invalid code path"),
    }
}

fn match_number(n: i32) {
    match n {
        0 => println!("N is zero"),
        1 | 2 | 3 => println!("N is small"),
        4..=12 => println!("N is smallish"),
        _ => println!("N is not small enough"),
    }
}

fn match_struct(item: &Item) {
    match item {
        Item { key: 0, .. } => println!("Key is Zero"),
        _ => println!("key {} -> value {}", item.key, item.value),
    }
}

fn sign_of(n: i32) -> Sign {
    if n < 0 {
        Sign::Negative
    } else if n > 0 {
        Sign::Positive
    } else {
        Sign::Zero
    }
}

fn match_tuple(x: i32, y: i32) {
    let x_sign = sign_of(x);
    let y_sign = sign_of(y);

    print!("({}, {}) is ", x, y);
    match (x_sign, y_sign) {
        (Sign::Zero, Sign::Zero) => println!(" Origin"),
        (Sign::Zero, _) => println!(" on Y-Axis"),
        (_, Sign::Zero) => println!(" on X-Axis"),
        (Sign::Positive, Sign::Positive) => println!(" in Quadrant I"),
        (Sign::Negative, Sign::Positive) => println!(" in Quadrant II"),
        (Sign::Negative, Sign::Negative) => println!(" in Quadant III"),
        (Sign::Positive, Sign::Negative) => println!(" in Quadrant IV"),
    }
}
