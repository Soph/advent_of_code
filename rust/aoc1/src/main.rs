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
  
  let numbers = read_and_parse(&args.path);

  two_numbers(numbers.clone());
  three_numbers(numbers);
}

fn read_and_parse(path: &str) -> Vec<i32> {
  let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

  let numbers: Vec<i32> = contents
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

  numbers
}

fn two_numbers(numbers: Vec<i32>) {
  for number_a in numbers.iter() {
    for number_b in numbers.iter() {
      if number_a != number_b {
        let result = number_a + number_b;
        if result == 2020 {
          println!("Found {} and {} with the result of {}", number_a, number_b, number_a * number_b);
          return;
        }
      }
    }
  }
}

fn three_numbers(numbers: Vec<i32>) {
  for number_a in numbers.iter() {
    for number_b in numbers.iter() {
      for number_c in numbers.iter() {
        if number_a != number_b && number_a != number_c && number_b != number_c {
          let result = number_a + number_b + number_c;
          if result == 2020 {
            println!("Found {}, {}, {} with the result of {}", number_a, number_b, number_c, number_a * number_b * number_c);
            return;
          }
        }
      }
    }
  }
}