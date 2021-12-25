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
    let mut seabed = read_and_parse(&args.path);

    print(&seabed);

    let mut count = 0;
    loop {
        count += 1;
        let new_seabed = move_cucumbers(&seabed);
        if new_seabed != seabed {
            seabed = new_seabed;
            println!("Count: {}", count);
            print(&seabed);
        } else {
            break;
        }
    }
}

fn move_cucumbers(seabed: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut last_seabed = seabed.clone();
    let mut new_seabed = seabed.clone();
    let max_x = seabed[0].len();
    let max_y = seabed.len();
    for i in 0..2 {
        for (y, row) in last_seabed.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                match c {
                    '>' => {
                        if i == 0 {
                            //println!("x: {} -> {}", x, (x+1)%max_x);
                            if last_seabed[y][(x + 1) % max_x] == '.' {
                                new_seabed[y][(x + 1) % max_x] = '>';
                                new_seabed[y][x] = '.';
                            }
                        }
                    }
                    'v' => {
                        if i == 1 {
                            //println!("y: {} -> {}", y, (y+1)%max_y);
                            if last_seabed[(y + 1) % max_y][x] == '.' {
                                new_seabed[(y + 1) % max_y][x] = 'v';
                                new_seabed[y][x] = '.';
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        last_seabed = new_seabed.clone();
    }
    return new_seabed;
}

fn print(seabed: &Vec<Vec<char>>) {
    for row in seabed {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn read_and_parse(path: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.split("\n").map(|s| s.chars().collect()).collect()
}
