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

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
struct Path {
    points: Vec<Point>,
    risk: u32,
}

fn main() {
    let args = Cli::from_args();
    let cave = read_and_parse(&args.path);

    find_paths(&cave);
}

fn read_and_parse(path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(ToOwned::to_owned)
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect()
}

fn find_paths(cave: &Vec<Vec<i32>>) {
    let search_matrix: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut unvisited: HashSet<Point> = HashSet::new();
    let mut total_risks: HashMap<Point, i32> = HashMap::new();
    for y in 0..cave.len() {
        for x in 0..cave[0].len() {
            if x == 0 && y == 0 {
                continue;
            }
            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            unvisited.insert(point.clone());
            total_risks.insert(point.clone(), std::i32::MAX);
        }
    }

    let mut current_node = Point { x: 0, y: 0 };
    total_risks.insert(current_node.clone(), 0);

    loop {
        let current_risk = total_risks.get(&current_node).unwrap().clone();
        unvisited.remove(&current_node);
        for (dy, dx) in &search_matrix {
            let new_x = current_node.x + dx;
            let new_y = current_node.y + dy;

            if new_x >= 0 && new_x < cave[0].len() as i32 && new_y >= 0 && new_y < cave.len() as i32
            {
                let node = Point { x: new_x, y: new_y };
                if unvisited.contains(&node) {
                    let risk = *total_risks.get(&node).unwrap();
                    let new_risk = current_risk + cave[new_x as usize][new_y as usize];
                    if risk > new_risk {
                        total_risks.insert(node.clone(), new_risk);
                        if node.x == cave[0].len() as i32 - 1 && node.y == cave.len() as i32 - 1 {
                            println!("{}", new_risk);
                            return;
                        }
                    }
                }
            }
        }
        println!("{}", unvisited.len());
        current_node = find_min_unvisited(&unvisited, &total_risks);
    }
}

fn find_min_unvisited(unvisited: &HashSet<Point>, total_risks: &HashMap<Point, i32>) -> Point {
    let mut min_risk = std::i32::MAX;
    let mut min_node = Point { x: 0, y: 0 };
    for node in unvisited {
        let risk = total_risks.get(&node).unwrap().clone();
        if risk < min_risk {
            min_risk = risk;
            min_node = node.clone();
        }
    }

    return min_node;
}
