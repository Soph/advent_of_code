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
    let instructions = read_and_parse(&args.path);

    let mut inp_blocks: Vec<Vec<String>> = vec![];
    let mut set: isize = -1;
    for instruction in &instructions {
        let mut parts = instruction.split(" ");
        let command = parts.next().unwrap();
        if command == "inp" {
            set += 1;
            inp_blocks.push(vec![]);
        }
        inp_blocks[set as usize].push(instruction.to_owned());
    }

    let mut inputs = vec![];
    inputs.push(HashSet::new());
    inputs[0].insert((0, 0, 0, 0));
    let mut outputs: Vec<HashSet<(i64, i64, i64, i64)>> = vec![];

    // run 1-9 gather outputs
    // take outputs, make them inputs to second block, run 1-9 on each, gather outputs
    // do till end, take outputs that have w == 0
    // back track :thinking:
    for digit in 0..14 {
        outputs.push(HashSet::new());
        for i in 1..=9 {
            let temp_input_string = format!("{}", char::from_digit(i, 10).unwrap());
            for input in &inputs[digit] {
                let output = run1(&inp_blocks[digit], &temp_input_string, input.clone());
                if digit == 13 {
                    // only care about those with z == 0
                    if output.3 == 0 {
                        outputs[digit].insert((0, output.1, output.2, output.3));
                    }
                } else {
                    outputs[digit].insert((0, output.1, output.2, output.3));
                }
            }
        }
        println!("{}: {}", digit, outputs[digit].len());
        inputs.push(outputs[digit].clone());
    }

    for output in &outputs[13] {
        if output.3 == 0 {
            println!("{:?}", output);
        }
    }

    let mut valid_digits: Vec<HashSet<u8>> = vec![HashSet::new(); 14];
    let mut valid_outputs: Vec<Vec<(i64, i64, i64, i64)>> = vec![vec![]; 14];
    let mut valid_inputs: Vec<Vec<(i64, i64, i64, i64)>> = vec![vec![]; 14];

    for output in outputs[13].clone() {
        valid_outputs[13].push(output);
    }

    for digit in (0..14).rev() {
        for i in 1..=9 {
            let temp_input_string = format!("{}", char::from_digit(i, 10).unwrap());
            println!("Checking: {} - {}", digit, temp_input_string);
            for input in &inputs[digit] {
                let output = run1(&inp_blocks[digit], &temp_input_string, input.clone());
                for valid_output in &valid_outputs[digit] {
                    // since w will be overwritten in first step, we don't care about it's value
                    if valid_output.1 == output.1
                        && valid_output.2 == output.2
                        && valid_output.3 == output.3
                    {
                        valid_digits[digit].insert(i as u8);
                        valid_inputs[digit].push(input.clone());
                    }
                }
            }
        }
        println!("{}: {}", digit, valid_inputs[digit].len());
        if digit > 0 {
            valid_outputs[digit - 1] = valid_inputs[digit].clone();
        }
    }

    let mut valid_digits_vec: Vec<Vec<u8>> = vec![vec![]; 14];
    for digit in 0..14 {
        valid_digits_vec[digit] = valid_digits[digit].clone().into_iter().collect();
        valid_digits_vec[digit].sort_by(|a, b| b.cmp(a));
    }

    for n in 0..valid_digits_vec.len() {
        valid_digits_vec[n].sort_by(|a, b| b.cmp(a));
    }
    find_first(&instructions, &valid_digits_vec);

    for n in 0..valid_digits_vec.len() {
        valid_digits_vec[n].sort_by(|a, b| a.cmp(b));
    }
    find_first(&instructions, &valid_digits_vec);
}

fn read_and_parse(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.split("\n").map(|s| s.to_owned()).collect()
}

