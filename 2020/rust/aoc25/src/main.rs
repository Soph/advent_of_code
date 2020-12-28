use std::fs;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

fn main() {
    let args = Cli::from_args();
    let (card_public_key, door_public_key) = read_and_parse(&args.path);

    find(card_public_key, door_public_key);
}

fn read_and_parse(path: &str) -> (u64, u64) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let parts: Vec<&str> = contents.split("\n").collect();

    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

fn find_loop_size(public_key_card: u64, subject: u64) -> Option<u64> {
    let mut value: u64 = 1;
    for i in 0..10000000 {
        value *= subject;
        value = value % 20201227;
        if value == public_key_card {
            return Some(i + 1);
        }
    }
    return None;
}

fn find(public_key_card: u64, public_key_door: u64) {
    let mut subject = 0;
    loop {
        match find_loop_size(public_key_card, subject) {
            Some(loop_size_card) => match find_loop_size(public_key_door, subject) {
                Some(loop_size_door) => {
                    let a = calculate_encryption_key(public_key_door, loop_size_card);
                    let b = calculate_encryption_key(public_key_card, loop_size_door);
                    println!(
                        "{}, {}, {}: {} vs {}",
                        subject, loop_size_card, loop_size_door, a, b
                    );
                    if a == b {
                        println!("result: {}", a);
                        return;
                    }
                }
                None => (),
            },
            None => (),
        }
        subject += 1;
    }
}

fn calculate_encryption_key(public_key: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= public_key as u64;
        value = value % 20201227;
    }

    value
}
