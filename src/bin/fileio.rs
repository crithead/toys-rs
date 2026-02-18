// Read text data from a file

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static DATA1: &str = "data/data1.csv";
static DATA2: &str = "data/data2.csv";
static DATA3: &str = "data/data3.csv"; // does not exist

/// Run the example functions.
fn main() {
    println!("--- fileio");
    open_file(DATA1);
    open_file(DATA2);
    open_file(DATA3);
    csv_example();
}

fn csv_example() {
    println!("--- fileio: CSV");
    //open_file(DATA1);
    //open_file(DATA3);
    create_file("poem.txt");
    open_file("poem.txt");
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
fn open_file(filename: &str) {
    println!("--- open_file");

    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => eprintln!("Couldn't read {}: {}", display, why.to_string()),
        Ok(_) => print!("--- {} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

static POEM: &str = "Mary had a little lamb
One day she saw it sicken
They shipped it off to the packing plant
Now they call it chicken
";

// https://doc.rust-lang.org/rust-by-example/std_misc/file/create.html
fn create_file(filename: &str) {
    let path = Path::new(filename);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    // Write the `POEM` string to `file`, returns `io::Result<()>`
    match file.write_all(POEM.as_bytes()) {
        Err(why) => {
            panic!("Couldn't write to {}: {}", display, why.to_string())
        }
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}
