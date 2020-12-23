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
    let start = read_and_parse(&args.path);
    // let (min, max) = find_min_max(&start);

    let mut playfield: Vec<usize> = Vec::new();
    for i in 0..start.len()+1 {
        playfield.push(i + 1);
    }
    println!("{}", playfield.len());
    for i in 0..start.len()-1 {
        playfield[start[i] as usize] = start[i+1] as usize;
    }
    playfield[0] = start[0]; // start value
    playfield[start[start.len() - 1]] = start[0];
    
    for i in 0..playfield.len() {
        println!("{} => {}", i, playfield[i]);
    }
    play(&mut playfield, 100);

    // Part 2
    let mut playfield: Vec<usize> = Vec::new();
    for i in 0..=1000000 {
        playfield.push(i + 1);
    }
    for i in 0..start.len()-1 {
        playfield[start[i] as usize] = start[i+1] as usize;
    }
    playfield[0] = start[0]; // start value
    playfield[start[start.len() - 1]] = start.len() + 1;
    let len = playfield.len();
    playfield[len-1] = start[0];

    play(&mut playfield, 10_000_000);
}

fn read_and_parse(path: &str) -> Vec<usize> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
}

fn play(playfield: &mut Vec<usize>, rounds: usize) {
    for _ in 0..rounds {
        let current = playfield[0];
        let p1 = playfield[current];
        let p2 = playfield[p1];
        let p3 = playfield[p2];

        let mut destination = current;
        loop {
            if destination == 0 {
                destination = playfield.len() - 1;
            } else {
                destination -= 1;
            }
            if destination != p1 && destination != p2 && destination != p3 && destination != 0 {
                break;
            }
        }

        let destination_points_to = playfield[destination];
        playfield[current] = playfield[p3];
        playfield[0] = playfield[current];
        playfield[destination] = p1;
        playfield[p3] = destination_points_to;
    }

    if rounds == 100 {
        print_playfield(playfield);
    } else {
        let cup_a = playfield[1];
        let cup_b = playfield[cup_a];
        println!("Result2: {}*{}={}", cup_a, cup_b, cup_a * cup_b);
    }
}

fn print_playfield(playfield: &Vec<usize>) {
    println!("Result: ");
    let mut last = playfield[1];
    loop  {
        print!("{} ", last);
        last = playfield[last];
        if last == 1 {
            break;
        }
    }
    println!("");
}
