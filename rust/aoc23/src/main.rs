use std::fs;
use structopt::StructOpt;
use std::cmp;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

fn main() {
    let args = Cli::from_args();
    let start = read_and_parse(&args.path);
    let (min, max) = find_min_max(&start);
    play(&start, min, max, 100);

    let mut part2 = start.clone();
    for i in max..=1000000 {
        part2.push(i);
    }

    play(&part2, min, 1000000, 10000000);
}

fn read_and_parse(path: &str) -> Vec<u32> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn play(start: &Vec<u32>, min: u32, max: u32, moves: u32) {
    let mut playfield = start.clone();
    let mut current_index = 0;
    let cups_count = playfield.len();
    for i in 0..moves {
        let current_value = playfield[current_index];
        if i % 10000 == 0 {
            println!("--- Move {} ---", i+1);
        }
        // print_cups(&playfield, current_index);
        let mut pick: Vec<u32> = playfield.drain(current_index+1..cmp::min(current_index+4,cups_count)).collect();
        if pick.len() < 3 {
            for _ in 0..(3-pick.len()) {
                pick.push(playfield.remove(0));
            }
        }
        // print_pick(&pick);
        let destination = find_destination(current_value, &pick, min, max);
        // println!("destination: {}", destination);
        let insert_position = playfield.iter().position(|&x| x == destination).unwrap() + 1;
        for i in 0..pick.len() {
            if i + insert_position < cups_count {
                // println!("Insert: {} at {}", pick[i], i + insert_position);
                playfield.insert(i + insert_position, pick[i]);
            } else {
                // println!("Insert: {} at {}", pick[i], (i + insert_position) - cups_count);
                playfield.insert((i + insert_position) - cups_count, pick[i]);
            }
        }
        current_index = playfield.iter().position(|&x| x == current_value).unwrap();
        // print_cups(&playfield, current_index);
        current_index += 1;
        if current_index >= playfield.len() {
            current_index = 0;
        }
    }

    let index_1 = playfield.iter().position(|&x| x == 1).unwrap();
    if moves == 100 {
        print!("Result1: ");
        for i in 1..playfield.len() {
            if i + index_1 < playfield.len() {
                print!("{} ", playfield[i+index_1]);
            } else {
                print!("{} ", playfield[i+index_1-playfield.len()]);
            }
        }
        println!("");
    } else {
        println!("Result2: {} {}", playfield[index_1+1], playfield[index_1+2]);
    }
}

fn find_destination(value: u32, pick: &Vec<u32>, min: u32, max: u32) -> u32 {
    let mut destination = value;
    loop {
        if destination == min {
            destination = max;
        } else {
            destination -= 1;
        }
        if !pick.contains(&destination) {
            return destination;
        }
    }
}

fn find_min_max(values: &Vec<u32>) -> (u32, u32) {
    let mut min = 1000;
    let mut max = 0;

    for value in values {
        if min > *value {
            min = *value;
        }
        if max < *value {
            max = *value;
        }
    }

    (min, max)
}

fn print_cups(playfield: &Vec<u32>, current_index: usize) {
    print!("cups: ");
    for i in 0..playfield.len() {
        if i == current_index {
            print!("({}) ", playfield[i]);
        } else {
            print!("{} ", playfield[i]);
        }
    }
    println!("");
}

fn print_pick(pick: &Vec<u32>) {
    print!("pick up: ");
    for value in pick {
        print!("{} ", value);
    }
    println!("");
}