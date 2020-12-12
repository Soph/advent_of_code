use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

struct Action {
    name: char,
    value: u32,
}

fn main() {
    let args = Cli::from_args();
    let directions = read_and_parse(&args.path);

    // Part 1
    println!("Part 1:");
    let position = move_ship(&directions);
    print_position(position);
    println!(
        "Manhatten Distance: {}",
        position.0.abs() + position.1.abs()
    );

    // Part 2
    println!("");
    println!("Part 2:");
    let waypoint = (10, 1);
    let position2 = handle_directions(&directions, waypoint);
    print_position(position2);
    println!(
        "Manhatten Distance: {}",
        position2.0.abs() + position2.1.abs()
    );
}

// Parsing
fn read_and_parse(path: &str) -> Vec<Action> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(ToOwned::to_owned)
        .map(|s| parse_action(&s))
        .collect()
}

fn parse_action(action: &String) -> Action {
    let mut mutable_action = action.clone();
    let value = mutable_action.split_off(1);
    let name = mutable_action.chars().next().unwrap();

    Action {
        name: name,
        value: value.parse().unwrap(),
    }
}

// Part 1
fn move_ship(directions: &Vec<Action>) -> (i32, i32) {
    let mut heading = 'E';
    let mut position: (i32, i32) = (0, 0);

    for direction in directions {
        match direction.name {
            'L' | 'R' => {
                heading = turn_ship(heading, direction);
            }
            'F' => {
                position = update_position(position, heading, direction.value as i32);
            }
            _ => {
                position = update_position(position, direction.name, direction.value as i32);
            }
        }
    }

    position
}

fn update_position(position: (i32, i32), heading: char, value: i32) -> (i32, i32) {
    match heading {
        'N' => (position.0, position.1 + value),
        'S' => (position.0, position.1 - value),
        'E' => (position.0 + value, position.1),
        'W' => (position.0 - value, position.1),
        _ => position,
    }
}

fn turn_ship(heading: char, action: &Action) -> char {
    let headings = vec!['N', 'E', 'S', 'W'];
    let current = headings.iter().position(|c| *c == heading).unwrap();

    let modifier = match action.name {
        'L' => 360 - action.value,
        'R' => action.value,
        _ => 0,
    };

    let new_index = (current as u32 + modifier / 90) % 4;
    headings[new_index as usize]
}

// Part 2
fn handle_directions(directions: &Vec<Action>, waypoint: (i32, i32)) -> (i32, i32) {
    let mut mut_waypoint = waypoint.clone();
    let mut position: (i32, i32) = (0, 0);

    for direction in directions {
        match direction.name {
            'L' | 'R' => {
                mut_waypoint = rotate_waypoint(mut_waypoint, direction);
            }
            'F' => {
                position = move_ship_by_waypoint(position, mut_waypoint, direction.value as i32);
            }
            _ => {
                mut_waypoint =
                    update_position(mut_waypoint, direction.name, direction.value as i32);
            }
        }
    }

    position
}

fn rotate_waypoint(waypoint: (i32, i32), action: &Action) -> (i32, i32) {
    let modifier = match action.name {
        'L' => 360 - action.value,
        'R' => action.value,
        _ => 0,
    };
    match modifier {
        90 => (waypoint.1, -waypoint.0),
        180 => (-waypoint.0, -waypoint.1),
        270 => (-waypoint.1, waypoint.0),
        _ => waypoint,
    }
}

fn move_ship_by_waypoint(position: (i32, i32), waypoint: (i32, i32), times: i32) -> (i32, i32) {
    (
        position.0 + waypoint.0 * times,
        position.1 + waypoint.1 * times,
    )
}

// Misc

fn print_position(position: (i32, i32)) {
    if position.1 > 0 {
        print!("N: {} ", position.1);
    } else {
        print!("S: {} ", position.1.abs());
    }
    if position.0 > 0 {
        print!("E: {} ", position.0);
    } else {
        print!("W: {} ", position.0.abs());
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_ship() {
        assert_eq!(
            turn_ship(
                'N',
                &Action {
                    name: 'L',
                    value: 90
                }
            ),
            'W'
        );
        assert_eq!(
            turn_ship(
                'N',
                &Action {
                    name: 'R',
                    value: 90
                }
            ),
            'E'
        );
        assert_eq!(
            turn_ship(
                'N',
                &Action {
                    name: 'L',
                    value: 180
                }
            ),
            'S'
        );
        assert_eq!(
            turn_ship(
                'N',
                &Action {
                    name: 'R',
                    value: 180
                }
            ),
            'S'
        );
        assert_eq!(
            turn_ship(
                'E',
                &Action {
                    name: 'L',
                    value: 90
                }
            ),
            'N'
        );
        assert_eq!(
            turn_ship(
                'E',
                &Action {
                    name: 'R',
                    value: 90
                }
            ),
            'S'
        );
        assert_eq!(
            turn_ship(
                'W',
                &Action {
                    name: 'L',
                    value: 180
                }
            ),
            'E'
        );
        assert_eq!(
            turn_ship(
                'W',
                &Action {
                    name: 'R',
                    value: 180
                }
            ),
            'E'
        );
    }
}
