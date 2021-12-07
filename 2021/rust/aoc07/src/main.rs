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
    let crabs = read_and_parse(&args.path);

    calc1(&crabs);
    calc2(&crabs);
}

fn read_and_parse(path: &str) -> Vec<i64> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split(",")
        .map(ToOwned::to_owned)
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn calc1(crabs: &Vec<i64>) {
    let data = crabs.clone();
    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();
    let mut min_sum = 0;

    for i in min..=max {
        let mut sum = 0;
        for j in data.iter() {
            sum += (i - j).abs();
        }
        if sum < min_sum || min_sum == 0 {
            min_sum = sum;
        }
    }

    println!("{}", min_sum);
}

fn calc2(crabs: &Vec<i64>) {
    let data = crabs.clone();
    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();
    let mut min_sum = 0;

    for i in min..=max {
        let mut sum = 0;
        for j in data.iter() {
            sum += (i - j).abs()*((i - j).abs()+1)/2;
        }
        if sum < min_sum || min_sum == 0 {
            min_sum = sum;
        }
    }

    println!("{}", min_sum);
}
