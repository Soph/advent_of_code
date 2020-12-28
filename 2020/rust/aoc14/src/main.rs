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

    let result = run(&data, 1);
    println!("Result1: {}", result);

    let result2 = run(&data, 2);
    println!("Result2: {}", result2);
}

fn read_and_parse(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.split("\n").map(ToOwned::to_owned).collect()
}

fn run(data: &Vec<String>, mask_type: u8) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask: String = "".to_string();

    for line in data {
        if line[0..3] == *"mem" {
            let (address, value) = parse_mem(&line);
            if mask_type == 1 {
                mem.insert(address, apply_mask(&mask, value));
            } else {
                let updated_mask = update_mask(&mask, address);
                let masks = generate_masks(&updated_mask);
                for sub_mask in masks {
                    mem.insert(apply_mask(&sub_mask, address), value);
                }
            }
        } else {
            let parts: Vec<&str> = line.split(" = ").collect();
            mask = parts[1].to_string();
        }
    }

    let mut sum = 0;
    for (_, v) in mem {
        sum += v;
    }
    sum
}

fn parse_mem(line: &str) -> (u64, u64) {
    let regexp = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let captures = regexp.captures(line).unwrap();

    (captures[1].parse().unwrap(), captures[2].parse().unwrap())
}

fn parse_mask(mask_string: &String) -> HashMap<u8, u8> {
    let mut result: HashMap<u8, u8> = HashMap::new();
    for (i, v) in mask_string.char_indices() {
        let bit_index = mask_string.len() - i - 1;
        match v {
            '0' => {
                result.insert(bit_index as u8, 0);
            }
            '1' => {
                result.insert(bit_index as u8, 1);
            }
            _ => (),
        };
    }

    result
}

fn generate_masks(mask_string: &String) -> Vec<String> {
    let mut mask_strings: Vec<String> = vec![];

    let x_count = mask_string.matches("X").count();
    for i in 0..1 << x_count {
        let variant = format!("{:0width$b}", i, width = x_count);
        let mut mutable_mask = mask_string.clone();
        for c in variant.chars() {
            mutable_mask = mutable_mask.replacen("X", c.to_string().as_str(), 1);
        }
        mask_strings.push(mutable_mask);
    }

    return mask_strings;
}

fn update_mask(mask: &String, value: u64) -> String {
    let value_mask = format!("{:036b}", value);
    let mut updated_chars: Vec<char> = vec![];

    for (i, v) in value_mask.char_indices() {
        match &mask.chars().nth(i).unwrap() {
            'X' => {
                updated_chars.push('X');
            }
            '0' => {
                updated_chars.push(v);
            }
            '1' => {
                updated_chars.push('1');
            }
            _ => ()
        }
    }

    updated_chars.into_iter().collect()
}

fn apply_mask(mask: &String, value: u64) -> u64 {
    let mut result = value;
    let parsed_mask = parse_mask(mask);

    for (k, v) in parsed_mask {
        match v {
            0 => {
                result &= u64::MAX - (1 << k);
            }
            1 => {
                result |= 1 << k;
            }
            _ => (),
        }
    }

    result
}
