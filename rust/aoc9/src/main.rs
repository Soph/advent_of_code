use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
    preamble_length: usize,
}

fn main() {
    let args = Cli::from_args();
    let numbers = read_and_parse(&args.path);
    let invalid = find_invalid(&numbers, args.preamble_length);

    println!("Invalid: {}", invalid);

    let mut contiguous_range = find_contiguous_range(&numbers, invalid);
    contiguous_range.sort();

    println!(
        "Sum of two largest numbers: {} + {} = {}",
        contiguous_range[contiguous_range.len() - 1],
        contiguous_range[0],
        contiguous_range[contiguous_range.len() - 1] + contiguous_range[0]
    );
}

fn read_and_parse(path: &str) -> Vec<u64> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

fn find_invalid(numbers: &Vec<u64>, preamble_length: usize) -> u64 {
    for i in preamble_length..numbers.len() {
        let mut preamble = numbers[i - preamble_length..i].to_vec();
        preamble.sort();
        if !valid_subtrahend_exists(&preamble, numbers[i]) {
            return numbers[i] as u64;
        }
    }

    0
}

fn valid_subtrahend_exists(preamble: &Vec<u64>, minuend: u64) -> bool {
    for n in preamble {
        for i in preamble {
            if n + i == minuend {
                return true;
            }
        }
    }

    false
}

fn find_contiguous_range(numbers: &Vec<u64>, invalid_number: u64) -> Vec<u64> {
    for i in 0..numbers.len() {
        let mut sum = 0;
        for n in i..numbers.len() {
            sum += numbers[n];
            if sum > invalid_number {
                continue;
            } else if sum == invalid_number {
                println!("Contiguous Range: {}..{}", i, n);
                return numbers[i..n].to_vec();
            }
        }
    }

    return vec![];
}
