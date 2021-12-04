use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

#[derive(Clone, Debug, PartialEq)]
struct BingoSet {
    numbers: Vec<Vec<u64>>,
    rows: Vec<u64>,
    columns: Vec<u64>,
    matches: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq)]
struct Play {
    bingo_sets: Vec<BingoSet>,
    numbers: Vec<u64>,
}

#[derive(Debug)]
struct Win {
    bingo_set: BingoSet,
    number: u64,
}

fn main() {
    let args = Cli::from_args();
    let play = read_and_parse(&args.path);

    let win = do_play_win(play.clone()).unwrap();
    let loose = do_play_loose(play.clone()).unwrap();

    println!("Win: {}", calc_answer(&win));
    println!("Loose: {}", calc_answer(&loose));
}

fn read_and_parse(path: &str) -> Play {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut splited = contents.split("\n\n");

    let numbers: Vec<u64> = splited
        .next()
        .unwrap()
        .split(",")
        .map(ToOwned::to_owned)
        .map(|s| s.parse().unwrap())
        .collect();

    return Play {
        bingo_sets: splited
            .map(ToOwned::to_owned)
            .map(|s| parse_bingo_set(s))
            .collect(),
        numbers: numbers,
    };
}

fn parse_bingo_set(bingo_set_string: String) -> BingoSet {
    let numbers: Vec<Vec<u64>> = bingo_set_string
        .split("\n")
        .map(|s| {
            s.replace("  ", " ")
                .trim()
                .split(" ")
                .map(|s| s.trim().parse::<u64>().expect("parse error"))
                .collect()
        })
        .collect();

    let len = numbers.len();
    return BingoSet {
        numbers: numbers,
        rows: vec![0; len],
        columns: vec![0; len],
        matches: vec![],
    };
}

fn do_play_win(mut play: Play) -> Option<Win> {
    for number in play.numbers {
        for i in 0..play.bingo_sets.len() {
            for x in 0..play.bingo_sets[i].numbers.len() {
                for y in 0..play.bingo_sets[i].numbers.len() {
                    if play.bingo_sets[i].numbers[x][y] == number {
                        play.bingo_sets[i].rows[x] += 1;
                        play.bingo_sets[i].columns[y] += 1;
                        play.bingo_sets[i].matches.push(number);
                        if (play.bingo_sets[i].rows[x] == play.bingo_sets[i].numbers.len() as u64)
                            || (play.bingo_sets[i].columns[y]
                                == play.bingo_sets[i].numbers.len() as u64)
                        {
                            // bingo
                            return Some(Win {
                                bingo_set: play.bingo_sets[i].clone(),
                                number: number,
                            });
                        }
                    }
                }
            }
        }
    }
    return None;
}

fn do_play_loose(mut play: Play) -> Option<Win> {
    let mut won: Vec<BingoSet> = vec![];
    for number in play.numbers {
        for i in 0..play.bingo_sets.len() {
            for x in 0..play.bingo_sets[i].numbers.len() {
                for y in 0..play.bingo_sets[i].numbers.len() {
                    if play.bingo_sets[i].numbers[x][y] == number {
                        if !won.contains(&play.bingo_sets[i]) {
                            play.bingo_sets[i].rows[x] += 1;
                            play.bingo_sets[i].columns[y] += 1;
                            play.bingo_sets[i].matches.push(number);
                            if (play.bingo_sets[i].rows[x]
                                == play.bingo_sets[i].numbers.len() as u64)
                                || (play.bingo_sets[i].columns[y]
                                    == play.bingo_sets[i].numbers.len() as u64)
                            {
                                // bingo
                                if won.len() < play.bingo_sets.len() - 1 {
                                    won.push(play.bingo_sets[i].clone());
                                } else {
                                    return Some(Win {
                                        bingo_set: play.bingo_sets[i].clone(),
                                        number: number,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return None;
}

fn calc_answer(win: &Win) -> u64 {
    let mut answer: u64 = 0;
    for row in &win.bingo_set.numbers {
        for value in row {
            if !win.bingo_set.matches.contains(&value) {
                answer += value;
            }
        }
    }
    return answer * win.number;
}
