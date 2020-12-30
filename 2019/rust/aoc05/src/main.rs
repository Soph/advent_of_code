use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

struct Instruction {
    opcode: i64,
    mode_a: i64,
    mode_b: i64,
    mode_c: i64,
}

fn main() {
    let args = Cli::from_args();

    let opcodes = read_and_parse(&args.path);

    // run1(&mut opcodes.clone(), 1);
    run2(&mut opcodes.clone(), 5);
}

fn read_and_parse(path: &str) -> Vec<i64> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

fn run1(opcodes: &mut Vec<i64>, input: i64) -> i64 {
    let mut pointer = 0;
    loop {
        let instruction = parse_instruction(opcodes[pointer]);
        match instruction.opcode {
            1 | 2 => {
                let pos_c = opcodes[pointer + 3] as usize;
                let param_a = if instruction.mode_a == 1 {
                    opcodes[pointer + 1]
                } else {
                    opcodes[opcodes[pointer + 1] as usize]
                };
                let param_b = if instruction.mode_b == 1 {
                    opcodes[pointer + 2]
                } else {
                    opcodes[opcodes[pointer + 2] as usize]
                };

                println!("{} */+ {} -> {}", param_a, param_b, pos_c);
                match instruction.opcode {
                    1 => opcodes[pos_c] = param_a + param_b,
                    2 => opcodes[pos_c] = param_a * param_b,
                    _ => (),
                }
                println!("{}", opcodes[pos_c]);
                pointer += 4;
            }
            3 => {
                let pos_a = opcodes[pointer + 1] as usize;
                opcodes[pos_a] = input;
                pointer += 2;
            }
            4 => {
                println!(
                    "Output {}: {}",
                    opcodes[pointer + 1],
                    opcodes[opcodes[pointer + 1] as usize]
                );
                pointer += 2;
            }
            _ => break,
        }
    }

    opcodes[0]
}

fn run2(opcodes: &mut Vec<i64>, input: i64) -> i64 {
    let mut pointer = 0;
    loop {
        let instruction = parse_instruction(opcodes[pointer]);
        match instruction.opcode {
            1 | 2 => {
                let pos_c = opcodes[pointer + 3] as usize;
                let param_a = if instruction.mode_a == 1 {
                    opcodes[pointer + 1]
                } else {
                    opcodes[opcodes[pointer + 1] as usize]
                };
                let param_b = if instruction.mode_b == 1 {
                    opcodes[pointer + 2]
                } else {
                    opcodes[opcodes[pointer + 2] as usize]
                };

                println!("{} */+ {} -> {}", param_a, param_b, pos_c);
                match instruction.opcode {
                    1 => opcodes[pos_c] = param_a + param_b,
                    2 => opcodes[pos_c] = param_a * param_b,
                    _ => (),
                }
                println!("{}", opcodes[pos_c]);
                pointer += 4;
            }
            3 => {
                let pos_a = opcodes[pointer + 1] as usize;
                opcodes[pos_a] = input;
                pointer += 2;
            }
            4 => {
                println!(
                    "Output {}: {}",
                    opcodes[pointer + 1],
                    opcodes[opcodes[pointer + 1] as usize]
                );
                pointer += 2;
            }
            5 | 6 => {
                let param_a = if instruction.mode_a == 1 {
                    opcodes[pointer + 1]
                } else {
                    opcodes[opcodes[pointer + 1] as usize]
                };
                let param_b = if instruction.mode_b == 1 {
                    opcodes[pointer + 2]
                } else {
                    opcodes[opcodes[pointer + 2] as usize]
                };

                match instruction.opcode {
                    5 => {
                        if param_a != 0 {
                            pointer = param_b as usize;
                        } else {
                            pointer += 3;
                        }
                    }
                    6 => {
                        if param_a == 0 {
                            pointer = param_b as usize;
                        } else {
                            pointer += 3;
                        }
                    }
                    _ => (),
                }
            }
            7 => {
                let param_a = if instruction.mode_a == 1 {
                    opcodes[pointer + 1]
                } else {
                    opcodes[opcodes[pointer + 1] as usize]
                };
                let param_b = if instruction.mode_b == 1 {
                    opcodes[pointer + 2]
                } else {
                    opcodes[opcodes[pointer + 2] as usize]
                };
                let pos_c = opcodes[pointer + 3] as usize;

                opcodes[pos_c] = if param_a < param_b { 1 } else { 0 };
                pointer += 4;
            }
            8 => {
                let param_a = if instruction.mode_a == 1 {
                    opcodes[pointer + 1]
                } else {
                    opcodes[opcodes[pointer + 1] as usize]
                };
                let param_b = if instruction.mode_b == 1 {
                    opcodes[pointer + 2]
                } else {
                    opcodes[opcodes[pointer + 2] as usize]
                };
                let pos_c = opcodes[pointer + 3] as usize;

                opcodes[pos_c] = if param_a == param_b { 1 } else { 0 };
                pointer += 4;
            }
            _ => break,
        }
    }

    opcodes[0]
}

fn parse_instruction(number: i64) -> Instruction {
    let opcode = number % 100;
    let mode_a = (number - opcode) % 1000 / 100;
    let mode_b = (number - opcode) % 10000 / 1000;
    let mode_c = if number > 10000 { 1 } else { 0 };

    println!(
        "opcode: {}, mode_a: {}, mode_b: {}, mode_c: {}",
        opcode, mode_a, mode_b, mode_c
    );
    Instruction {
        opcode: opcode,
        mode_a: mode_a,
        mode_b: mode_b,
        mode_c: mode_c,
    }
}
