use std::fs;
use structopt::StructOpt;
// use regex::Regex;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
  /// The path to the file to read
  path: String,
}

fn main() {
  let args = Cli::from_args();
  
  let lines = read_and_parse(&args.path);

  let valid1 = count_valid1(lines.clone());

  println!("Contains {} valid passwords for rule 1", valid1);

  let valid2 = count_valid2(lines);
  println!("Contains {} valid passwords for rule 2", valid2);
}

fn read_and_parse(path: &str) -> Vec<String> {
  let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

  let lines: Vec<String> = contents
        .split("\n")
        .map(ToOwned::to_owned)
        .collect();

  return lines;
}

fn count_valid1(lines: Vec<String>) -> u64 {
  let mut valid = 0;
  for line in lines.iter() {
    if is_valid1(line) {
      valid += 1;
    } else {
    }
  }
  valid
}

fn is_valid1(line: &str) -> bool {
  let parts: Vec<&str> = line.split(':').collect();
  let rules: Vec<&str> = parts[0].split_whitespace().collect();
  let minmax: Vec<usize> = rules[0].split('-').map(|s| s.parse().expect("parse error")).collect();

  // This produced 130. It also returned positive if there where more letters then given in the range :(
  // let regexp_str = format!(r#"\b(?:[^{letter}\s]*{letter}){{{range}}}[^{letter}\s]*\b"#, letter=rules[1], range=rules[0].replace("-", ","));
  // println!("{}", regexp_str);
  // let regexp = Regex::new(regexp_str.as_str()).unwrap();

  // regexp.is_match(parts[0].replace(" ","").as_str())

  let count_letter = parts[1].matches(&rules[1]).count();

  if count_letter >= minmax[0] && count_letter <= minmax[1] {
    true
  } else {
    false
  }
}

fn count_valid2(lines: Vec<String>) -> u64 {
  let mut valid = 0;
  for line in lines.iter() {
    if is_valid2(line) {
      valid += 1;
    } else {
    }
  }
  valid
}

fn is_valid2(line: &str) -> bool {
  let parts: Vec<&str> = line.split(':').collect();
  let rules: Vec<&str> = parts[0].split_whitespace().collect();
  let minmax: Vec<usize> = rules[0].split('-').map(|s| s.parse().expect("parse error")).collect();

  let password = parts[1].replace(" ","");

  // this would not work with utf8 passwords
  let letter_a = password.chars().nth(minmax[0]-1).expect("message").to_string();
  let letter_b = password.chars().nth(minmax[1]-1).expect("message").to_string();

  if (letter_a == rules[1] || letter_b == rules[1]) && (letter_a != letter_b)  {
    println!("valid: {} -> {}/{} {}/{}", line, letter_a, rules[1], letter_b, rules[1]);

    true
  } else {
    println!("invalid: {} -> {}/{} {}/{}", line, letter_a, rules[1], letter_b, rules[1]);

    false
  }
}