fn find_first(instructions: &Vec<String>, potential_numbers: &Vec<Vec<u8>>) {
    loop {
        for a in 0..potential_numbers[0].len() {
            for b in 0..potential_numbers[0].len() {
                for c in 0..potential_numbers[0].len() {
                    for d in 0..potential_numbers[0].len() {
                        for e in 0..potential_numbers[0].len() {
                            for f in 0..potential_numbers[0].len() {
                                for g in 0..potential_numbers[0].len() {
                                    for h in 0..potential_numbers[0].len() {
                                        for i in 0..potential_numbers[0].len() {
                                            for j in 0..potential_numbers[0].len() {
                                                for k in 0..potential_numbers[0].len() {
                                                    for l in 0..potential_numbers[0].len() {
                                                        for m in 0..potential_numbers[0].len() {
                                                            for n in 0..potential_numbers[0].len() {
                                                                let mut number = "".to_owned();
                                                                number.push_str(
                                                                    potential_numbers[0][a]
                                                                        .to_string()
                                                                        .as_str(),
                                                                );
                                                                number.push_str(
                                                                    potential_numbers[1][b]
                                                                        .to_string()
                                                                        .as_str(),
                                                                );
                                                                number.push_str(
                                                                    potential_numbers[2][c]
                                                                        .to_string()
                                                                        .as_str(),
                                                                );
                                                                number.push_str(
                                                                    potential_numbers[3][d]
                                                                        .to_string()
                                                                        .as_str(),
                                                                );
                                                                number.push_str(
                                                                    potential_numbers[4][e]
                                                                        .to_string()
                                                                        .as_str(),
                                                                );
                                                                number.push_str(
                                                                    potential_numbers[5][f]
                                                                        .to_string()
                                                                        .as_str(),
                                                                );
                                                                number.push_str(
                                                                    potential_numbers[6][g]
                                                                        .to_string()
                                                                        .as_str(),
                                                                );
                                                                number.push_str(
                                                                    potential_numbers[7][h]
                                                                        .to_string()
                                                                        .as_str(),
                                                                );
                                                                number.push_str(
                                                                    potential_numbers[8][i]
                                                                        .to_string()
                                                                        .as_str(),
                                                                );
                                                                number.push_str(
                                                                    potential_numbers[9][j]
                                                                        .to_string()
                                                                        .as_str(),
                                                                );
                                                                number.push_str(
                                                                    potential_numbers[10][k]
                                                                        .to_string()
                                                                        .as_str(),
                                                                );
                                                                number.push_str(
                                                                    potential_numbers[11][l]
                                                                        .to_string()
                                                                        .as_str(),
                                                                );
                                                                number.push_str(
                                                                    potential_numbers[12][m]
                                                                        .to_string()
                                                                        .as_str(),
                                                                );
                                                                number.push_str(
                                                                    potential_numbers[13][n]
                                                                        .to_string()
                                                                        .as_str(),
                                                                );
                                                                let output = run1(
                                                                    &instructions,
                                                                    &number,
                                                                    (0, 0, 0, 0),
                                                                );
                                                                if output.3 == 0 {
                                                                    println!("Works! {}", number);
                                                                    return;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn run1(
    instructions: &Vec<String>,
    input: &String,
    state: (i64, i64, i64, i64),
) -> (i64, i64, i64, i64) {
    //println!("{} -> {} ", instructions.concat(), input);
    let mut w = state.0;
    let mut x = state.1;
    let mut y = state.2;
    let mut z = state.3;
    let mut input_chars = input.chars().collect::<Vec<char>>();

    for instruction in instructions {
        let mut parts = instruction.split(" ");
        let command = parts.next().unwrap();
        let param_a = parts.next().unwrap();
        match command {
            "inp" => match param_a {
                "w" => {
                    w = input_chars.remove(0).to_digit(10).unwrap() as i64;
                }
                "x" => {
                    x = input_chars.remove(0).to_digit(10).unwrap() as i64;
                }
                "y" => {
                    y = input_chars.remove(0).to_digit(10).unwrap() as i64;
                }
                "z" => {
                    z = input_chars.remove(0).to_digit(10).unwrap() as i64;
                }
                _ => {}
            },
            "add" => {
                let param_b = parts.next().unwrap();
                let mut right_value: i64 = 0;
                if param_b.parse::<i64>().is_ok() {
                    right_value = param_b.parse::<i64>().unwrap();
                } else {
                    match param_b {
                        "w" => {
                            right_value = w;
                        }
                        "x" => {
                            right_value = x;
                        }
                        "y" => {
                            right_value = y;
                        }
                        "z" => {
                            right_value = z;
                        }
                        _ => {}
                    }
                }
                match param_a {
                    "w" => {
                        w = w + right_value;
                    }
                    "x" => {
                        x = x + right_value;
                    }
                    "y" => {
                        y = y + right_value;
                    }
                    "z" => {
                        z = z + right_value;
                    }
                    _ => {}
                }
            }
            "mul" => {
                let param_b = parts.next().unwrap();
                let mut right_value: i64 = 0;
                if param_b.parse::<i64>().is_ok() {
                    right_value = param_b.parse::<i64>().unwrap();
                } else {
                    match param_b {
                        "w" => {
                            right_value = w;
                        }
                        "x" => {
                            right_value = x;
                        }
                        "y" => {
                            right_value = y;
                        }
                        "z" => {
                            right_value = z;
                        }
                        _ => {}
                    }
                }
                match param_a {
                    "w" => {
                        w = w * right_value;
                    }
                    "x" => {
                        x = x * right_value;
                    }
                    "y" => {
                        y = y * right_value;
                    }
                    "z" => {
                        z = z * right_value;
                    }
                    _ => {}
                }
            }
            "div" => {
                let param_b = parts.next().unwrap();
                let mut right_value: i64 = 0;
                if param_b.parse::<i64>().is_ok() {
                    right_value = param_b.parse::<i64>().unwrap();
                } else {
                    match param_b {
                        "w" => {
                            right_value = w;
                        }
                        "x" => {
                            right_value = x;
                        }
                        "y" => {
                            right_value = y;
                        }
                        "z" => {
                            right_value = z;
                        }
                        _ => {}
                    }
                }
                match param_a {
                    "w" => {
                        w = w / right_value;
                    }
                    "x" => {
                        x = x / right_value;
                    }
                    "y" => {
                        y = y / right_value;
                    }
                    "z" => {
                        z = z / right_value;
                    }
                    _ => {}
                }
            }
            "mod" => {
                let param_b = parts.next().unwrap();
                let mut right_value: i64 = 0;
                if param_b.parse::<i64>().is_ok() {
                    right_value = param_b.parse::<i64>().unwrap();
                } else {
                    match param_b {
                        "w" => {
                            right_value = w;
                        }
                        "x" => {
                            right_value = x;
                        }
                        "y" => {
                            right_value = y;
                        }
                        "z" => {
                            right_value = z;
                        }
                        _ => {}
                    }
                }
                match param_a {
                    "w" => {
                        w = w % right_value;
                    }
                    "x" => {
                        x = x % right_value;
                    }
                    "y" => {
                        y = y % right_value;
                    }
                    "z" => {
                        z = z % right_value;
                    }
                    _ => {}
                }
            }
            "eql" => {
                let param_b = parts.next().unwrap();
                let mut right_value: i64 = 0;
                if param_b.parse::<i64>().is_ok() {
                    right_value = param_b.parse::<i64>().unwrap();
                } else {
                    match param_b {
                        "w" => {
                            right_value = w;
                        }
                        "x" => {
                            right_value = x;
                        }
                        "y" => {
                            right_value = y;
                        }
                        "z" => {
                            right_value = z;
                        }
                        _ => {}
                    }
                }
                match param_a {
                    "w" => {
                        w = if w == right_value { 1 } else { 0 };
                    }
                    "x" => {
                        x = if x == right_value { 1 } else { 0 };
                    }
                    "y" => {
                        y = if y == right_value { 1 } else { 0 };
                    }
                    "z" => {
                        z = if z == right_value { 1 } else { 0 };
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    (w, x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let instructions = vec!["inp x".to_owned(), "mul x -1".to_owned()];
        let (w, x, y, z) = run1(&instructions, "8".to_owned());

        assert_eq!(x, -8);

        let instructions = vec![
            "inp z".to_owned(),
            "inp x".to_owned(),
            "mul z 3".to_owned(),
            "eql z x".to_owned(),
        ];
        let (w, x, y, z) = run1(&instructions, "13".to_owned());

        assert_eq!(z, 1);

        let instructions = vec![
            "inp w".to_owned(),
            "add z w".to_owned(),
            "mod z 2".to_owned(),
            "div w 2".to_owned(),
            "add y w".to_owned(),
            "mod y 2".to_owned(),
            "div w 2".to_owned(),
            "add x w".to_owned(),
            "mod x 2".to_owned(),
            "div w 2".to_owned(),
            "mod w 2".to_owned(),
        ];
        let (w, x, y, z) = run1(&instructions, "9".to_owned());

        assert_eq!(w, 1);
        assert_eq!(x, 0);
        assert_eq!(y, 0);
        assert_eq!(z, 1);
    }
}
