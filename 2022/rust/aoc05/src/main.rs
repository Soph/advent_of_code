use std::fs;
use structopt::StructOpt;
use std::str::FromStr;
use std::convert::Infallible;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd)]
struct Move {
    count: i64,
    from: i64,
    to: i64,
}

impl FromStr for Move {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Move, Infallible> {
        let parts: Vec<&str> = s.split(' ').collect();

        Ok(Move {
            count: parts[1].parse().unwrap(),
            from: parts[3].parse().unwrap(),
            to: parts[5].parse().unwrap(),
        })      
    }
}



fn main() {
    let args = Cli::from_args();
    let all = read_and_parse(&args.path);

    println!("Test: {:?}", all);

    // let mut results: Vec<i32> = all.iter().map(|elf| elf.iter().sum()).collect();
    // results.sort();

    // println!("Max: {}", results.last().unwrap());
    // println!(
    //     "Last 3: {}",
    //     results.as_slice()[results.len() - 3..]
    //         .to_vec()
    //         .iter()
    //         .sum::<i32>()
    // );
}

fn read_and_parse(path: &str) -> Vec<Move> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let parts: Vec<&str> = contents.split("\n\n").collect();

    // parse moves
    let moves = parts[1]
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();

    moves
}

/*
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
*/
fn parse_stacks(lines: &str) -> Vec<Vec<char>> {
    let mut points = Vec::new();
    for line in lines.split("\n") {
        for 
    }
}
