// This utility works like the Python range function in that it
// produces a string of space separated numbers on stdout from
// START (inclusive) to STOP (exclusive).

use std::process::exit;
use toys::str_to_int;

fn main() {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        if args[1] == "-h" || args[1] == "--help" {
            print_help();
            exit(0);
        } else {
            a = 0;
            b = str_to_int(&args[1]);
            c = 1;
        }
    } else if args.len() == 3 {
        a = str_to_int(&args[1]);
        b = str_to_int(&args[2]);
        c = 1;
    } else if args.len() >= 4 {
        a = str_to_int(&args[1]);
        b = str_to_int(&args[2]);
        c = str_to_int(&args[3]);
    }

    if c == 0 {
        c = 1;
    }

    if c > 0 {
        //for (int i = a; i < b; i += c) {
        for i in (a..b).step_by(c as usize) {
            print!("{}", i);
            if i < b - c {
                print!(" ");
            }
        }
    } else {
        // c < 0
        // TODO Fix the case where it counts backwards
        //for (int i = a; i > b; i += c) {
        for i in (a..b).step_by(c as usize) {
            print!("{}", i);
            if i > b + c {
                print!(" ");
            }
        }
    }
}

fn print_help() {
    println!("range - print a sequence of numbers\n");
    println!("    range [START] STOP [INCR]\n");
    println!("    START   The first value to print (default: 0)");
    println!("    STOP    The number after the last value to print");
    println!("    INCR    The distance between numbers (default: 1)\n");
    println!("EXAMPLES\n");
    println!("    range 10");
    println!("0 1 2 3 4 5 6 7 8 9\n");
    println!("    range 10 15");
    println!("10 11 12 13 14\n");
    println!("    range 3 10 3");
    println!("3 6 9\n");
}

//static void print_help(void)
//{
//    cout << "Print a space-separated list of numbers from A (inclusive)" << endl;
//    cout << "to B (exclusive) at intervals of C to the standard output." << endl;
//    cout << endl;
//    cout << "    range B | A B | A B C | --help | -h" << endl;
//    cout << endl;
//    cout << "    A     start of range (default = 0)" << endl;
//    cout << "    B     end of range (default = 1)" << endl;
//    cout << "    C     increment (default = 1)" << endl;
//    cout << "   -h     print a help message and exit" << endl;
//    cout << endl;
//    cout << "It is like the Python range function for bash for loops." << endl;
//    cout << "i.e. for i in $(range A B C); do ... done" << endl;
//    cout << endl;
//}
