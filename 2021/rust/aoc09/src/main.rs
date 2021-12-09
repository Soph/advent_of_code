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
    x: i64,
    y: i64,
}

fn main() {
    let args = Cli::from_args();
    let cave = read_and_parse(&args.path);

    find1(&cave);
    find2(&cave);
}

fn read_and_parse(path: &str) -> Vec<Vec<i64>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(ToOwned::to_owned)
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i64).collect())
        .collect()
}

fn find1(cave: &Vec<Vec<i64>>) {
    let mut sum = 0;

    let low_points = find_low_points(&cave);

    for low in low_points {
        sum += cave[low.y as usize][low.x as usize] + 1;
    }
    println!("{}", sum);
}

fn find_low_points(cave: &Vec<Vec<i64>>) -> Vec<Point> {
    let search_matrix: Vec<(i64, i64)> = vec![(-1, 0), (0, 1), (0, -1), (1, 0)];
    let mut low_points: Vec<Point> = vec![];

    for y in 0..cave.len() {
        for x in 0..cave[y].len() {
            let mut count_bigger = 0;
            for (dy, dx) in &search_matrix {
                let new_x = x as i64 + dx;
                let new_y = y as i64 + dy;

                if new_x >= 0
                    && new_x < cave[y].len() as i64
                    && new_y >= 0
                    && new_y < cave.len() as i64
                {
                    if cave[new_y as usize][new_x as usize] > cave[y][x] {
                        count_bigger += 1;
                    }
                } else {
                    count_bigger += 1;
                }
            }
            if count_bigger == 4 {
                low_points.push(Point {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }

    return low_points;
}

fn find2(cave: &Vec<Vec<i64>>) {
    let low_points = find_low_points(&cave);
    let mut sum = 1;
    let mut basins = vec![];
    for low in low_points {
        let mut basin: Vec<Point> = vec![low];
        loop {
            let mut found = false;
            for point in basin.clone() {
                let neighbors = find_basin_neighbors(&cave, point);
                for neighbor in neighbors {
                    if !basin.contains(&neighbor) {
                        found = true;
                        basin.push(neighbor);
                    }
                }
            }
            if !found {
                break;
            }
        }
        basins.push(basin);
    }

    let mut lengths: Vec<usize> = basins.iter().map(|basin| basin.len()).collect();
    lengths.sort();
    lengths.iter().rev().take(3).for_each(|length| {
        sum *= length;
    });

    println!("{}", sum);
}

fn find_basin_neighbors(cave: &Vec<Vec<i64>>, point: Point) -> Vec<Point> {
    let search_matrix: Vec<(i64, i64)> = vec![(-1, 0), (0, 1), (0, -1), (1, 0)];
    let mut neighbors: Vec<Point> = vec![];

    for (dy, dx) in &search_matrix {
        let new_x = point.x as i64 + dx;
        let new_y = point.y as i64 + dy;

        if new_x >= 0
            && new_x < cave[point.y as usize].len() as i64
            && new_y >= 0
            && new_y < cave.len() as i64
        {
            if cave[new_y as usize][new_x as usize] != 9 {
                neighbors.push(Point { x: new_x, y: new_y });
            }
        }
    }

    return neighbors;
}
