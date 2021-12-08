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
    count2(&lines);
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

    println!("Part1: {}", count);
}

fn count2(lines: &Vec<Line>) {
    let mut sum: u64 = 0;

    for line in lines {
        let mapping = find_mapping(line);
        let mut digit = "".to_owned();
        for output in &line.outputs {
            let sorted = sort_letters(output.clone());
            digit.push_str(&mapping[&sorted].to_string());
        }
        sum += digit.parse::<u64>().unwrap();
    }

    println!("Part2: {}", sum);
}

fn find_mapping(line: &Line) -> HashMap<String, u64> {
    let mut char_mapping = char_map();
    let mut five_char_count: HashMap<char, u64> = HashMap::new();
    let mut six_char_count: HashMap<char, u64> = HashMap::new();

    for signal in line.signals.clone() {
        let s: Vec<char> = signal.chars().collect();
        match signal.len() {
            2 => {
                for c in s {
                    let mut chars = char_mapping[&c].clone();
                    chars.retain(|&x| x == 'c' || x == 'f');
                    char_mapping.insert(c, chars);
                }
            }
            3 => {
                for c in s {
                    let mut chars = char_mapping[&c].clone();
                    chars.retain(|&x| x == 'a' || x == 'c' || x == 'f');
                    char_mapping.insert(c, chars);
                }
            }
            4 => {
                for c in s {
                    let mut chars = char_mapping[&c].clone();
                    chars.retain(|&x| x == 'b' || x == 'c' || x == 'd' || x == 'f');
                    char_mapping.insert(c, chars);
                }
            }
            5 => {
                for c in s {
                    *five_char_count.entry(c).or_insert(0) += 1;
                }
            }
            6 => {
                for c in s {
                    *six_char_count.entry(c).or_insert(0) += 1;
                }
            }
            _ => (),
        }
    }

    for (k, v) in five_char_count {
        if v == 3 {
            let mut chars = char_mapping[&k].clone();
            chars.retain(|&x| x == 'a' || x == 'd' || x == 'g');
            char_mapping.insert(k, chars);
        }
    }

    for (k, v) in six_char_count {
        if v == 3 {
            let mut chars = char_mapping[&k].clone();
            chars.retain(|&x| x == 'a' || x == 'b' || x == 'f' || x == 'g');
            char_mapping.insert(k, chars);
        }
    }

    loop {
        for (c, chars) in char_mapping.clone() {
            if chars.len() == 1 {
                for (k, v) in char_mapping.clone() {
                    if k == c {
                        continue;
                    }
                    let mut cs = v.clone();
                    cs.retain(|&x| x != chars[0]);
                    char_mapping.insert(k, cs);
                }
            }
        }
        let mut more_then_one = false;
        for (_, chars) in char_mapping.clone() {
            if chars.len() != 1 {
                more_then_one = true;
            }
        }
        if !more_then_one {
            break;
        }
    }

    let mut inverted_mapping: HashMap<char, char> = HashMap::new();
    for (k, v) in char_mapping {
        inverted_mapping.insert(v[0], k);
    }

    return convert(inverted_mapping);
}

fn char_map() -> HashMap<char, Vec<char>> {
    let mut map = HashMap::new();

    map.insert('a', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    map.insert('b', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    map.insert('c', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    map.insert('d', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    map.insert('e', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    map.insert('f', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    map.insert('g', vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);

    return map;
}

fn convert(found_mapping: HashMap<char, char>) -> HashMap<String, u64> {
    let mut mapping = HashMap::new();

    for (k, v) in digits() {
        let mut key = "".to_owned();
        for letter in k.chars() {
            key.push(found_mapping[&letter]);
        }
        mapping.insert(sort_letters(key), v);
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
