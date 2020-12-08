use std::collections::HashSet;
use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

struct Instruction {
    command: String,
    argument: i16,
}

fn main() {
    let args = Cli::from_args();
    let mut instructions = read_and_parse(&args.path);

    run(&instructions);
    trial(&mut instructions);
}

// Parsing
fn read_and_parse(path: &str) -> Vec<Instruction> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(|s| parse_instruction(&s))
        .collect()
}

fn parse_instruction(line: &str) -> Instruction {
    let parts: Vec<String> = line.split(" ").map(ToOwned::to_owned).collect();

    Instruction {
        command: parts[0].clone(),
        argument: parts[1].parse().unwrap(),
    }
}

fn run(instructions: &Vec<Instruction>) -> u16 {
    let mut executed: HashSet<i16> = HashSet::new();
    let mut pointer: i16 = 0;
    let mut acc = 0;

    loop {
        let instruction = &instructions[pointer as usize];
        match instruction.command.as_str() {
            "nop" => pointer += 1,
            "acc" => {
                acc += instruction.argument;
                pointer += 1;
            }
            "jmp" => {
                pointer += instruction.argument;
            }
            _ => (),
        }
        if executed.contains(&pointer) {
            println!("Infinite Loop at {} acc: {}", pointer, acc);
            return pointer as u16;
        } else {
            executed.insert(pointer);
        }
        if pointer >= instructions.len() as i16 {
            println!("Terminated with acc: {}", acc);
            return 0;
        }
    }
}

fn trial(instructions: &mut Vec<Instruction>) {
    let mut changed: u16 = 0;

    loop {
        let last = run(instructions);
        if last == 0 {
            println!(
                "Changed command at {} to {}",
                changed, instructions[changed as usize].command
            );
            return;
        } else {
            if changed > 0 {
                // we flipped once, so flip back to old code
                instructions[changed as usize] = flip(&instructions[changed as usize]);
            }
            changed = mutate_instructions(instructions, changed + 1 as u16);
        }
    }
}

fn mutate_instructions(instructions: &mut Vec<Instruction>, offset: u16) -> u16 {
    for i in offset as usize..instructions.len() - 1 {
        if &instructions[i].command == "acc" {
            continue;
        }
        instructions[i] = flip(&instructions[i]);
        return i as u16;
    }

    0
}

fn flip(instruction: &Instruction) -> Instruction {
    if instruction.command == "jmp" {
        return Instruction {
            command: "nop".to_string(),
            argument: instruction.argument,
        };
    } else {
        return Instruction {
            command: "jmp".to_string(),
            argument: instruction.argument,
        };
    }
}
