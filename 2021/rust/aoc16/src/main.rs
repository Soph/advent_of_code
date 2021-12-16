use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    id: u8,
    number: u16,
    sub_packets: Vec<Packet>,
}

fn main() {
    let args = Cli::from_args();
    let binary = read_and_parse(&args.path);

    println!("{}", binary);
}

fn read_and_parse(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| hex_to_bin(chunk.into_iter().collect::<String>()))
        .collect::<String>()
}

fn hex_to_bin(hex: String) -> String {
    let mut bin = String::new();
    for c in hex.chars() {
        match c {
            '0' => bin.push_str("0000"),
            '1' => bin.push_str("0001"),
            '2' => bin.push_str("0010"),
            '3' => bin.push_str("0011"),
            '4' => bin.push_str("0100"),
            '5' => bin.push_str("0101"),
            '6' => bin.push_str("0110"),
            '7' => bin.push_str("0111"),
            '8' => bin.push_str("1000"),
            '9' => bin.push_str("1001"),
            'A' => bin.push_str("1010"),
            'B' => bin.push_str("1011"),
            'C' => bin.push_str("1100"),
            'D' => bin.push_str("1101"),
            'E' => bin.push_str("1110"),
            'F' => bin.push_str("1111"),
            _ => println!("Unknown character: {}", c),
        }
    }
    bin
}

fn parse_literal(literal: String) -> Packet {
    let mut mut_literal = literal.clone();
    let version = isize::from_str_radix(&mut_literal.drain(..3).collect::<String>(), 2).unwrap();
    let id = isize::from_str_radix(&mut_literal.drain(..3).collect::<String>(), 2).unwrap();
    let mut number_string = String::new();

    loop {
        let mut chunk = mut_literal.drain(..5).collect::<String>();
        let identifier = chunk.drain(..1).collect::<String>();
        if identifier == "1" {
            number_string.push_str(&chunk.drain(..4).collect::<String>());
        } else {
            number_string.push_str(&chunk.drain(..4).collect::<String>());
            break;
        }
    }

    let number = isize::from_str_radix(&number_string, 2).unwrap();
   
    Packet {
        version: version as u8,
        id: id as u8,
        number: number as u16,
        sub_packets: vec![],
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_parsing() {
        assert_eq!(parse_literal("110100101111111000101000".to_string()), Packet { version: 6, id: 4, number: 2021, sub_packets: vec![] });
    }

}