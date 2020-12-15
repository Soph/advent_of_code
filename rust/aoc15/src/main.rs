use std::collections::HashMap;
use std::collections::VecDeque;
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
    let data = read_and_parse(&args.path);

    let result1 = run(&data, 2020);
    println!("Result for 2020th - {}", result1);

    let result2 = run(&data, 30000000);
    println!("Result for 30000000th - {}", result2);
}

fn read_and_parse(path: &str) -> Vec<u64> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

fn run(data: &Vec<u64>, nth: u64) -> u64 {
    let mut used: HashMap<u64, VecDeque<u64>> = HashMap::new();
    let mut i = 0;
    let mut current_value = 0;

    for value in data {
        used.entry(*value)
            .or_insert(VecDeque::with_capacity(2))
            .push_front(i);
        current_value = *value;
        i += 1;
    }

    loop {
        let current_history = used
            .entry(current_value)
            .or_insert(VecDeque::with_capacity(2));
        match current_history.back() {
            Some(x) => {
                if current_history.len() == 1 && *x == i - 1 {
                    // was used right before
                    current_value = 0;
                } else {
                    match current_history.get(0) {
                        Some(y) => {
                            // println!("History for {}: {},{}", current_value, x, y);
                            current_value = y - x;
                        }
                        None => {
                            println!("No second value in History, should not happen!?");
                        }
                    }
                }
                if i == nth - 1 {
                    return current_value;
                }
                let new_history = used
                    .entry(current_value)
                    .or_insert(VecDeque::with_capacity(2));
                new_history.push_front(i);
                new_history.truncate(2);
                i += 1;
            }
            None => {
                println!("Not in History, should not happen!?");
            }
        }
    }
}
