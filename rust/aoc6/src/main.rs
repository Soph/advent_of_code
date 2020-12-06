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

    // question 1
    let answers1 = read_and_parse(&args.path, &parse_answers1);
    let count1 = full_count(&answers1);
    println!("Count for 1: {}", count1);

    // question 2
    let answers2 = read_and_parse(&args.path, &parse_answers2);
    let count2 = full_count(&answers2);
    println!("Count for 2: {}", count2);
}

fn read_and_parse(path: &str, parse: &dyn Fn(&str) -> String) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.split("\n\n").map(|s| parse(s)).collect()
}

fn parse_answers1(answers: &str) -> String {
    let one_line = answers.replace("\n", "");
    let mut result: Vec<char> = one_line.chars().collect();

    result.sort();
    result.dedup();

    result.into_iter().collect()
}

fn full_count(all_answers: &Vec<String>) -> u32 {
    let mut sum = 0;
    for answers in all_answers {
        sum += answers.len() as u32;
    }

    sum
}

fn parse_answers2(answers: &str) -> String {
    let mut letters = HashMap::new();

    let lines: Vec<&str> = answers.split("\n").collect();
    let person_count = lines.len();

    for line in lines {
        for char in line.chars() {
            let entry = letters.entry(char).or_insert(0);
            *entry += 1;
        }
    }

    letters
        .into_iter()
        .filter(|&(_, v)| v == person_count)
        .collect::<HashMap<char, usize>>()
        .keys()
        .collect()
}
