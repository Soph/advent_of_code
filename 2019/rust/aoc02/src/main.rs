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

    let opcodes = read_and_parse(&args.path);

    let result1 = run(&mut opcodes.clone(), 12, 2);

    println!("Result1: {}", result1);

    let mut stop = false;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut copy = opcodes.clone();
            let result = run(&mut copy, noun, verb);
            if result == 19690720 {
                println!(
                    "Noun: {}, Verb: {} - Result2: {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                stop = true;
            }
            if stop {
                break;
            }
        }
        if stop {
            break;
        }
    }
}

fn read_and_parse(path: &str) -> Vec<u64> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

fn run(opcodes: &mut Vec<u64>, noun: u64, verb: u64) -> u64 {
    opcodes[1] = noun;
    opcodes[2] = verb;
    let mut pointer = 0;
    loop {
        match opcodes[pointer] {
            1 | 2 => {
                let pos_a = opcodes[pointer + 1] as usize;
                let pos_b = opcodes[pointer + 2] as usize;
                let pos_c = opcodes[pointer + 3] as usize;
                match opcodes[pointer] {
                    1 => opcodes[pos_c] = opcodes[pos_a] + opcodes[pos_b],
                    2 => opcodes[pos_c] = opcodes[pos_a] * opcodes[pos_b],
                    _ => (),
                }
                pointer += 4;
            }
            _ => break,
        }
    }

    opcodes[0]
}
