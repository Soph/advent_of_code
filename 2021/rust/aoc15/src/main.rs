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
    let risks = find_path2(
        &cave,
        Point {
            x: 0,
            y: 0,
        },
        0,
    );
    let mut min_risk = std::u32::MAX;
    for risk in risks {
        if risk < min_risk {
            min_risk = risk;
        }
    }

    println!("Part1: {}", min_risk);
}

fn find_path(cave: &Vec<Vec<i32>>, path: &Path) -> Vec<Path> {
    let mut paths: Vec<Path> = vec![];
    let search_matrix: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 0),
    ];

    for (dy, dx) in &search_matrix {
        let new_x = path.points.last().unwrap().x + dx;
        let new_y = path.points.last().unwrap().y + dy;

        if new_x >= 0
            && new_x < cave[0].len() as i32
            && new_y >= 0
            && new_y < cave.len() as i32
        {
            let next = Point {
                x: new_x as i32,
                y: new_y as i32,
            };
            let mut new_path = path.clone();
            if new_path.points.contains(&next) {
                continue;
            }
            new_path.points.push(next.clone());
            new_path.risk += cave[new_y as usize][new_x as usize] as u32;
            if next.x + 1 == cave[0].len() as i32 && next.y + 1 == cave.len() as i32 {
                paths.push(new_path);
            } else {
                paths.append(&mut find_path(cave, &new_path));
            }
        }
    }

    return paths;
}

fn find_path2(cave: &Vec<Vec<i32>>, last: Point, risk: u32) -> Vec<u32> {
    let mut paths: Vec<u32> = vec![];
    let search_matrix: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 0),
    ];

    for (dy, dx) in &search_matrix {
        let new_x = last.x + dx;
        let new_y = last.y + dy;

        if new_x >= 0
            && new_x < cave[0].len() as i32
            && new_y >= 0
            && new_y < cave.len() as i32
        {
            let next = Point {
                x: new_x as i32,
                y: new_y as i32,
            };
            if next.x + 1 == cave[0].len() as i32 && next.y + 1 == cave.len() as i32 {
                paths.push(risk + cave[new_y as usize][new_x as usize] as u32);
            } else {
                paths.append(&mut find_path2(cave, next, risk + cave[new_y as usize][new_x as usize] as u32));
            }
        }
    }

    return paths;
}