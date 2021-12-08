use std::collections::HashMap;
use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

#[derive(Clone, Debug, PartialEq)]
struct Line {
    signals: Vec<String>,
    outputs: Vec<String>,
}

fn main() {
    let args = Cli::from_args();
    let lines = read_and_parse(&args.path);

    count1(&lines);
}

fn read_and_parse(path: &str) -> Vec<Line> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.split("\n").map(|s| parse_line(s)).collect()
}

fn parse_line(line: &str) -> Line {
    let mut parts = line.split(" | ");
    let signals = parts
        .next()
        .unwrap()
        .split(" ")
        .map(ToOwned::to_owned)
        .map(|s| sort_letters(s))
        .collect();
    let outputs = parts
        .next()
        .unwrap()
        .split(" ")
        .map(ToOwned::to_owned)
        .collect();

    return Line {
        signals: signals,
        outputs: outputs,
    };
}

fn sort_letters(letters: String) -> String {
    let mut chars: Vec<char> = letters.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    return String::from_iter(chars);
}

fn count1(lines: &Vec<Line>) {
    let mut count = 0;

    for line in lines {
        for output in &line.outputs {
            match output.len() {
                2 | 3 | 4 | 7 => {
                    count += 1;
                }
                _ => (),
            }
        }
    }

    println!("{}", count);
}

fn find_mapping(line: Line) -> HashMap<String, u64> {
    let mut mapping = HashMap::new();

    return mapping;
}

fn convert(found_mapping: HashMap<String, String>) -> HashMap<String, u64> {
    let mut mapping = HashMap::new();

    for (k, v) in digits() {
        let mut key = "".to_owned();
        for letter in k.chars() {
            key.push_str(&found_mapping[&letter.to_string()]);
        }
        mapping.insert(key, v);
    }

    return mapping;
}

fn digits() -> HashMap<String, u64> {
    let mut map = HashMap::new();

    map.insert("abcefg".to_string(), 0);
    map.insert("cf".to_string(), 1);
    map.insert("acdeg".to_string(), 2);
    map.insert("acdfg".to_string(), 3);
    map.insert("bcdf".to_string(), 4);
    map.insert("abdfg".to_string(), 5);
    map.insert("abdefg".to_string(), 6);
    map.insert("acf".to_string(), 7);
    map.insert("abcdefg".to_string(), 8);
    map.insert("abcdfg".to_string(), 9);

    return map;
}
