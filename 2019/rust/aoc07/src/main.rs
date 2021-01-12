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
    
    calc1(&mut opcodes.clone());
    calc2(&mut opcodes.clone());
}

fn read_and_parse(path: &str) -> Vec<i64> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect()
}

fn calc1(opcodes: &mut Vec<i64>) {
    let mut max = 0;
    for a in 0..=4 {
        for b in 0..=4 {
            if a == b {
                continue;
            }
            for c in 0..=4 {
                if c == a || c == b {
                    continue;
                }
                for d in 0..=4 {
                    if d == a || d == b || d == c{
                        continue;
                    }
                    for e in 0..=4 {
                        if e == a || e == b || e == c || e == d{
                            continue;
                        }
                        println!("Testing: {},{},{},{},{}", a, b, c, d, e);
                        let result_a = run(&mut opcodes.clone(), &mut 0, vec!(a, 0));
                        let result_b = run(&mut opcodes.clone(), &mut 0, vec!(b, result_a.1));
                        let result_c = run(&mut opcodes.clone(), &mut 0, vec!(c, result_b.1));
                        let result_d = run(&mut opcodes.clone(), &mut 0, vec!(d, result_c.1));
                        let result_e = run(&mut opcodes.clone(), &mut 0, vec!(e, result_d.1));
                        if result_e.1 > max {
                            max = result_e.1;
                        }
                    }
                }
            }
        }
    }
    
    println!("Part 1 Result: {}", max);
}

fn calc2(opcodes: &mut Vec<i64>) {
    let mut max = 0;
    for a in 5..=9 {
        for b in 5..=9 {
            if a == b {
                continue;
            }
            for c in 5..=9 {
                if c == a || c == b {
                    continue;
                }
                for d in 5..=9 {
                    if d == a || d == b || d == c{
                        continue;
                    }
                    for e in 5..=9 {
                        if e == a || e == b || e == c || e == d{
                            continue;
                        }
                        println!("Testing: {},{},{},{},{}", a, b, c, d, e);
                        let mut opcodes_a = opcodes.clone();
                        let mut opcodes_b = opcodes.clone();
                        let mut opcodes_c = opcodes.clone();
                        let mut opcodes_d = opcodes.clone();
                        let mut opcodes_e = opcodes.clone();
                        let mut pointer_a = 0;
                        let mut pointer_b = 0;
                        let mut pointer_c = 0;
                        let mut pointer_d = 0;
                        let mut pointer_e = 0;
                        let mut result_a;
                        let mut result_b;
                        let mut result_c;
                        let mut result_d;
                        let mut result_e = (0, 0);
                        let mut last_output = 0;
                                                            
                        result_a = run(&mut opcodes_a, &mut pointer_a, vec!(a, result_e.1));
                        result_b = run(&mut opcodes_b, &mut pointer_b, vec!(b, result_a.1));
                        result_c = run(&mut opcodes_c, &mut pointer_c, vec!(c, result_b.1));
                        result_d = run(&mut opcodes_d, &mut pointer_d, vec!(d, result_c.1));
                        result_e = run(&mut opcodes_e, &mut pointer_e, vec!(e, result_d.1));
                        loop {
                            result_a = run(&mut opcodes_a, &mut pointer_a, vec!(result_e.1));
                            if result_a.0 == 99  {
                                break;
                            } else {
                                last_output = result_a.1;
                            }
                            result_b = run(&mut opcodes_b, &mut pointer_b, vec!(result_a.1));
                            if result_b.0 == 99 {
                                break;
                            } else {
                                last_output = result_b.1;
                            }
                            result_c = run(&mut opcodes_c, &mut pointer_c, vec!(result_b.1));
                            if result_c.0 == 99 {
                                break;
                            } else {
                                last_output = result_c.1;
                            }
                            result_d = run(&mut opcodes_d, &mut pointer_d, vec!(result_c.1));
                            if result_d.0 == 99 {
                                break;
                            } else {
                                last_output = result_d.1;
                            }
                            result_e = run(&mut opcodes_e, &mut pointer_e, vec!(result_d.1));
                            if result_e.0 == 99 {
                                break;
                            } else {
                                last_output = result_e.1;
                            }
                        }
                        if last_output > max {
                            max = last_output;
                        }
                    }
                }
            }
        }
    }
    
    println!("Part 2 Result: {}", max);
}

