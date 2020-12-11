use std::cmp;
use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

const NO_SEAT: u8 = 0;
const EMPTY_SEAT: u8 = 1;
const FULL_SEAT: u8 = 2;

fn main() {
    let args = Cli::from_args();
    let seating = read_and_parse(&args.path);

    print_plan(&seating);
    println!("START");

    let count1 = do_all_seating(&seating, 4, &occupied_seats_count1);
    let count2 = do_all_seating(&seating, 5, &occupied_seats_count3);

    println!("Count 1 Occupied Seats: {}", count1);
    println!("Count 2 Occupied Seats: {}", count2);
}

fn read_and_parse(path: &str) -> Vec<Vec<u8>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(ToOwned::to_owned)
        .map(|s| {
            s.chars()
                .map(|s| match s {
                    'L' => EMPTY_SEAT,
                    '#' => FULL_SEAT,
                    _ => NO_SEAT,
                })
                .collect()
        })
        .collect()
}

fn do_all_seating(
    seating: &Vec<Vec<u8>>,
    max_seats: u8,
    occupied_seats_count: &dyn Fn(&Vec<Vec<u8>>, usize, usize) -> u8,
) -> u64 {
    let mut iterations = 0;
    let mut used_seating = seating.clone();

    loop {
        let result = do_seating(&used_seating, max_seats, occupied_seats_count);

        if !result.1 {
            break;
        }
        println!("Iteration: {}", iterations);
        iterations += 1;
        used_seating = result.0.clone();
        print_plan(&used_seating);
    }

    count_full_seats(&used_seating)
}

fn do_seating(
    seating: &Vec<Vec<u8>>,
    max_seats: u8,
    occupied_seats_count: &dyn Fn(&Vec<Vec<u8>>, usize, usize) -> u8,
) -> (Vec<Vec<u8>>, bool) {
    let mut changed: bool = false;
    let mut new_seating: Vec<Vec<u8>> = seating.clone();

    for y in 0..seating.len() {
        for x in 0..seating[y].len() {
            let occupied_seats = match seating[y][x] {
                NO_SEAT => 0,
                _ => occupied_seats_count(seating, x, y),
            };
            print!("{}", occupied_seats);
            match seating[y][x] {
                FULL_SEAT => {
                    if occupied_seats >= max_seats {
                        changed = true;
                        new_seating[y][x] = EMPTY_SEAT;
                    }
                }
                EMPTY_SEAT => {
                    if occupied_seats == 0 {
                        changed = true;
                        new_seating[y][x] = FULL_SEAT;
                    }
                }
                _ => (),
            }
        }
        println!("");
    }

    (new_seating, changed)
}

fn occupied_seats_count1(seats: &Vec<Vec<u8>>, cur_x: usize, cur_y: usize) -> u8 {
    // println!("Window: {},{} -> {},{}", min_x, min_y, max_x, max_y);
    // println!("seats_max: {},{} window_min: {},{} window_max: {},{}", seats.len(), seats[0].len(), min_y, min_x, max_y, max_x);
    let mut count = 0;
    let min_y = if cur_y < 1 { 0 } else { cur_y - 1 };
    let min_x = if cur_x < 1 { 0 } else { cur_x - 1 };

    for y in min_y..=(cur_y + 1) {
        for x in min_x..=(cur_x + 1) {
            if (x, y) == (cur_x, cur_y) {
                continue;
            }
            match seats.get(y) {
                Some(row) => match row.get(x) {
                    Some(seat) => {
                        if *seat == FULL_SEAT {
                            count += 1;
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    count
}

fn occupied_seats_count3(seats: &Vec<Vec<u8>>, cur_x: usize, cur_y: usize) -> u8 {
    let search_matrix: Vec<(i16, i16)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (0, -1),
        (1, 0),
        (1, 1),
        (1, -1),
    ];
    let max_search = cmp::max(seats.len(), seats[0].len());
    let mut count = 0;

    //println!("Point: {},{}", cur_x, cur_y);
    for direction in search_matrix {
        //println!("Searching now with direction: {},{}", direction.0, direction.1);
        for i in 1..max_search {
            let x = cur_x as i16 + direction.0 * i as i16;
            let y = cur_y as i16 + direction.1 * i as i16;
            //println!("{},{}",x,y);
            if x < 0 || y < 0 {
                break;
            }
            match seats.get(y as usize) {
                Some(row) => {
                    match row.get(x as usize) {
                        Some(seat) => {
                            match *seat {
                                FULL_SEAT => {
                                    //println!("Found Full Seat");
                                    count += 1;
                                    break;
                                }
                                EMPTY_SEAT => {
                                    //println!("Found Empty Seat");
                                    break;
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }

    count
}

fn count_full_seats(seating: &Vec<Vec<u8>>) -> u64 {
    let mut count = 0;

    for row in seating {
        for seat in row {
            if *seat == FULL_SEAT {
                count += 1;
            }
        }
    }

    count
}

fn print_plan(seats: &Vec<Vec<u8>>) {
    for rows in seats {
        for seat in rows {
            print!(
                "{}",
                match *seat {
                    FULL_SEAT => "#",
                    EMPTY_SEAT => "L",
                    _ => ".",
                }
            );
        }
        println!("");
    }
}
