use std::collections::HashMap;
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
    let mut numbers = read_and_parse(&args.path);
    numbers.sort();

    let differences = find_differences(&numbers);
    println!(
        "Differences: 1=>{}, 3=>{}",
        differences[&1],
        differences[&3] + 1
    );
    println!("Part1: {}", (differences[&1] * (differences[&3] + 1)));

    numbers.push(numbers.last().unwrap() + 3);
    let chunks = find_chunks(&numbers);
    println!("Chunks:");
    for (k, v) in &chunks {
        println!("{}: {}", k, v);
    }
    // A chunk of 5 has 7 different variants
    // A chunk of 4 has 4 different variants
    // A chunk of 3 has 2 different variants
    let value = 7_u64.pow(chunks[&5]) * 4_u64.pow(chunks[&4]) * 2_u64.pow(chunks[&3]);
    println!("Part2: {}", value);
}

fn read_and_parse(path: &str) -> Vec<u32> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

fn find_differences(numbers: &Vec<u32>) -> HashMap<u32, u32> {
    let mut result: HashMap<u32, u32> = HashMap::new();

    let mut last_joltage = 0;
    for number in numbers {
        let diff = number - last_joltage;
        *result.entry(diff).or_insert(0) += 1;
        last_joltage = *number;
    }

    result
}

fn find_chunks(numbers: &Vec<u32>) -> HashMap<u32, u32> {
    let mut chunk_size = 1;
    let mut last_number = 0;
    let mut result: HashMap<u32, u32> = HashMap::new();

    for number in numbers {
        if number - last_number == 1 {
            chunk_size += 1
        } else {
            *result.entry(chunk_size).or_insert(0) += 1;
            chunk_size = 1;
        }
        last_number = *number;
    }

    result
}
