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
    let octopuses = read_and_parse(&args.path);

    run1(&octopuses);
    run2(&octopuses);
}

fn read_and_parse(path: &str) -> Vec<Vec<u8>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(ToOwned::to_owned)
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn run1(octopuses: &Vec<Vec<u8>>) {
    let mut playfield = octopuses.clone();
    let search_matrix: Vec<(i64, i64)> = vec![
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];
    let mut flashes = 0;
    let mut turns = 0;

    loop {
        turns += 1;
        // energy spike
        for y in 0..playfield.len() {
            for x in 0..playfield[y].len() {
                playfield[y][x] += 1;
            }
        }

        // flashes
        loop {
            let mut found = false;
            for y in 0..playfield.len() {
                for x in 0..playfield[y].len() {
                    if playfield[y][x] > 9 {
                        playfield[y][x] = 0;
                        flashes += 1;
                        found = true;
                        for (dy, dx) in &search_matrix {
                            let new_x = x as i64 + dx;
                            let new_y = y as i64 + dy;

                            if new_x >= 0
                                && new_x < playfield[y].len() as i64
                                && new_y >= 0
                                && new_y < playfield.len() as i64
                            {
                                // hasn't flashed yet?
                                if playfield[new_y as usize][new_x as usize] > 0 {
                                    playfield[new_y as usize][new_x as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
            if !found {
                break;
            }
        }
        if turns == 100 {
            break;
        }
    }

    println!("{}", flashes);
}

fn run2(octopuses: &Vec<Vec<u8>>) {
    let mut playfield = octopuses.clone();
    let search_matrix: Vec<(i64, i64)> = vec![
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];
    let mut flashes = 0;
    let mut turns = 0;

    loop {
        let mut turn_flashes = 0;
        turns += 1;
        // energy spike
        for y in 0..playfield.len() {
            for x in 0..playfield[y].len() {
                playfield[y][x] += 1;
            }
        }

        // flashes
        loop {
            let mut found = false;
            for y in 0..playfield.len() {
                for x in 0..playfield[y].len() {
                    if playfield[y][x] > 9 {
                        playfield[y][x] = 0;
                        flashes += 1;
                        turn_flashes += 1;
                        found = true;
                        for (dy, dx) in &search_matrix {
                            let new_x = x as i64 + dx;
                            let new_y = y as i64 + dy;

                            if new_x >= 0
                                && new_x < playfield[y].len() as i64
                                && new_y >= 0
                                && new_y < playfield.len() as i64
                            {
                                // hasn't flashed yet?
                                if playfield[new_y as usize][new_x as usize] > 0 {
                                    playfield[new_y as usize][new_x as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
            if !found {
                break;
            }
        }
        if turn_flashes == octopuses.len() * octopuses[0].len() {
            println!("Flash Party: {}", turns);
            break;
        }
    }

    println!("{}", flashes);
}

fn _print_playfield(playfield: &Vec<Vec<u8>>) {
    for y in 0..playfield.len() {
        for x in 0..playfield[y].len() {
            if playfield[y][x] > 9 {
                print!("X");
            } else {
                print!("{}", playfield[y][x]);
            }
        }
        println!();
    }
    println!();
}
