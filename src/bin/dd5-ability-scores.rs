//! Dungeons & Dragons Ability Scores
// cargo run --bin dd5-ability-scores > /tmp/scores.txt
// dd5-ability-scores -h | [ -v ] [ -p N ] [-s STR]

extern crate clap;

use clap::{App, Arg};
use std::cmp::Ordering;

const VERSION: &str = "1.1";

struct Options {
    average: bool,
    help: bool,
    points: u8,
    separator: String,
    verbose: bool,
}

/// Given an Ability Score, return the cost to buy it.
fn cost(score: u8) -> u8 {
    match score {
        8 => 0,
        9 => 1,
        10 => 2,
        11 => 3,
        12 => 4,
        13 => 5,
        14 => 7,
        15 => 9,
        _ => panic!("Score out of range"),
    }
}

/// Generate a list of Ability Score Sets with a given total cost.
/// Returns a vector of 6-element arrays with elements sorted descending.
fn generate_ability_score_sets(opts: &Options) -> Vec<[u8; 6]> {
    let mut sets = Vec::<[u8; 6]>::new();

    if opts.points == 0 {
        // Generate the default sets
        sets.push([15, 15, 15, 8, 8, 8]);
        sets.push([13, 13, 13, 12, 12, 12]);
    } else {
        for a in 8..=15 {
            for b in 8..=15 {
                for c in 8..=15 {
                    for d in 8..=15 {
                        for e in 8..=15 {
                            for f in 8..=15 {
                                let mut scores = [a, b, c, d, e, f];
                                if total_cost(&scores) == opts.points {
                                    scores.sort_by(|a, b| b.cmp(a));
                                    sets.push(scores)
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    sets
}

/// Get command line options.
///     -a      Average (statistics)
///     -h      Print usage and exit
///     -p N    Points total
///     -s STR  Field separator string
///     -v      Enebale versbose output
fn get_options() -> Options {
    let matches = App::new("D&D 5e Ability Score Set Calculator")
        .version(VERSION)
        .arg(
            Arg::with_name("points")
                .short("p")
                .long("points")
                .value_name("POINTS")
                .help("Points available to buy Ability Scores")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("separator")
                .short("s")
                .long("separator")
                .value_name("SEPARATOR")
                .help("Output field separator")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("average")
                .short("a")
                .long("average")
                .help("Include average in output"),
        )
        .arg(
            Arg::with_name("help")
                .short("h")
                .long("help")
                .help("Print usage information and exit"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Enable extra messages"),
        )
        .get_matches();

    let average = matches.is_present("average");
    let help = matches.is_present("help");
    let verbose = matches.is_present("verbose");

    let points = matches
        .value_of("points")
        .unwrap_or("27")
        .parse::<u8>()
        .unwrap();

    let mut separator =
        matches.value_of("separator").unwrap_or(", ").to_string();
    if separator == "TAB" || separator == "\\t" {
        separator = "\t".to_string();
    }

    Options {
        average,
        help,
        points,
        separator,
        verbose,
    }
}

/// Print all ability score sets with a given total number of points.
fn main() {
    let opts = get_options();

    if opts.verbose {
        println!("average  : {}", opts.average);
        println!("help     : {}", opts.help);
        println!("points   : {}", opts.points);
        println!("separtor : '{}'", opts.separator);
        println!("verbose  : {}", opts.verbose);
    }

    if opts.help {
        print_help();
        return;
    }

    if opts.verbose {
        println!("Verbose mode on");
    }

    let mut sets = generate_ability_score_sets(&opts);

    // Sort descending (operators are reversed)
    sets.sort_by(|a, b| {
        for i in 0..6 {
            if a[i] > b[i] {
                return Ordering::Less;
            } else if a[i] < b[i] {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    });

    sets.dedup();
    print_ability_score_sets(&sets, &opts.separator, opts.average);

    if opts.verbose {
        println!("Point cost: {}", opts.points);
        println!("Number of sets: {}", sets.len());
    }
}

/// Print Ability Score Sets to the console.
fn print_ability_score_sets(
    sets: &Vec<[u8; 6]>, separator: &str, average: bool,
) {
    for set in sets.iter() {
        print!(
            "{}{}{}{}{}{}{}{}{}{}{}",
            set[0],
            separator,
            set[1],
            separator,
            set[2],
            separator,
            set[3],
            separator,
            set[4],
            separator,
            set[5]
        );
        if average {
            let sum: u8 = set.iter().sum();
            let mean = sum as f32 / 6.0;
            println!("{}{:.3}", separator, mean);
        } else {
            println!("");
        }
    }
}

/// Print a usage message.
fn print_help() {
    println!("Dungeons & Dragons 5e - Ability Scores");
    println!("Print a list of ability score sets that total N points.\n");
    println!("    -a, --average     Include average in output");
    println!("    -h, --help        Print usage and exit");
    println!("    -p N, --points N  Points total (default = 27)");
    println!("    -s STR, --separator STR");
    println!("                      Field separator string");
    println!("    -v, --verbose     Enable verbose output");
    println!("    -V                Print version information and exit\n");
}

/// Calculate the total cost of an Ability Score set.
fn total_cost(scores: &[u8; 6]) -> u8 {
    let mut total_cost = 0;
    for i in 0..=5 {
        total_cost += cost(scores[i]);
    }
    total_cost
}
