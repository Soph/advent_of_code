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
    let fishes = read_and_parse(&args.path);

    calc1(&fishes, 80);
    calc2(&fishes, 80);
    calc2(&fishes, 256);
}

fn read_and_parse(path: &str) -> Vec<i64> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split(",")
        .map(ToOwned::to_owned)
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn calc1(fishes: &Vec<i64>, runtime: i64) {
    let mut data = fishes.clone();

    for _ in 0..runtime {
        let mut new_fish = vec![];
        for x in 0..data.len() {
            let new = data[x] - 1;
            match new {
                -1 => {
                    data[x] = 6;
                    new_fish.push(8);
                }
                _ => data[x] = data[x] - 1,
            }
        }
        data.append(&mut new_fish);
    }

    println!("{}", data.len());
}

fn calc2(fishes: &Vec<i64>, runtime: i64) {
    let mut data: HashMap<i64, i64> = HashMap::new();
    for fish in fishes {
        *data.entry(*fish).or_insert(0) += 1;
    }

    for _ in 0..runtime {
        let mut new_data: HashMap<i64, i64> = HashMap::new();
        for (age, count) in data {
            let new_age = age - 1;
            match new_age {
                -1 => {
                    *new_data.entry(6).or_insert(0) += count;
                    new_data.insert(8, count);
                }
                _ => {
                    *new_data.entry(new_age).or_insert(0) += count;
                }
            }
        }
        data = new_data;
    }

    let mut sum = 0;
    for (_, count) in data.iter() {
        sum += count;
    }
    println!("{}: {}", runtime, sum);
}
