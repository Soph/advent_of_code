use std::collections::HashSet;
use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let args = Cli::from_args();
    let (points, foldings) = read_and_parse(&args.path);

    run1(&points, &foldings);
}

fn read_and_parse(path: &str) -> (Vec<Point>, Vec<(String, i32)>) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut split = contents.split("\n\n");
    let points = split
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            let mut parts = line.split(",");
            let x = parts.next().unwrap().trim().parse::<i32>().unwrap();
            let y = parts.next().unwrap().trim().parse::<i32>().unwrap();
            Point { x: x, y: y }
        })
        .collect::<Vec<Point>>();

    let foldings = split
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            let lines = line.split(" along ");
            let mut folding = lines.last().unwrap().trim().split("=");
            let coordinate = folding.next().unwrap();
            let value = folding.next().unwrap().trim().parse::<i32>().unwrap();
            (coordinate.to_string(), value)
        })
        .collect::<Vec<(String, i32)>>();

    (points, foldings)
}

fn run1(points: &Vec<Point>, foldings: &Vec<(String, i32)>) {
    let mut map: HashSet<Point> = HashSet::new();

    for point in points {
        map.insert(point.clone());
    }

    let mut count = 0;
    for folding in foldings {
        let mut new_map = HashSet::new();
        for point in map.iter() {
            let mut new_point = point.clone();
            if folding.0 == "x" && point.x > folding.1 {
                new_point.x = folding.1 * 2 - point.x;
            } else if folding.0 == "y" && point.y > folding.1 {
                new_point.y = folding.1 * 2 - point.y;
            }
            if !new_map.contains(&new_point.clone()) {
                new_map.insert(new_point);
            }
        }
        map = new_map;
        count += 1;
        println!("{}: {}", count, map.len());
    }

    pretty_print(&map);

    println!("{}", map.len());
}

fn pretty_print(map: &HashSet<Point>) {
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX;
    let mut max_y = std::i32::MIN;

    for point in map.iter() {
        min_x = std::cmp::min(min_x, point.x);
        max_x = std::cmp::max(max_x, point.x);
        min_y = std::cmp::min(min_y, point.y);
        max_y = std::cmp::max(max_y, point.y);
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.contains(&Point { x: x, y: y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
