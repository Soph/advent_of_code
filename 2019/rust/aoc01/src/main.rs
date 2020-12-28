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

    let numbers = read_and_parse(&args.path);

    calculate_1(&numbers);
    calculate_2(&numbers);
}

fn read_and_parse(path: &str) -> Vec<i64> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

fn calculate_1(numbers: &Vec<i64>) {
    let mut sum = 0;
    for number in numbers {
        sum += number / 3 - 2;
    }

    println!("Result1: {}", sum);
}

fn calculate_2(numbers: &Vec<i64>) {
    let mut sum = 0;
    for number in numbers {
        let mut current = *number;
        let mut number_sum = 0;
        loop {
            let result = current / 3 - 2;
            if result < 0 {
                break;
            }
            current = result;
            number_sum += result;
        }
        sum += number_sum;
    }

    println!("Result2: {}", sum);
}
