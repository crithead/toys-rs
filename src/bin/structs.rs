// An example of a struct holding a shared mutable reference to another struct.

use std::cell::RefCell;
use std::rc::Rc;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

struct Place {
    name: String,
    point: Rc<Point>,
}

impl Place {
    fn new(name: String, point: Rc<Point>) -> Self {
        Place { name, point }
    }

    fn print(&self) {
        let x = self.point.x;
        let y = self.point.y;
        println!("{} ({}, {})", self.name, x, y)
    }
}

// Two structs hold a reference to a piece of shared data (the point).
fn example1() {
    println!("--- Example 1");

    let a;
    let b;
    {
        // Create some data stored on the heap (immutable)
        let shared_point: Rc<Point> = Rc::new(Point::new(33, 44));
        // Pass a refernce to it to one object
        a = Place::new("Arizona".to_string(), shared_point.clone());
        // Pass a refernce to it to another object
        b = Place::new("Bethesda".to_string(), shared_point.clone());
        // shared_object goes out of scope here and is dropped
    }
    // A & B both have a refernce to the shared data
    a.print();
    b.print();
}

// ---------------------------------------------------------------------------

#[derive(PartialEq)]
enum LogLevel {
    Error,
    Notice,
}

struct Logger {
    count: u32,
    level: LogLevel,
}

impl Logger {
    fn new(level: LogLevel) -> Self {
        Logger { count: 0, level }
    }

    //    fn level_string(&self) -> &str {
    //        match self.level {
    //            LogLevel::Error => "ERROR",
    //            LogLevel::Notice => "NOTICE",
    //        }
    //    }

    fn error(&mut self, message: &str) {
        if self.level == LogLevel::Error || self.level == LogLevel::Notice {
            self.count += 1;
            println!("{} ERROR {}", self.count, message);
        }
    }
    fn notice(&mut self, message: &str) {
        if self.level == LogLevel::Notice {
            self.count += 1;
            println!("{} NOTICE {}", self.count, message);
        }
    }
}

struct User {
    // This struct contains an Rc smart pointer (for sharing) to a
    // RefCell (for mutability) containing a Logger instance (for logging).
    logger: Rc<RefCell<Logger>>,
    name: String,
}

impl User {
    fn new(logger: Rc<RefCell<Logger>>, name: String) -> Self {
        User { logger, name }
    }

    fn h2(&self, a: i32, b: i32) -> i32 {
        // Get a reference to the Logger
        let mut log = self.logger.borrow_mut();

        if a > 1000 || a < -1000 {
            log.error(&format!("{}: 'a' too large", self.name));
            return 0;
        }
        if b > 1000 || b < -1000 {
            log.error(&format!("{}: 'b' too large", self.name));
            return 0;
        }

        // Use the Logger
        log.notice(&format!("{}: 'a' and 'b' are just right", self.name));
        a * a + b * b
    }
}

fn example2() {
    println!("--- Example 2");

    let mut log = Logger::new(LogLevel::Notice);
    log.error("One little");
    log.notice("Two little");
    log.error("Three little");
    log.notice("Log messages");

    log.level = LogLevel::Error;
    log.error("One more");
    log.notice("Two more");
    log.error("Three more");
    log.notice("Log messages");

    // Create a Logger wraped in a RefCell wrapped in an Rc
    let logger = Rc::new(RefCell::new(Logger::new(LogLevel::Error)));
    // Pass a copy of the Rc to the User
    let user = User::new(logger.clone(), "Albert".to_string());
    // Use the User
    let value = user.h2(3, 4);
    println!("h^2 = {}", value);
    let value = user.h2(6666, 8888);
    println!("h^2 = {}", value);
}

// ---------------------------------------------------------------------------

fn main() {
    example1();
    example2();
}
