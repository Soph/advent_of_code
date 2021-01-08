use std::cmp;
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

    let orbits = read_and_parse(&args.path);

    let mut count = 0;
    for (key, _) in &orbits {
        count += count_path(&orbits, key)
    }
    println!("Orbits: {}", count);

    let mut path_you = path(&orbits, &"YOU".to_string());
    let mut path_san = path(&orbits, &"SAN".to_string());

    path_san.reverse();
    path_you.reverse();

    for i in 0..cmp::max(path_you.len(), path_san.len()) {
        if path_you[0..i] != path_san[0..i] {
            println!(
                "Transfers: {}",
                path_you.len() + path_san.len() - (2 * (i - 1))
            );
            break;
        }
    }
}

fn read_and_parse(path: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();

    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    for line in contents.split("\n") {
        let parts: Vec<&str> = line.split(")").collect();
        result.insert(parts[1].to_string(), parts[0].to_string());
    }

    result
}

fn count_path(orbits: &HashMap<String, String>, key: &String) -> u64 {
    let mut count = 0;
    let mut current_key = key.clone();
    loop {
        match orbits.get(&current_key) {
            Some(object) => {
                count += 1;
                current_key = object.clone();
            }
            None => {
                return count;
            }
        }
    }
}

fn path(orbits: &HashMap<String, String>, key: &String) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_key = key.clone();
    loop {
        match orbits.get(&current_key) {
            Some(object) => {
                if *object == "COM".to_string() {
                    return result;
                }
                result.push(object.clone());
                current_key = object.clone();
            }
            None => {
                return result;
            }
        }
    }
}
