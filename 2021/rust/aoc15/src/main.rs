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

    let part2_cave = modify_cave(&cave);

    find_paths(&part2_cave)
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
    let search_matrix: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
    let mut visited: HashSet<Point> = HashSet::new();
    let mut found_risks: HashMap<Point, i32> = HashMap::new();
    let mut current_node = Point { x: 0, y: 0 };
    found_risks.insert(current_node.clone(), 0);

    loop {
        let current_risk = found_risks.get(&current_node.clone()).unwrap().clone();
        visited.insert(current_node.clone());
        found_risks.remove(&current_node); // visiting it now

        for (dy, dx) in &search_matrix {
            let new_x = current_node.x + dx;
            let new_y = current_node.y + dy;

            if new_x >= 0 && new_x < cave[0].len() as i32 && new_y >= 0 && new_y < cave.len() as i32
            {
                let node = Point { x: new_x, y: new_y };
                if !visited.contains(&node) {
                    let risk = found_risks.get(&node);
                    let new_risk = current_risk + cave[new_y as usize][new_x as usize];
                    if risk.is_none() || *risk.unwrap() > new_risk {
                        found_risks.insert(node.clone(), new_risk);
                        if node.x == cave[0].len() as i32 - 1 && node.y == cave.len() as i32 - 1 {
                            println!("{}", new_risk);
                            return;
                        }
                    }
                }
            }
        }
        let mut risk_vec: Vec<_> = found_risks.iter().collect();
        risk_vec.sort_by(|a, b| a.1.cmp(b.1));
        current_node = risk_vec[0].0.clone();
    }
}

fn modify_cave(cave: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_cave = vec![];

    for y in 0..cave.len() * 5 {
        let mut x_cave = vec![];
        for x in 0..cave[0].len() * 5 {
            let mut new_value = cave[y % cave.len()][x % cave.len()]
                + (x as i32 / cave[0].len() as i32)
                + (y as i32 / cave.len() as i32);
            if new_value > 9 {
                new_value = new_value - 9;
            }
            x_cave.push(new_value);
        }
        new_cave.push(x_cave.clone());
    }

    return new_cave;
}
