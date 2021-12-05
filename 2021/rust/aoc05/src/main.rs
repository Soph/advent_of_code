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

#[derive(Clone, Debug, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let args = Cli::from_args();
    let lines = read_and_parse(&args.path);

    run1(&lines);
    run2(&lines);
}

fn read_and_parse(path: &str) -> Vec<Line> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(ToOwned::to_owned)
        .map(|s| parse_line(s))
        .collect()
}

fn parse_line(line: String) -> Line {
    let points: Vec<String> = line.split(" -> ").map(ToOwned::to_owned).collect();
    let start: Vec<i64> = points[0].split(",").map(|s| s.parse().unwrap()).collect();
    let end: Vec<i64> = points[1].split(",").map(|s| s.parse().unwrap()).collect();

    return Line {
        start: Point {
            x: start[0],
            y: start[1],
        },
        end: Point {
            x: end[0],
            y: end[1],
        },
    };
}

fn run1(lines: &Vec<Line>) {
    let mut points: HashMap<Point, i64> = HashMap::new();
    let mut more_than_one: HashSet<Point> = HashSet::new();

    for line in lines {
        let dx = line.start.x - line.end.x;
        let dy = line.start.y - line.end.y;

        if dx != 0 && dy != 0 {
            // Line is not horizontal or vertical
            continue;
        }

        let mut y = line.start.y;
        let mut x = line.start.x;
        while (dy > 0 && y >= line.end.y)
            || (dy < 0 && y <= line.end.y)
            || (dx > 0 && x >= line.end.x)
            || (dx < 0 && x <= line.end.x)
        {
            let point = Point { x: x, y: y };
            points.insert(point.clone(), points.get(&point).unwrap_or(&0) + 1);
            if points[&point] > 1 {
                more_than_one.insert(point);
            }
            if dy > 0 {
                y -= 1;
            } else if dy < 0 {
                y += 1;
            }
            if dx > 0 {
                x -= 1;
            } else if dx < 0 {
                x += 1;
            }
        }
    }
    println!("Points with more then 1: {}", more_than_one.len());
}

fn run2(lines: &Vec<Line>) {
    let mut points: HashMap<Point, i64> = HashMap::new();
    let mut more_than_one: HashSet<Point> = HashSet::new();

    for line in lines {
        let dx = line.start.x - line.end.x;
        let dy = line.start.y - line.end.y;

        let mut y = line.start.y;
        let mut x = line.start.x;
        while (dy > 0 && y >= line.end.y)
            || (dy < 0 && y <= line.end.y)
            || (dx > 0 && x >= line.end.x)
            || (dx < 0 && x <= line.end.x)
        {
            let point = Point { x: x, y: y };
            points.insert(point.clone(), points.get(&point).unwrap_or(&0) + 1);
            if points[&point] > 1 {
                more_than_one.insert(point);
            }
            if dy > 0 {
                y -= 1;
            } else if dy < 0 {
                y += 1;
            }
            if dx > 0 {
                x -= 1;
            } else if dx < 0 {
                x += 1;
            }
        }
    }
    println!("Points with more then 1: {}", more_than_one.len());
}
