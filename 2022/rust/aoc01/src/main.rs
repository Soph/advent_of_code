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
    let all = read_and_parse(&args.path);

    let mut results: Vec<i32> = all.iter().map(|elf| elf.iter().sum()).collect();
    results.sort();

    println!("Max: {}", results.last().unwrap());
    println!(
        "Last 3: {}",
        results.as_slice()[results.len() - 3..]
            .to_vec()
            .iter()
            .sum::<i32>()
    );
}

fn read_and_parse(path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n\n")
        .map(|s| s.split("\n").map(|l| l.parse().unwrap()).collect())
        .collect()
}
