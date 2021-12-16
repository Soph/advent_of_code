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
    version: u64,
    id: u64,
    number: u64,
    sub_packets: Vec<Packet>,
}

fn main() {
    let args = Cli::from_args();
    let binary = read_and_parse(&args.path);

    let parsed_packet = parse_packet(binary);
    println!("Sum of Versions: {}", sum_versions(&parsed_packet.0));
    println!("Operation result: {}", calc_packets(&parsed_packet.0));
}

fn read_and_parse(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    parse_hex_string(contents)
}

fn parse_hex_string(hex: String) -> String {
    hex.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| hex_to_bin(chunk.into_iter().collect::<String>()))
        .collect::<String>()
}

fn sum_versions(packet: &Packet) -> u64 {
    let mut sum = 0;

    sum += packet.version as u64;
    for packet in packet.sub_packets.clone() {
        sum += sum_versions(&packet.clone());
    }

    sum
}

fn calc_packets(packet: &Packet) -> u64 {
    let mut values = packet.sub_packets.iter().map(|p| calc_packets(p));

    match packet.id {
        0 => {
            return values.sum();
        }
        1 => {
            return values.product();
        }
        2 => {
            return values.min().unwrap();
        }
        3 => {
            return values.max().unwrap();
        }
        5 => {
            if values.next() > values.next() {
                return 1;
            } else {
                return 0;
            }
        }
        6 => {
            if values.next() < values.next() {
                return 1;
            } else {
                return 0;
            }
        }
        7 => {
            if values.next() == values.next() {
                return 1;
            } else {
                return 0;
            }
        }
        _ => {
            return packet.number as u64;
        }
    }
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

fn parse_number(number: String) -> (u64, String) {
    let mut mut_number = number.clone();
    let mut number_string = String::new();
    loop {
        let mut chunk = mut_number.drain(..5).collect::<String>();
        let identifier = chunk.drain(..1).collect::<String>();
        if identifier == "1" {
            number_string.push_str(&chunk.drain(..4).collect::<String>());
        } else {
            number_string.push_str(&chunk.drain(..4).collect::<String>());
            break;
        }
    }

    (
        isize::from_str_radix(&number_string, 2).unwrap() as u64,
        mut_number,
    )
}

fn parse_packet(literal: String) -> (Packet, String) {
    let mut mut_literal = literal.clone();
    let version = isize::from_str_radix(&mut_literal.drain(..3).collect::<String>(), 2).unwrap();
    let id = isize::from_str_radix(&mut_literal.drain(..3).collect::<String>(), 2).unwrap();

    match id {
        4 => {
            let parsed = parse_number(mut_literal.clone());
            return (
                Packet {
                    version: version as u64,
                    id: id as u64,
                    number: parsed.0,
                    sub_packets: vec![],
                },
                parsed.1,
            );
        }
        _ => {
            let length_type =
                isize::from_str_radix(&mut_literal.drain(..1).collect::<String>(), 2).unwrap();
            let length_bits = if length_type == 0 { 15 } else { 11 };
            let length =
                isize::from_str_radix(&mut_literal.drain(..length_bits).collect::<String>(), 2)
                    .unwrap();
            let mut sub_packets = vec![];

            match length_type {
                0 => {
                    println!("parsing packets with length: {}", length);
                    // bit length
                    let mut used_length = 0;
                    while used_length < length {
                        let parsed = parse_packet(mut_literal.clone());
                        used_length += mut_literal.len() as isize - parsed.1.len() as isize;
                        sub_packets.push(parsed.0);
                        mut_literal = parsed.1.clone();
                    }
                }
                _ => {
                    println!("parsing {} packets", length);
                    for _ in 0..length {
                        let parsed = parse_packet(mut_literal.clone());
                        sub_packets.push(parsed.0);
                        mut_literal = parsed.1.clone();
                    }
                }
            }
            return (
                Packet {
                    version: version as u64,
                    id: id as u64,
                    number: 0,
                    sub_packets: sub_packets,
                },
                mut_literal.clone(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_parsing() {
        assert_eq!(
            parse_packet("110100101111111000101000".to_string()),
            (
                Packet {
                    version: 6,
                    id: 4,
                    number: 2021,
                    sub_packets: vec![]
                },
                "000".to_string()
            )
        );
    }

    #[test]
    fn test_operational_parsing() {
        assert_eq!(
            parse_packet("00111000000000000110111101000101001010010001001000000000".to_string()),
            (
                Packet {
                    version: 1,
                    id: 6,
                    number: 0,
                    sub_packets: vec![
                        Packet {
                            version: 6,
                            id: 4,
                            number: 10,
                            sub_packets: vec![]
                        },
                        Packet {
                            version: 2,
                            id: 4,
                            number: 20,
                            sub_packets: vec![]
                        }
                    ]
                },
                "0000000".to_string()
            )
        );
        assert_eq!(
            parse_packet("11101110000000001101010000001100100000100011000001100000".to_string()),
            (
                Packet {
                    version: 7,
                    id: 3,
                    number: 0,
                    sub_packets: vec![
                        Packet {
                            version: 2,
                            id: 4,
                            number: 1,
                            sub_packets: vec![]
                        },
                        Packet {
                            version: 4,
                            id: 4,
                            number: 2,
                            sub_packets: vec![]
                        },
                        Packet {
                            version: 1,
                            id: 4,
                            number: 3,
                            sub_packets: vec![]
                        }
                    ]
                },
                "00000".to_string()
            )
        );
    }

    #[test]
    fn test_calculation() {
        assert_eq!(
            calc_packets(&parse_packet(parse_hex_string("C200B40A82".to_string())).0),
            3
        );
        assert_eq!(
            calc_packets(&parse_packet(parse_hex_string("04005AC33890".to_string())).0),
            54
        );
        assert_eq!(
            calc_packets(&parse_packet(parse_hex_string("880086C3E88112".to_string())).0),
            7
        );
        assert_eq!(
            calc_packets(&parse_packet(parse_hex_string("CE00C43D881120".to_string())).0),
            9
        );
        assert_eq!(
            calc_packets(&parse_packet(parse_hex_string("D8005AC2A8F0".to_string())).0),
            1
        );
        assert_eq!(
            calc_packets(&parse_packet(parse_hex_string("F600BC2D8F".to_string())).0),
            0
        );
        assert_eq!(
            calc_packets(&parse_packet(parse_hex_string("9C005AC2F8F0".to_string())).0),
            0
        );
        assert_eq!(
            calc_packets(
                &parse_packet(parse_hex_string("9C0141080250320F1802104A08".to_string())).0
            ),
            1
        );
    }
}
