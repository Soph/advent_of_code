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
    let chunks = read_and_parse(&args.path);

    let filtered = check1(&chunks);
    check2(&filtered);
}

fn read_and_parse(path: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(ToOwned::to_owned)
        .map(|s| s.chars().collect())
        .collect()
}

fn check1(chunks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut sum = 0;
    let mut filtered_chunks = vec![];
    for chunk in chunks {
        let mut opens: Vec<char> = vec![];
        let mut broken = false;

        for c in chunk {
            match c {
                '(' | '<' | '{' | '[' => opens.push(*c),
                ')' => match opens.last() {
                    Some('(') => {
                        opens.pop();
                    }
                    _ => {
                        sum += value_per_bracket(c);
                        broken = true;
                        break;
                    }
                },
                '>' => match opens.last() {
                    Some('<') => {
                        opens.pop();
                    }
                    _ => {
                        sum += value_per_bracket(c);
                        broken = true;
                        break;
                    }
                },
                ']' => match opens.last() {
                    Some('[') => {
                        opens.pop();
                    }
                    _ => {
                        sum += value_per_bracket(c);
                        broken = true;
                        break;
                    }
                },
                '}' => match opens.last() {
                    Some('{') => {
                        opens.pop();
                    }
                    _ => {
                        sum += value_per_bracket(c);
                        broken = true;
                        break;
                    }
                },
                _ => (),
            }
        }
        if !broken {
            filtered_chunks.push(chunk.clone());
        }
    }

    println!("{}", sum);

    return filtered_chunks;
}

fn value_per_bracket(bracket: &char) -> i32 {
    return match bracket {
        ')' => 3,
        '>' => 25137,
        '}' => 1197,
        ']' => 57,
        _ => 0,
    };
}

fn value_per_bracket2(bracket: &char) -> u64 {
    return match bracket {
        ')' => 1,
        '>' => 4,
        '}' => 3,
        ']' => 2,
        _ => 0,
    };
}

fn check2(chunks: &Vec<Vec<char>>) {
    let mapping: HashMap<char, char> =
        HashMap::from([('(', ')'), ('<', '>'), ('{', '}'), ('[', ']')]);

    let mut sums: Vec<u64> = vec![];

    for chunk in chunks {
        let mut opens: Vec<char> = vec![];
        let mut sum = 0;

        for c in chunk {
            match c {
                '(' | '<' | '{' | '[' => opens.push(*c),
                _ => {
                    opens.pop();
                }
            }
        }
        opens.reverse();
        for open in &opens {
            sum *= 5;
            sum += value_per_bracket2(mapping.get(open).unwrap());
        }

        sums.push(sum)
    }
    sums.sort();
    println!("{:?}", sums[sums.len() / 2]);
}
