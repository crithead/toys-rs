// Common utility functions for the toys
use std::process::exit;

pub struct Options {}

impl Options {
    pub fn new() -> Self {
        Options {}
    }

    pub fn has(&self, _s: &str) -> bool {
        false
    }
}

/// Convert &str to i32 or exit the program
pub fn str_to_int(s: &str) -> i32 {
    match s.parse::<i32>() {
        Ok(value) => value,
        Err(_) => {
            println!("invalid number: {}", s);
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Options, str_to_int};

    #[test]
    fn new_empty() {
        let opts = Options::new();
        assert_eq!(opts.has("-a"), false);
    }

    #[test]
    fn str_to_int_1() {
        assert_eq!(str_to_int("227"), 227);
        assert_eq!(str_to_int("0"), 0);
        assert_eq!(str_to_int("-11"), -11);
    }
}
