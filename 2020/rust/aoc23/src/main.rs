use std::fs;
use structopt::StructOpt;
use std::collections::VecDeque;
use std::time::SystemTime;

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
        part2.push_back(i);
    }

    play(&part2, min, 1000000, 10000000);
}

fn read_and_parse(path: &str) -> VecDeque<u32> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn play(start: &VecDeque<u32>, min: u32, max: u32, moves: u32) {
    let mut playfield = start.clone();
    let now = SystemTime::now();
    for i in 0..moves {
        let current_value = playfield[0];
        if i % 10000 == 0 {
            println!("{:?}", now.elapsed().unwrap());
            println!("--- Move {} ---", i+1);
        }
        //print_cups(&playfield, 0);
        let mut pick: VecDeque<u32> = playfield.drain(1..=3).collect();
        //print_pick(&pick);
        let destination = find_destination(current_value, &pick, min, max);
        //println!("destination: {}", destination);
        if playfield[playfield.len()-1] == destination {
            pick.append(&mut playfield);
            playfield = pick;
        } else {
            let insert_position = playfield.iter().position(|&x| x == destination).unwrap() + 1;
            let mut tail = playfield.split_off(insert_position);
            playfield.append(&mut pick);
            playfield.append(&mut tail);
        }
        //print_cups(&playfield, 0);
        playfield.rotate_left(1);
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

fn find_destination(value: u32, pick: &VecDeque<u32>, min: u32, max: u32) -> u32 {
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

fn find_min_max(values: &VecDeque<u32>) -> (u32, u32) {
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

fn print_cups(playfield: &VecDeque<u32>, current_index: usize) {
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

fn print_pick(pick: &VecDeque<u32>) {
    print!("pick up: ");
    for value in pick {
        print!("{} ", value);
    }
    println!("");
}