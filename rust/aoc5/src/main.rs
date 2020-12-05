use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

struct Seat {
    row: u8,
    column: u8,
    id: u16,
}

const PLANE_ROWS: u8 = 127;
const ROW_COLUMNS: u8 = 7;

fn main() {
    let args = Cli::from_args();
    let mut boarding_passes = read_and_parse(&args.path);
    boarding_passes.sort_by(|a, b| b.id.cmp(&a.id));
    let max_id = boarding_passes[0].id;

    println!("max id: {}", max_id);

    let seat_id = find_seat_id(&boarding_passes);

    println!("Free seat id: {}", seat_id);

    let seat = seat_for_seat_id(&boarding_passes, seat_id);
    println!(
        "Free seat row: {}, column: {}, id: {}",
        seat.row, seat.column, seat.id
    );
}

fn read_and_parse(path: &str) -> Vec<Seat> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(|s| parse_boarding_pass(s))
        .collect()
}

// Initial solution to part 1, but while doing second part I had to sort, then no need to search
#[allow(dead_code)]
fn find_highest(boarding_passes: &Vec<Seat>) -> u16 {
    let mut max = 0;
    for pass in boarding_passes {
        if pass.id > max {
            max = pass.id;
        }
    }

    max
}

// just out of curiosity
fn seat_for_seat_id(boarding_passes: &Vec<Seat>, seat_id: u16) -> Seat {
    let next_seat = boarding_passes
        .into_iter()
        .find(|&x| x.id == seat_id + 1)
        .unwrap();

    match next_seat.column {
        0 => Seat {
            row: next_seat.row - 1,
            column: 7,
            id: next_seat.id - 1,
        },
        _ => Seat {
            row: next_seat.row,
            column: next_seat.column - 1,
            id: next_seat.id - 1,
        },
    }
}

fn find_seat_id(boarding_passes: &Vec<Seat>) -> u16 {
    let max = boarding_passes.len() - 2; // skip last row, can't be it
    for i in 1..max {
        // gap between current and next is more then 1
        if boarding_passes[i].id - boarding_passes[i + 1].id > 1 {
            return boarding_passes[i].id - 1;
        }
    }

    0
}

fn parse_boarding_pass(boarding_pass: &str) -> Seat {
    let rows_string = &boarding_pass[..7];
    let column_string = &boarding_pass[7..10];

    let column = parse_column(column_string);
    let row = parse_row(rows_string);
    let id: u16 = row as u16 * 8 + column as u16;

    Seat {
        row: row,
        column: column,
        id: id,
    }
}

fn parse_column(column_string: &str) -> u8 {
    let mut column = 0;
    let mut section = ROW_COLUMNS;
    for c in column_string.chars() {
        section -= section / 2;
        if c == 'R' {
            column += section;
        }
    }
    column
}

fn parse_row(row_string: &str) -> u8 {
    let mut row = 0;
    let mut section = PLANE_ROWS;
    for c in row_string.chars() {
        section -= section / 2;
        if c == 'B' {
            row += section;
        }
    }
    row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let boarding_pass = String::from("BFFFBBFRRR");
        let rows_string = &boarding_pass[..7];
        let seat_string = &boarding_pass[7..10];
        assert_eq!(rows_string, "BFFFBBF");
        assert_eq!(seat_string, "RRR");
    }

    #[test]
    fn test_parse_row() {
        assert_eq!(parse_row(&"FBFBBFF"), 44);
        assert_eq!(parse_row(&"BFFFBBF"), 70);
        assert_eq!(parse_row(&"FFFBBBF"), 14);
        assert_eq!(parse_row(&"BBFFBBF"), 102);
    }

    #[test]
    fn test_parse_column() {
        assert_eq!(parse_column(&"RLR"), 5);
        assert_eq!(parse_column(&"RRR"), 7);
        assert_eq!(parse_column(&"RLL"), 4);
    }

    #[test]
    fn test_parse_boarding_pass() {
        assert_eq!(parse_boarding_pass(&"FBFBBFFRLR").row, 44);
        assert_eq!(parse_boarding_pass(&"FBFBBFFRLR").column, 5);
        assert_eq!(parse_boarding_pass(&"FBFBBFFRLR").id, 357);
    }
}
