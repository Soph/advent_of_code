use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

struct Movement {
    direction: String,
    amount: i32,
}

fn main() {
    let args = Cli::from_args();
    let movements = read_and_parse(&args.path);

    move1(&movements);
    move2(&movements);
}

// Parsing
fn read_and_parse(path: &str) -> Vec<Movement> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.split("\n").map(|s| parse_movements(&s)).collect()
}

fn parse_movements(line: &str) -> Movement {
    let parts: Vec<String> = line.split(" ").map(ToOwned::to_owned).collect();

    Movement {
        direction: parts[0].clone(),
        amount: parts[1].parse().unwrap(),
    }
}

fn move1(movements: &Vec<Movement>) {
    let mut vertical = 0;
    let mut horizontal = 0;

    for movement in movements {
        match movement.direction.as_str() {
            "up" => vertical = vertical - movement.amount,
            "down" => vertical = vertical + movement.amount,
            "forward" => horizontal = horizontal + movement.amount,
            _ => (),
        }
    }

    println!(
        "Found {}, {} -> {}",
        horizontal,
        vertical,
        horizontal * vertical
    );
}

fn move2(movements: &Vec<Movement>) {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    for movement in movements {
        match movement.direction.as_str() {
            "up" => aim = aim - movement.amount,
            "down" => aim = aim + movement.amount,
            "forward" => {
                horizontal = horizontal + movement.amount;
                depth = depth + aim * movement.amount;
            }
            _ => (),
        }
    }

    println!(
        "Found {}, {}, {} -> {}",
        horizontal,
        depth,
        aim,
        horizontal * depth
    );
}
