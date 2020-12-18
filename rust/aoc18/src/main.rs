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

fn main() {
    let args = Cli::from_args();
    let data = read_and_parse(&args.path);

    let mut sum = 0;
    for line in &data {
        let result = calculate_all(&line, calculate_numbers1);
        sum += result;
    }

    println!("Result1: {}", sum);

    sum = 0;
    for line in &data {
        let result = calculate_all(&line, calculate_numbers2);
        sum += result;
    }

    println!("Result2: {}", sum)
}

fn read_and_parse(path: &String) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.split("\n").map(ToOwned::to_owned).collect()
}

fn calculate_all(line: &String, calculate_numbers: fn(&String) -> u64) -> u64 {
    let mut processing = line.clone();
    let regex = Regex::new(r"\([^\)\(]*\)").unwrap();
    loop {
        let mut found = false;
        let mut replacements: HashMap<String, u64> = HashMap::new();
        for capture in regex.captures_iter(processing.as_str()) {
            found = true;
            replacements.insert(
                capture[0].to_string(),
                calculate_numbers(&capture[0].replace("(", "").replace(")", "").to_string()),
            );
        }

        for (key, value) in replacements {
            processing = processing.replace(key.as_str(), value.to_string().as_str());
        }
        // println!("{}", processing);
        if !found {
            let result = calculate_numbers(&processing);
            println!("{} => {}", &line, result);
            return result;
        }
    }
}

fn calculate_numbers1(numbers: &String) -> u64 {
    let mut processing = numbers.replace(" ", "");
    let regex = Regex::new(r"(\d+)([\+\*])(\d+)").unwrap();

    loop {
        let mut found = false;
        let mut replacements: HashMap<String, u64> = HashMap::new();
        let captures = regex.captures_iter(processing.as_str()).next();
        match captures {
            Some(capture) => match &capture[2] {
                "+" => {
                    found = true;
                    replacements.insert(
                        capture[0].to_string(),
                        capture[1].parse::<u64>().unwrap() + capture[3].parse::<u64>().unwrap(),
                    );
                }
                "*" => {
                    found = true;
                    replacements.insert(
                        capture[0].to_string(),
                        capture[1].parse::<u64>().unwrap() * capture[3].parse::<u64>().unwrap(),
                    );
                }
                _ => (),
            },
            None => (),
        }

        for (key, value) in replacements {
            processing = processing.replacen(key.as_str(), value.to_string().as_str(), 1);
        }
        if !found {
            return processing.parse().unwrap();
        }
    }
}

fn calculate_numbers2(numbers: &String) -> u64 {
    let mut processing = numbers.replace(" ", "");
    let regex_add = Regex::new(r"(\d+)([\+])(\d+)").unwrap();
    let regex_mul = Regex::new(r"(\d+)([\*])(\d+)").unwrap();

    loop {
        let mut found = false;
        let mut replacements: HashMap<String, u64> = HashMap::new();
        let captures = regex_add.captures_iter(processing.as_str()).next();
        match captures {
            Some(capture) => match &capture[2] {
                "+" => {
                    found = true;
                    replacements.insert(
                        capture[0].to_string(),
                        capture[1].parse::<u64>().unwrap() + capture[3].parse::<u64>().unwrap(),
                    );
                }
                _ => (),
            },
            None => (),
        }

        for (key, value) in replacements {
            processing = processing.replacen(key.as_str(), value.to_string().as_str(), 1);
        }
        if !found {
            break;
        }
    }

    loop {
        let mut found = false;
        let mut replacements: HashMap<String, u64> = HashMap::new();
        let captures = regex_mul.captures_iter(processing.as_str()).next();
        match captures {
            Some(capture) => match &capture[2] {
                "*" => {
                    found = true;
                    replacements.insert(
                        capture[0].to_string(),
                        capture[1].parse::<u64>().unwrap() * capture[3].parse::<u64>().unwrap(),
                    );
                }
                _ => (),
            },
            None => (),
        }

        for (key, value) in replacements {
            processing = processing.replacen(key.as_str(), value.to_string().as_str(), 1);
        }
        if !found {
            return processing.parse().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate1() {
        assert_eq!(calculate_numbers1(&"1 + 2 * 3 + 4 * 5 + 6".to_string()), 71);
        assert_eq!(calculate_numbers1(&"4 * 11".to_string()), 44);
        assert_eq!(calculate_numbers1(&"3 * 2 * 3".to_string()), 18);
    }

    #[test]
    fn test_calculate2() {
        assert_eq!(
            calculate_numbers2(&"1 + 2 * 3 + 4 * 5 + 6".to_string()),
            231
        );
        assert_eq!(calculate_numbers2(&"4 * 11".to_string()), 44);
        assert_eq!(calculate_numbers2(&"3 * 2 * 3".to_string()), 18);
    }

    #[test]
    fn test_calculate_all() {
        assert_eq!(
            calculate_all(
                &"7 * 7 * (5 + 9 + 9) * (8 * 6 * 5 + 4 * 2)".to_string(),
                calculate_numbers1
            ),
            549976
        );
        assert_eq!(calculate_all(&"((5 + 8 + 7 + 6 * 6 + 3) + 4 * 3 * (5 * 2) * 5 * (2 * 9)) + 6 + 3 * (2 * (2 + 4 + 8) * 4) + (9 + 3 + 6 * (9 + 5 * 6 * 3 * 8 * 5) + (4 * 6) + (6 * 3 + 2 + 8))".to_string(), calculate_numbers1), 49473700);
        assert_eq!(
            calculate_all(
                &"4 + 5 + 6 * 6 * 6 + (3 * 2 * 3 * (2 * 6 + 9) + 6 + (5 + 2 + 9 * 6))".to_string(),
                calculate_numbers1
            ),
            1020
        );
    }

    #[test]
    fn test_calculate_all2() {
        assert_eq!(
            calculate_all(
                &"7 * 7 * (5 + 9 + 9) * (8 * 6 * 5 + 4 * 2)".to_string(),
                calculate_numbers2
            ),
            549976
        );
        assert_eq!(calculate_all(&"((5 + 8 + 7 + 6 * 6 + 3) + 4 * 3 * (5 * 2) * 5 * (2 * 9)) + 6 + 3 * (2 * (2 + 4 + 8) * 4) + (9 + 3 + 6 * (9 + 5 * 6 * 3 * 8 * 5) + (4 * 6) + (6 * 3 + 2 + 8))".to_string(), calculate_numbers2), 49473700);
        assert_eq!(
            calculate_all(
                &"4 + 5 + 6 * 6 * 6 + (3 * 2 * 3 * (2 * 6 + 9) + 6 + (5 + 2 + 9 * 6))".to_string(),
                calculate_numbers2
            ),
            1020
        );
    }
}
