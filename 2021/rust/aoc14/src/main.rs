use std::collections::HashMap;
use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let args = Cli::from_args();
    let (pattern, insertion_rules) = read_and_parse(&args.path);

    run1(&pattern, &insertion_rules);

    let (pattern2, insertion_rules2) = read_and_parse2(&args.path);

    run2(&pattern2, &insertion_rules2);
}

fn read_and_parse(path: &str) -> (String, HashMap<String, String>) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut split = contents.split("\n\n");
    let pattern = split.next().unwrap().to_owned();

    let mut insertion_rules: HashMap<String, String> = HashMap::new();
    for line in split.next().unwrap().split("\n") {
        let mut insertion_rule = line.split(" -> ");
        let key = insertion_rule.next().unwrap().to_string();
        let key_chars: Vec<char> = key.chars().collect();
        let mut value: String = "".to_string();
        value.push(key_chars[0]);
        value.push_str(insertion_rule.next().unwrap());
        insertion_rules.insert(key, value);
    }

    (pattern, insertion_rules)
}

fn run1(pattern: &String, insertion_rules: &HashMap<String, String>) {
    let mut run_pattern = pattern.clone();
    for _ in 0..10 {
        let pattern_chars: Vec<char> = run_pattern.chars().collect();
        let mut new_pattern: String = "".to_string();
        for i in 1..pattern_chars.len() {
            let mut key: String = "".to_string();
            key.push(pattern_chars[i - 1]);
            key.push(pattern_chars[i]);
            new_pattern.push_str(insertion_rules.get(&key).unwrap());
        }
        new_pattern.push(pattern_chars[pattern_chars.len() - 1]);
        run_pattern = new_pattern.clone();
    }

    count(&run_pattern);
}

fn read_and_parse2(path: &str) -> (String, HashMap<String, Vec<String>>) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut split = contents.split("\n\n");
    let pattern = split.next().unwrap().to_owned();

    let mut insertion_rules: HashMap<String, Vec<String>> = HashMap::new();
    for line in split.next().unwrap().split("\n") {
        let mut insertion_rule = line.split(" -> ");
        let key = insertion_rule.next().unwrap().to_string();
        let key_chars: Vec<char> = key.chars().collect();
        let mut value1: String = "".to_string();
        let insert_letter = insertion_rule.next().unwrap();
        value1.push(key_chars[0]);
        value1.push_str(insert_letter);
        let mut value2: String = "".to_string();
        value2.push_str(insert_letter);
        value2.push(key_chars[1]);
        insertion_rules.insert(key, vec![value1, value2]);
    }

    (pattern, insertion_rules)
}

fn run2(pattern: &String, insertion_rules: &HashMap<String, Vec<String>>) {
    let mut run_pattern: HashMap<String, u128> = HashMap::new();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    for i in 1..pattern_chars.len() {
        let mut key: String = "".to_string();
        key.push(pattern_chars[i - 1]);
        key.push(pattern_chars[i]);

        *run_pattern.entry(key).or_insert(0) += 1;
    }

    for _ in 0..40 {
        let mut new_pattern: HashMap<String, u128> = HashMap::new();
        for (pattern, count) in &run_pattern {
            for rule in insertion_rules.get(pattern).unwrap() {
                *new_pattern.entry(rule.clone()).or_insert(0) += count;
            }
        }
        run_pattern = new_pattern;
    }
    count2(&pattern, &run_pattern);
}

fn count2(pattern: &String, run_pattern: &HashMap<String, u128>) {
    let mut letter_count: HashMap<char, u128> = HashMap::new();
    for (letters, count) in run_pattern {
        *letter_count
            .entry(letters.chars().next().unwrap())
            .or_insert(0) += count;
    }
    // last letter in the initial string needs to be +1 once since it's not the first letter of a pair
    *letter_count
        .entry(pattern.chars().last().unwrap())
        .or_insert(0) += 1;

    let mut min = 0;
    let mut max = 0;
    for (_, v) in letter_count {
        if v > max {
            max = v;
        }
        if min == 0 || min > v {
            min = v;
        }
    }
    println!("Part2: {}", max - min);
}

fn count(pattern: &String) {
    let mut min = 0;
    let mut max = 0;
    let mut count: HashMap<char, u64> = HashMap::new();
    for c in pattern.chars() {
        let count_value = count.entry(c).or_insert(0);
        *count_value += 1;
    }

    for (_, v) in count {
        if v > max {
            max = v;
        }
        if min == 0 || min > v {
            min = v;
        }
    }
    println!("Part1: {}", max - min);
}
