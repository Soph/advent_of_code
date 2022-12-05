use std::convert::Infallible;
use std::fs;
use std::str::FromStr;
use structopt::StructOpt;

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
    let (mut stacks, moves) = read_and_parse(&args.path);

    let mut stacks_1 = stacks.clone();
    for m in moves.clone() {
        for n in 0..(m.count as usize) {
            let c = stacks_1[(m.from - 1) as usize].pop().unwrap();
            stacks_1[(m.to - 1) as usize].push(c);
        }
    }

    println!(
        "{:?}",
        stacks_1
            .iter()
            .map(|s| s.last().copied().unwrap())
            .collect::<Vec<char>>()
    );

    let mut stacks_2 = stacks.clone();
    for m in moves {
        let mut chars = Vec::new();
        for n in 0..(m.count as usize) {
            chars.push(stacks_2[(m.from - 1) as usize].pop().unwrap());
        }
        chars.reverse();
        stacks_2[(m.to - 1) as usize].append(&mut chars.clone());
    }

    println!(
        "{:?}",
        stacks_2
            .iter()
            .map(|s| s.last().copied().unwrap())
            .collect::<Vec<char>>()
    );
}

fn read_and_parse(path: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let parts: Vec<&str> = contents.split("\n\n").collect();

    let stacks = parse_stacks(parts[0]);

    // parse moves
    let moves = parts[1]
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();

    (stacks, moves)
}

/*
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
*/
fn parse_stacks(lines: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();

    let mut first = true;
    for line in lines.split("\n") {
        let mut y = 0;
        for chars in line.chars().collect::<Vec<char>>().chunks(4) {
            if first {
                stacks.push(Vec::new());
            }
            if chars[0] != ' ' {
                stacks[y].insert(0, chars[1]);
            }
            y += 1;
        }
        first = false;
    }

    stacks
}
