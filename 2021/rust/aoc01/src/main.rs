use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

fn main() {
    let args = Cli::from_args();
    let depths = read_and_parse(&args.path);

    sweep1(&depths);
    sweep2(depths);
}

fn read_and_parse(path: &str) -> Vec<i32> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let numbers = contents
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    numbers
}

fn sweep1(depths: &Vec<i32>) {
    let mut last = depths[0];
    let mut increases = 0;

    for depth in depths {
        if *depth > last {
            increases += 1;
        }
        last = *depth;
    }

    println!("Found {} simple increases", increases);
}

fn sweep2(depths: Vec<i32>) {
    let mut increases = 0;
    let mut window_a = depths[0]+depths[1]+depths[2];
    let mut window_b;

    for n in 1..depths.len() {
        window_b = depths[n] + depths[n+1] + depths[n+2];
        if window_a < window_b {
            increases += 1;
        }
        window_a = window_b;
        if n + 4 > depths.len() {
            break;
        }
    }

    println!("Found {} window increases", increases);
}