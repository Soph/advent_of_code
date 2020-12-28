use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let args = Cli::from_args();

    let paths = read_and_parse(&args.path);

    run_1(&paths);
    run_2(&paths);
}

fn read_and_parse(path: &str) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(|s| s.split(",").map(|s| s.to_string()).collect())
        .collect()
}

fn run_1(paths: &Vec<Vec<String>>) {
    let mut points: HashSet<Point> = HashSet::new();
    let mut matching: HashSet<Point> = HashSet::new();
    let mut point: Point;
    for i in 0..paths.len() {
        point = Point { x: 0, y: 0 };
        let mut length = 0;
        for steps in &paths[i] {
            let value = steps[1..].to_string();
            for _ in 0..value.parse::<i64>().unwrap() {
                length += 1;
                match &steps[0..1] {
                    "R" => {
                        point.x += 1;
                    }
                    "D" => {
                        point.y -= 1;
                    }
                    "L" => {
                        point.x -= 1;
                    }
                    "U" => {
                        point.y += 1;
                    }
                    _ => (),
                }
                if i == 0 {
                    points.insert(point.clone());
                } else {
                    if points.contains(&point) {
                        matching.insert(point.clone());
                    }
                }
            }
        }
    }

    let mut min = 5000000;
    for item in matching {
        if min > item.x.abs() + item.y.abs() {
            min = item.x.abs() + item.y.abs();
        }
        println!("{},{}", item.x, item.y);
    }

    println!("Shortest Part 1: {}", min);
}

fn run_2(paths: &Vec<Vec<String>>) {
    let mut points: HashMap<Point, u32> = HashMap::new();
    let mut matching: HashMap<Point, Vec<u32>> = HashMap::new();
    let mut point: Point;
    for i in 0..paths.len() {
        point = Point { x: 0, y: 0 };
        let mut length = 0;
        for steps in &paths[i] {
            let value = steps[1..].to_string();
            for _ in 0..value.parse::<i64>().unwrap() {
                length += 1;
                match &steps[0..1] {
                    "R" => {
                        point.x += 1;
                    }
                    "D" => {
                        point.y -= 1;
                    }
                    "L" => {
                        point.x -= 1;
                    }
                    "U" => {
                        point.y += 1;
                    }
                    _ => (),
                }
                if i == 0 {
                    points.entry(point.clone()).or_insert(length);
                } else {
                    if points.contains_key(&point) {
                        let hit = matching.entry(point.clone()).or_insert(Vec::new());
                        hit.push(*points.entry(point.clone()).or_insert(0));
                        hit.push(length);
                    }
                }
            }
        }
    }

    let mut min = 5000000;
    for (item, value) in matching {
        if min > value[0] + value[1] {
            min = value[0] + value[1];
        }
        println!("{},{}", item.x, item.y);
    }

    println!("Shortest Part2: {}", min);
}
