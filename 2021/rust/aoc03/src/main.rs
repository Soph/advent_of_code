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
    let diagnostics = read_and_parse(&args.path);

    analyse1(&diagnostics);
    analyse2(&diagnostics);
}

fn read_and_parse(path: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(ToOwned::to_owned)
        .map(|s| s.chars().map(|s| s.to_digit(10).unwrap()).collect())
        .collect()
}

fn analyse1(diagnostics: &Vec<Vec<u32>>) {
    let mut sums: Vec<u32> = vec![0; diagnostics[0].len()];
    let mut gamma: String = "".to_owned();
    let mut epsilon: String = "".to_owned();

    for values in diagnostics {
        for n in 0..values.len() {
            sums[n] += values[n];
        }
    }

    for sum in &sums {
        if sum > &(diagnostics.len() as u32 / 2) {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }
    let gamma_int = isize::from_str_radix(&*gamma, 2).unwrap();
    let epsilon_int = isize::from_str_radix(&*epsilon, 2).unwrap();

    println!("Sums: {:?}, Gamma: {}, Epsilon: {}", &sums, gamma, epsilon);
    println!(
        "Gamma: {}, Epsilon: {}, result: {}",
        gamma_int,
        epsilon_int,
        gamma_int * epsilon_int
    );
}

fn generate_string(bin_vec: &Vec<u32>) -> String {
    let mut string: String = "".to_owned();
    for bin in bin_vec {
        if *bin == 1 {
            string.push_str("1");
        } else {
            string.push_str("0");
        }
    }
    return string;
}

fn generate_sums(diagnostics: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut sums: Vec<u32> = vec![0; diagnostics[0].len()];

    for values in diagnostics {
        for n in 0..values.len() {
            sums[n] += values[n];
        }
    }

    return sums;
}

fn analyse2(diagnostics: &Vec<Vec<u32>>) {
    let filtered_oxygen = filter_oxygen(&diagnostics);
    let filtered_co2 = filter_co2(&diagnostics);

    let oxygen_int = isize::from_str_radix(&*generate_string(&filtered_oxygen[0]), 2).unwrap();
    let co2_int = isize::from_str_radix(&*generate_string(&filtered_co2[0]), 2).unwrap();

    println!("{} * {} = {}", oxygen_int, co2_int, oxygen_int * co2_int);
}

fn filter_oxygen(diagnostics: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut filtered: Vec<Vec<u32>> = diagnostics.clone();

    for n in 0..diagnostics[0].len() {
        let sums = generate_sums(&filtered);
        let mut new_filtered: Vec<Vec<u32>> = vec![];
        for values in &filtered {
            if ((sums[n] as f64) >= filtered.len() as f64 / 2.0) && values[n] == 1 {
                new_filtered.push(values.clone());
            } else if ((sums[n] as f64) < filtered.len() as f64 / 2.0) && values[n] == 0 {
                new_filtered.push(values.clone());
            }
        }
        if new_filtered.len() > 0 {
            filtered = new_filtered;
        }
    }

    return filtered;
}

fn filter_co2(diagnostics: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut filtered: Vec<Vec<u32>> = diagnostics.clone();

    for n in 0..diagnostics[0].len() {
        let sums = generate_sums(&filtered);
        let mut new_filtered: Vec<Vec<u32>> = vec![];
        for values in &filtered {
            if ((sums[n] as f64) < filtered.len() as f64 / 2.0) && values[n] == 1 {
                new_filtered.push(values.clone());
            } else if ((sums[n] as f64) >= filtered.len() as f64 / 2.0) && values[n] == 0 {
                new_filtered.push(values.clone());
            }
        }
        if new_filtered.len() > 0 {
            filtered = new_filtered;
        }
    }

    return filtered;
}
