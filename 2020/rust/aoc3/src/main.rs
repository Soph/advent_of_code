use std::fs;
use structopt::StructOpt;
// use regex::Regex;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

fn main() {
    let args = Cli::from_args();
    let slope = read_and_parse(&args.path);

    println!("{},{}", slope.len(), slope[0].len());
    let trees = downhill(3, 1, &slope);

    println!("{} trees for 3,1!", trees);

    let product = downhill(1, 1, &slope)
        * trees
        * downhill(5, 1, &slope)
        * downhill(7, 1, &slope)
        * downhill(1, 2, &slope);

    println!("{} all trees multiplied", product);
}

fn read_and_parse(path: &str) -> Vec<Vec<bool>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(|s| {
            s.chars()
                .map(|r| if r == '#' { true } else { false })
                .collect()
        })
        .collect()
}

fn downhill(step_right: usize, step_down: usize, slope: &Vec<Vec<bool>>) -> u64 {
    let num_horizontal = slope[0].len();
    let mut trees = 0;
    let mut pos_x = 0;
    let mut pos_y = 0;

    loop {
        pos_x = (pos_x + step_right) % num_horizontal;
        pos_y += step_down;

        println!("{},{}", pos_x, pos_y);
        if pos_y >= slope.len() {
            break;
        }
        println!("{}", slope[pos_y][pos_x]);
        if slope[pos_y][pos_x] {
            trees += 1;
        }
    }

    trees
}
