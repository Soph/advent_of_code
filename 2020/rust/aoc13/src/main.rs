use std::collections::HashSet;
use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

struct Schedule {
    arrival: u64,
    buses: Vec<u64>,
}

fn main() {
    let args = Cli::from_args();
    let schedule = read_and_parse(&args.path);

    let mut min = schedule.arrival * 10;
    let mut min_id = 0;
    for id in &schedule.buses {
        if *id == 0 {
            continue;
        }
        let time = ((schedule.arrival / id) + 1) * id;
        if time < min {
            min = time;
            min_id = *id;
        }
    }

    println!("Result 1: {}", (min - schedule.arrival) * min_id);

    let time = find_align_time(&schedule.buses);

    println!("Result 2: {}", time);
}

fn read_and_parse(path: &str) -> Schedule {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let parts: Vec<&str> = contents.split("\n").collect();
    let arrival = parts[0].parse().unwrap();
    let buses: Vec<u64> = parts[1]
        .split(",")
        .map(|s| if s == "x" { 0 } else { s.parse().unwrap() })
        .collect();

    Schedule {
        arrival: arrival,
        buses: buses,
    }
}

fn find_align_time(buses: &Vec<u64>) -> u64 {
    let filtered_buses = filtered_buses(buses);

    for bus in &filtered_buses {
        println!("{},{}", bus.0, bus.1);
    }

    let start_bus = filtered_buses[0];
    let mut used: HashSet<usize> = HashSet::new();

    let mut i = 0;
    let mut iterator = 1;
    loop {
        i += iterator;
        let test = start_bus.1 * i - start_bus.0 as u64;
        let mut found = 0;
        for bus in &filtered_buses {
            if (test + bus.0 as u64) % bus.1 != 0 {
                break;
            } else {
                if !used.contains(&bus.0) && start_bus.0 != bus.0 {
                    println!(
                        "Match {}: {}, {} - iterator: {} => {}",
                        i,
                        start_bus.1,
                        bus.1,
                        iterator,
                        (iterator * bus.1 as u64)
                    );
                    iterator *= bus.1 as u64;
                    used.insert(bus.0);
                }
                found += 1;
            }
        }
        // println!("Test failed: {}", found);

        if found == filtered_buses.len() {
        	println!("Found: {}", i);
            return test;
        }
    }
}

fn filtered_buses(buses: &Vec<u64>) -> Vec<(usize, u64)> {
    let mut mapped: Vec<(usize, u64)> = vec![];
    for i in 0..buses.len() {
        if buses[i] > 0 {
            mapped.push((i, buses[i]));
        }
    }

    let filtered: Vec<(usize, u64)> = mapped.into_iter().filter(|b| b.1 > 0).collect();
    filtered
}
