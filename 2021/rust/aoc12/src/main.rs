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

fn main() {
    let args = Cli::from_args();
    let cave_paths = read_and_parse(&args.path);

    find_paths1(&cave_paths);
    find_paths2(&cave_paths);
}

fn read_and_parse(path: &str) -> HashMap<String, Vec<String>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut cave_paths = HashMap::new();
    for line in contents.lines() {
        let mut split = line.split("-");
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        if second != "start" {
            cave_paths
                .entry(first.to_string())
                .or_insert(vec![])
                .push(second.to_string());
        }
        if second != "end" && first != "start" {
            cave_paths
                .entry(second.to_string())
                .or_insert(vec![])
                .push(first.to_string());
        }
    }
    return cave_paths;
}

fn find_paths1(cave_paths: &HashMap<String, Vec<String>>) {
    let mut valid_paths: Vec<Vec<String>> = vec![];

    let paths = find_path(
        &cave_paths,
        "start".to_string(),
        &vec![],
        &mut HashSet::new(),
    );
    for path in paths {
        if path.last().unwrap() == "end" {
            valid_paths.push(path);
        }
    }
    println!("Part1: {}", valid_paths.len());
}

fn find_path(
    cave_paths: &HashMap<String, Vec<String>>,
    current: String,
    path: &Vec<String>,
    visited: &mut HashSet<String>,
) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = vec![];
    if current == "end" {
        let mut new_path = path.clone();
        new_path.push(current.to_string());
        paths.push(new_path.clone());
        return paths;
    }
    let next = cave_paths.get(&current);

    match next {
        Some(next_targets) => {
            for target in next_targets {
                let mut local_visited = visited.clone();
                if local_visited.contains(target) {
                    if target.to_lowercase() == *target {
                        // visited lower case letter already, not again!
                        continue;
                    }
                } else {
                    local_visited.insert(target.to_string());
                }
                let mut new_path = path.clone();
                new_path.push(current.to_string());
                for path in find_path(
                    cave_paths,
                    target.to_string(),
                    &new_path,
                    &mut local_visited,
                ) {
                    paths.push(path);
                }
            }
        }
        None => (),
    }

    return paths;
}

fn find_paths2(cave_paths: &HashMap<String, Vec<String>>) {
    let mut valid_paths: Vec<Vec<String>> = vec![];

    let paths = find_path2(
        &cave_paths,
        "start".to_string(),
        &vec![],
        &mut HashMap::new(),
    );
    for path in paths {
        if path.last().unwrap() == "end" {
            valid_paths.push(path);
        }
    }
    println!("Part2: {}", valid_paths.len());
}

fn has_two(visited: &HashMap<String, u8>) -> bool {
    for (_, count) in visited {
        if *count == 2 {
            return true;
        }
    }
    return false;
}

fn find_path2(
    cave_paths: &HashMap<String, Vec<String>>,
    current: String,
    path: &Vec<String>,
    visited: &mut HashMap<String, u8>,
) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = vec![];
    if current == "end" {
        let mut new_path = path.clone();
        new_path.push(current.to_string());
        paths.push(new_path.clone());
        return paths;
    }
    let next = cave_paths.get(&current);

    match next {
        Some(next_targets) => {
            for target in next_targets {
                let mut local_visited = visited.clone();
                if target.to_lowercase() == *target {
                    let has_two = has_two(&local_visited);
                    match local_visited.get(target) {
                        Some(count) => {
                            if *count > 0 && has_two {
                                // visited lower case letter already, not again!
                                continue;
                            }
                        }
                        None => (),
                    }
                    *local_visited.entry(target.clone()).or_insert(0) += 1;
                }
                let mut new_path = path.clone();
                new_path.push(current.to_string());
                for path in find_path2(
                    cave_paths,
                    target.to_string(),
                    &new_path,
                    &mut local_visited,
                ) {
                    paths.push(path);
                }
            }
        }
        None => (),
    }

    return paths;
}
