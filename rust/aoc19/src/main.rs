use regex::Regex;
use std::collections::HashMap;
use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

struct Data {
    rules: HashMap<u32, String>,
    data: Vec<String>,
}

fn main() {
    let args = Cli::from_args();
    let data = read_and_parse(&args.path);

    let rule1 = build_pattern(&data.rules, 0, 0);
    println!("Final Pattern 1: {}", rule1);
    println!("Result1: {}", count1(&rule1, &data.data));

    let mut updated_rules = data.rules.clone();
    updated_rules.insert(8, "42 +".to_string());
    updated_rules.insert(11, "42 31 | 42 11 31".to_string());

    let rule2 = build_pattern(&updated_rules, 0, 0);

    println!("Pattern 11: {}", build_pattern(&updated_rules, 11, 0));
    println!("Final Pattern 2: {}", rule2);
    println!("Result2: {}", count1(&rule2, &data.data));
}

fn read_and_parse(path: &str) -> Data {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let parts: Vec<String> = contents.split("\n\n").map(ToOwned::to_owned).collect();

    let rules: HashMap<u32, String> = parts[0]
        .split("\n")
        .map(ToOwned::to_owned)
        .map(|s| {
            let mut result = s.split(": ");
            (
                result.next().unwrap().parse().unwrap(),
                result.next().unwrap().to_string(),
            )
        })
        .collect();
    let data = parts[1].split("\n").map(ToOwned::to_owned).collect();

    Data {
        data: data,
        rules: rules,
    }
}

fn build_pattern(rules: &HashMap<u32, String>, index: u32, depth: u32) -> String {
	// Max recursion of 30, is enough to get the right answer
    if depth > 30 {
        return String::new();
    }
    if rules[&index].contains("\"") {
        return rules[&index].replace("\"", "");
    } else {
        let mut result = String::new();
        for item in rules[&index].split(" ") {
            match item {
                "|" => {
                    result.push('|');
                }
                "+" => {
                    result.push('+');
                }
                "*" => {
                    result.push('*');
                }
                _ => {
                    let pattern = build_pattern(rules, item.parse().unwrap(), depth + 1);
                    if pattern.contains("|") {
                        result.push_str(format!("({})", pattern.as_str()).as_str());
                    } else {
                        result.push_str(pattern.as_str());
                    }
                }
            }
        }
        return result;
    }
}

fn count1(rule: &String, data: &Vec<String>) -> u32 {
    let regex = Regex::new(format!(r"^{}$", &rule).as_str()).unwrap();
    let mut count = 0;
    for line in data {
        if regex.is_match(&line) {
            println!("Match: {}", line);
            count += 1;
        }
    }
    count
}