fn run(opcodes: &mut Vec<i64>, pointer: &mut usize, input: Vec<i64>) -> (i64, i64) {
    let mut input_index = 0;
    loop {
        let instruction = parse_instruction(opcodes[*pointer]);
        match instruction.opcode {
            1 | 2 => {
                let pos_c = opcodes[*pointer + 3] as usize;
                let param_a = if instruction.mode_a == 1 {
                    opcodes[*pointer + 1]
                } else {
                    opcodes[opcodes[*pointer + 1] as usize]
                };
                let param_b = if instruction.mode_b == 1 {
                    opcodes[*pointer + 2]
                } else {
                    opcodes[opcodes[*pointer + 2] as usize]
                };

                println!("{} */+ {} -> {}", param_a, param_b, pos_c);
                match instruction.opcode {
                    1 => opcodes[pos_c] = param_a + param_b,
                    2 => opcodes[pos_c] = param_a * param_b,
                    _ => (),
                }
                println!("{}", opcodes[pos_c]);
                *pointer += 4;
            }
            3 => {
                let pos_a = opcodes[*pointer + 1] as usize;
                opcodes[pos_a] = input[input_index];
                input_index += 1;
                *pointer += 2;
            }
            4 => {
                println!(
                    "Output {}: {}",
                    opcodes[*pointer + 1],
                    opcodes[opcodes[*pointer + 1] as usize]
                );
                let output = opcodes[opcodes[*pointer + 1] as usize];
                *pointer += 2;
                return (0, output);
            }
            5 | 6 => {
                let param_a = if instruction.mode_a == 1 {
                    opcodes[*pointer + 1]
                } else {
                    opcodes[opcodes[*pointer + 1] as usize]
                };
                let param_b = if instruction.mode_b == 1 {
                    opcodes[*pointer + 2]
                } else {
                    opcodes[opcodes[*pointer + 2] as usize]
                };

                match instruction.opcode {
                    5 => {
                        if param_a != 0 {
                            *pointer = param_b as usize;
                        } else {
                            *pointer += 3;
                        }
                    }
                    6 => {
                        if param_a == 0 {
                            *pointer = param_b as usize;
                        } else {
                            *pointer += 3;
                        }
                    }
                    _ => (),
                }
            }
            7 => {
                let param_a = if instruction.mode_a == 1 {
                    opcodes[*pointer + 1]
                } else {
                    opcodes[opcodes[*pointer + 1] as usize]
                };
                let param_b = if instruction.mode_b == 1 {
                    opcodes[*pointer + 2]
                } else {
                    opcodes[opcodes[*pointer + 2] as usize]
                };
                let pos_c = opcodes[*pointer + 3] as usize;

                opcodes[pos_c] = if param_a < param_b { 1 } else { 0 };
                *pointer += 4;
            }
            8 => {
                let param_a = if instruction.mode_a == 1 {
                    opcodes[*pointer + 1]
                } else {
                    opcodes[opcodes[*pointer + 1] as usize]
                };
                let param_b = if instruction.mode_b == 1 {
                    opcodes[*pointer + 2]
                } else {
                    opcodes[opcodes[*pointer + 2] as usize]
                };
                let pos_c = opcodes[*pointer + 3] as usize;

                opcodes[pos_c] = if param_a == param_b { 1 } else { 0 };
                *pointer += 4;
            },
            99 => {
                return (99, 0);
            }
            _ => break,
        }
    }

    return (99, 0);
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
