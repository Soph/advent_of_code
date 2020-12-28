use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

fn main() {
    let args = Cli::from_args();
    let directions = read_and_parse(&args.path);

    let size = 160;

    let mut grid: Vec<Vec<bool>> = Vec::new();
    for y in 0..size {
        grid.push(Vec::new());
        for _ in 0..size {
            grid[y].push(false)
        }
    }

    play(&mut grid, &directions, size / 2);

    play2(&mut grid, 100);
}

fn read_and_parse(path: &str) -> Vec<Vec<Direction>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(|line| parse_line(&line.to_string()))
        .collect()
}

fn parse_line(line: &String) -> Vec<Direction> {
    let mut result: Vec<Direction> = Vec::new();
    let mut last: Option<char> = None;

    for c in line.chars() {
        match c {
            'e' => match last {
                None => {
                    result.push(Direction::East);
                }
                Some(x) => match x {
                    'n' => {
                        result.push(Direction::NorthEast);
                        last = None;
                    }
                    's' => {
                        result.push(Direction::SouthEast);
                        last = None;
                    }
                    _ => (),
                },
            },
            'w' => match last {
                None => {
                    result.push(Direction::West);
                }
                Some(x) => match x {
                    'n' => {
                        result.push(Direction::NorthWest);
                        last = None;
                    }
                    's' => {
                        result.push(Direction::SouthWest);
                        last = None;
                    }
                    _ => (),
                },
            },
            x => {
                last = Some(x);
            }
        }
    }

    result
}

fn play(grid: &mut Vec<Vec<bool>>, all_directions: &Vec<Vec<Direction>>, center: usize) {
    let start = (center, center);
    for directions in all_directions {
        let mut position = start;
        for direction in directions {
            match direction {
                Direction::East => position.1 += 1,
                Direction::West => position.1 -= 1,
                Direction::NorthWest => position.0 -= 1,
                Direction::SouthEast => position.0 += 1,
                Direction::NorthEast => {
                    position.0 -= 1;
                    position.1 += 1;
                }
                Direction::SouthWest => {
                    position.0 += 1;
                    position.1 -= 1;
                }
            }
            println!("{}, {}", position.1, position.0);
        }
        println!("Flipping {},{}", position.1, position.0);
        grid[position.0][position.1] = !grid[position.0][position.1];
    }

    let mut count_black = 0;
    for row in grid {
        for column in row {
            if *column {
                count_black += 1;
            }
        }
    }

    println!("Black: {}", count_black);
}

fn play2(grid: &mut Vec<Vec<bool>>, days: usize) {
    let mut working_grid = grid.clone();
    let check_directions: Vec<(i32, i32)> =
        vec![(0, 1), (0, -1), (1, 0), (-1, 0), (-1, 1), (1, -1)];
    let max = working_grid.len();

    for i in 0..days {
        let mut new_grid = working_grid.clone();
        for y in 0..max {
            for x in 0..max {
                let mut found = 0;
                for direction in &check_directions {
                    let new_x: i32 = x as i32 + direction.1 as i32;
                    let new_y: i32 = y as i32 + direction.0 as i32;
                    if new_x > 0 && new_x < max as i32 && new_y > 0 && new_y < max as i32 {
                        if working_grid[new_y as usize][new_x as usize] {
                            found += 1;
                        }
                    }
                }
                if working_grid[y][x] {
                    if found == 0 || found > 2 {
                        new_grid[y][x] = false;
                    }
                } else {
                    if found == 2 {
                        new_grid[y][x] = true;
                    }
                }
            }
        }
        let mut count_black = 0;
        for row in &new_grid {
            for column in row {
                if *column {
                    count_black += 1;
                }
            }
        }
        working_grid = new_grid.clone();

        println!("Day {} - Black: {}", i + 1, count_black);
    }
}
