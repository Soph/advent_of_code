use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

#[derive(Copy, Clone)]
struct Range {
    start: u64,
    end: u64,
}

#[derive(Copy, Clone)]
struct Rule {
    range1: Range,
    range2: Range,
}

struct Data {
    rules: HashMap<String, Rule>,
    own_ticket: Vec<u64>,
    other_tickets: Vec<Vec<u64>>,
}

fn main() {
    let args = Cli::from_args();
    let data = read_and_parse(&args.path);

    let (result1, valid_tickets) = scan_tickets(&data);

    println!("Result1: {}", result1);

    let mappings: HashMap<u64, HashSet<String>> = find_mappings(&data.rules, &valid_tickets);
    let mapping = find_mapping(mappings);

    for (key, value) in &mapping {
        println!("{} => {}", key, value);
    }

    let mut sum = 1;
    for i in 0..data.own_ticket.len() {
        println!("{}: {}", mapping[&(i as u64)], data.own_ticket[i]);
        if mapping[&(i as u64)].starts_with("departure") {
            sum *= data.own_ticket[i];
        }
    }
    println!("Result2: {}", sum);
}

fn read_and_parse(path: &str) -> Data {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let parts: Vec<String> = contents.split("\n\n").map(ToOwned::to_owned).collect();

    let own_ticket: Vec<u64> = parts[1]
        .split("\n")
        .last()
        .unwrap()
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let other_tickets: Vec<Vec<u64>> = parts[2]
        .split("\n")
        .map(ToOwned::to_owned)
        .collect::<Vec<String>>()[1..]
        .iter()
        .map(|s| {
            s.split(",")
                .map(|s| s.parse().expect("parse error"))
                .collect()
        })
        .collect();

    let rule_lines: Vec<String> = parts[0].split("\n").map(ToOwned::to_owned).collect();
    let mut rules: HashMap<String, Rule> = HashMap::new();

    for line in rule_lines {
        let part: Vec<String> = line.split(": ").map(ToOwned::to_owned).collect();

        rules.insert(part[0].clone(), parse_rule(&part[1]));
    }

    Data {
        rules: rules,
        own_ticket: own_ticket,
        other_tickets: other_tickets,
    }
}

fn scan_tickets(data: &Data) -> (u64, Vec<Vec<u64>>) {
    let mut invalid: Vec<u64> = vec![];
    let mut valid_tickets: Vec<Vec<u64>> = vec![];
    for ticket in &data.other_tickets {
        match validate_rules(
            ticket,
            &data.rules.values().map(ToOwned::to_owned).collect(),
        ) {
            Some(x) => {
                invalid.push(x);
            }
            None => {
                valid_tickets.push(ticket.clone());
            }
        }
    }

    (invalid.iter().sum(), valid_tickets)
}

fn validate_rules(ticket: &Vec<u64>, rules: &Vec<Rule>) -> Option<u64> {
    let mut valid;
    for value in ticket {
        valid = false;
        for rule in rules {
            if (value >= &rule.range1.start && value <= &rule.range1.end)
                || (value >= &rule.range2.start && value <= &rule.range2.end)
            {
                valid = true;
            }
        }
        if !valid {
            return Some(*value);
        }
    }

    None
}

fn parse_rule(rule: &String) -> Rule {
    let mut ranges: Vec<Range> = vec![];
    for part in rule.split(" or ") {
        let values: Vec<u64> = part
            .split("-")
            .map(|s| s.parse().expect("parse error"))
            .collect();
        ranges.push(Range {
            start: values[0],
            end: values[1],
        })
    }

    Rule {
        range1: ranges[0],
        range2: ranges[1],
    }
}

fn invalid_rules(tickets: &Vec<Vec<u64>>, rule: &Rule) -> HashSet<u64> {
    let mut result: HashSet<u64> = HashSet::new();

    for ticket in tickets {
        for (i, value) in ticket.iter().enumerate() {
            if !((value >= &rule.range1.start && value <= &rule.range1.end)
                || (value >= &rule.range2.start && value <= &rule.range2.end))
            {
                result.insert(i as u64);
            }
        }
    }

    result
}

fn find_mappings(
    rules: &HashMap<String, Rule>,
    tickets: &Vec<Vec<u64>>,
) -> HashMap<u64, HashSet<String>> {
    let mut result: HashMap<u64, HashSet<String>> = HashMap::new();
    for (name, rule) in rules {
        let invalid_rules = invalid_rules(tickets, rule);
        for i in 0..tickets[0].len() {
            if !invalid_rules.contains(&(i as u64)) {
                result
                    .entry(i as u64)
                    .or_insert(HashSet::new())
                    .insert(name.clone());
            }
        }
    }

    result
}

fn find_mapping(mappings: HashMap<u64, HashSet<String>>) -> HashMap<u64, String> {
    let mut result: HashMap<u64, String> = HashMap::new();
    let mut mutable_mappings: HashMap<u64, HashSet<String>> = mappings.clone();

    let mut found: String = "".to_string();
    loop {
        for (position, names) in &mutable_mappings {
            if names.len() == 1 {
                let name = names.into_iter().last().unwrap();
                result.insert(*position, name.to_string());
                found = name.to_string();
            }
        }

        let keys = mappings.keys();
        for key in keys {
            let names = mutable_mappings.entry(*key).or_insert(HashSet::new());
            if names.contains(&found) {
                names.remove(&found);
            }
        }

        if mappings.len() == result.len() {
            return result;
        }
    }
}
