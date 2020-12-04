use regex::Regex;
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
    let passports = read_and_parse(&args.path);

    let count1 = count_valid1(&passports);

    println!("Count for first question is {}", count1);

    let count2 = count_valid2(&passports);

    println!("Count for second question is {}", count2);
}

fn read_and_parse(path: &str) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n\n")
        .map(ToOwned::to_owned)
        .map(|s| s.split_whitespace().map(ToOwned::to_owned).collect())
        .collect()
}

fn count_valid1(passports: &Vec<Vec<String>>) -> u64 {
    let mut count = 0;

    for passport in passports {
        if valid_passport_keys(passport) {
            count += 1;
        }
    }

    count
}

fn check_require(keys: Vec<String>) -> bool {
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for key in required {
        match keys.iter().position(|s| s == key) {
            None => return false,
            _ => (),
        }
    }

    true
}

fn valid_passport_keys(passport: &Vec<String>) -> bool {
    let keys = passport
        .into_iter()
        .map(|s| s.split(":").next().unwrap().to_owned())
        .collect();

    check_require(keys)
}

fn count_valid2(passports: &Vec<Vec<String>>) -> u64 {
    let mut count = 0;

    for passport in passports {
        if valid_passport_keys(passport) && valid_passport_values(passport) {
            println!("{}", passport.join("-"));
            count += 1;
        }
    }

    count
}

fn valid_passport_values(passport: &Vec<String>) -> bool {
    for attribute in passport {
        let key_value: Vec<String> = attribute.split(":").map(ToOwned::to_owned).collect();
        match key_value[0].as_str() {
            "byr" => {
                if !validate_number(key_value[1].parse().expect("parse error"), 1920, 2002) {
                    return false;
                }
            }
            "iyr" => {
                if !validate_number(key_value[1].parse().expect("parse error"), 2010, 2020) {
                    return false;
                }
            }
            "eyr" => {
                if !validate_number(key_value[1].parse().expect("parse error"), 2020, 2030) {
                    return false;
                }
            }
            "hgt" => {
                if !validate_height(&key_value[1]) {
                    return false;
                }
            }
            "hcl" => {
                if !validate_hair_color(&key_value[1]) {
                    return false;
                }
            }
            "ecl" => {
                if !validate_eye_color(&key_value[1]) {
                    return false;
                }
            }
            "pid" => {
                if !validate_passport_number(&key_value[1]) {
                    return false;
                }
            }
            _ => (),
        }
    }
    true
}

fn validate_number(number: u64, min: u64, max: u64) -> bool {
    if number >= min && number <= max {
        return true;
    }

    false
}

fn validate_eye_color(color: &str) -> bool {
    let valid = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    match valid.iter().position(|&s| s == color) {
        None => return false,
        _ => return true,
    }
}

fn validate_height(height: &String) -> bool {
    let mut mutable_height = height.clone();
    let height_value: String = mutable_height.drain(..mutable_height.len() - 2).collect();

    match mutable_height.as_str() {
        "cm" => return validate_number(height_value.parse().expect("parse error"), 150, 193),
        "in" => return validate_number(height_value.parse().expect("parse error"), 59, 76),
        _ => return false,
    }
}

fn validate_hair_color(hair_color: &String) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();

    re.is_match(hair_color)
}

fn validate_passport_number(passport_number: &String) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();

    re.is_match(passport_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_passport_number() {
        assert_eq!(validate_passport_number(&String::from("000000000")), true);
        assert_eq!(validate_passport_number(&String::from("0000000000")), false);
        assert_eq!(validate_passport_number(&String::from("00000000")), false);
    }
}
