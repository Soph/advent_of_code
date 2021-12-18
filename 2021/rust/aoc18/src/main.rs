use std::fs;
use std::process;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

fn main() {
    let args = Cli::from_args();
    let lines = read_and_parse(&args.path);
    let mut result = lines[0].clone();

    for i in 1..lines.len() {
        result = reduce_line(add_lines(result, lines[i].clone()));
        println!("Addition Result: {}", result);
    }

    println!("{}", result);
    println!("{}", calc_magnitude(result));
}

fn read_and_parse(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.split("\n").map(|s| s.to_owned()).collect()
}

fn add_lines(left: String, right: String) -> String {
    println!("Addition: [{},{}]", left, right);
    format!("[{},{}]", left, right)
}

fn explode_line(line: &String, once: bool) -> String {
    let mut result = line.clone();
    loop {
        let chars = result.chars().collect::<Vec<char>>();
        result = "".to_owned();
        let mut exploded = false;
        let mut depth = 0;
        for mut i in 0..chars.len() {
            match chars[i] {
                '[' => {
                    depth += 1;
                    result.push(chars[i]);
                },
                ']' => {
                    depth -= 1;
                    result.push(chars[i]);
                },
                ',' => {
                    if chars[i-1].to_digit(10).is_some() && chars[i+1].to_digit(10).is_some() && depth >= 5 {
                        // number pair!
                        exploded = true;
                        // explode!
                        let mut start_pos = i-1;
                        while chars[start_pos].to_digit(10).is_some() {
                            start_pos -= 1;
                        }
                        let mut end_pos = i+1;
                        while chars[end_pos].to_digit(10).is_some() {
                            end_pos += 1;
                        }
                        //println!("Pair: {}", chars[start_pos+1..end_pos].iter().collect::<String>());
                        // left number
                        let mut number_string = format!("{}", chars[start_pos+1]);
                        for j in start_pos+2..i {
                            number_string.push(chars[j]);
                        }
                        //println!("Number String: {}", number_string);
                        let left_number = number_string.parse::<u64>().unwrap();
                        // right number
                        number_string = format!("{}", chars[i+1]);
                        for j in i+2..end_pos {
                            number_string.push(chars[j]);
                        }
                        //println!("Number String: {}", number_string);
                        let right_number = number_string.parse::<u64>().unwrap();

                        // remove old numbers
                        result = chars[0..start_pos].iter().collect::<String>();

                        let left_chars = result.chars().collect::<Vec<char>>();
                        // search left
                        let mut right_string = "".to_owned();
                        for mut j in 1..left_chars.len() {
                            let char_pos = left_chars.len() - j;
                            //println!("check: {}: {}", char_pos, left_chars[char_pos]);
                            if left_chars[char_pos].to_digit(10).is_some() {
                                let mut start_pos = left_chars.len() - j;
                                while chars[start_pos].to_digit(10).is_some() {
                                    j += 1;
                                    start_pos -= 1;
                                }
                                let number_string = format!("{}", chars[start_pos+1..=char_pos].iter().collect::<String>());

                                result = format!(
                                    "{}{}{}",
                                    &result[0..=start_pos],
                                    number_string.parse::<u64>().unwrap() as u64 + left_number,
                                    right_string
                                );
                                break;
                            } else {
                                right_string = format!("{}{}", left_chars[char_pos], &right_string);
                            }
                        }
                        result.push('0');
                        //println!("result after left search: {}", result);
                        // search right
                        i = end_pos+1; // digit + ]
                        // println!(
                        //     "rest before right: {}",
                        //     chars[i..].iter().collect::<String>().as_str()
                        // );
                        for j in i..chars.len() {
                            i += 1;                            
                            if chars[j].to_digit(10).is_some() {
                                let mut end_pos = j+1;
                                while chars[end_pos].to_digit(10).is_some() {
                                    end_pos += 1;
                                    i += 1;
                                }
                                let number_string = format!("{}", chars[j..end_pos].iter().collect::<String>());
                                // println!("Number String: {}", number_string);

                                result.push_str(&format!(
                                    "{}",
                                    number_string.parse::<u64>().unwrap() as u64 + right_number
                                ));

                                break;
                            } else {
                                result.push(chars[j]);
                            }
                        }
                        // println!("result after right search: {}", result);
                        // println!(
                        //     "left after right search: {}",
                        //     chars[i..].iter().collect::<String>().as_str()
                        // );                                
                    } else {
                        result.push(chars[i]);
                    }
                },
                _ => result.push(chars[i]),
            }
            if exploded {
                if i < chars.len() {
                    result.push_str(chars[i..].iter().collect::<String>().as_str());
                }
                if exploded {
                    println!("Exploded: {}", result);
                }
                break;
            }
        }
        if !exploded || once {
            return result;
        }
    }
}

fn split_line(line: &String, once: bool) -> String {
    let mut last_result = line.clone();
    //loop {
        let mut result = "".to_owned();
        let chars = last_result.chars().collect::<Vec<char>>();
        let mut split = false;
        for mut i in 0..chars.len() {
            match chars[i] {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let mut number_string = format!("{}", chars[i]);
                    if chars[i + 1].to_digit(10).is_some() {
                        i += 1;
                        number_string.push(chars[i]);
                    }
                    let number = number_string.parse::<u64>().unwrap();
                    if number >= 10 {
                        let left = number / 2;
                        let right = number / 2 + (number % (left * 2));
                        result.push_str(&format!("[{},{}]", left, right));
                        split = true;
                    } else {
                        result.push(chars[i]);
                    }
                },
                _ => result.push(chars[i]),
            }
            if split {
                if i < chars.len() {
                    result.push_str(chars[i + 1..].iter().collect::<String>().as_str());
                }
                if split {
                    println!("Split:    {}", result);
                }
                break;
            }
        }
        if !split || once {
            return result;
        }
        return result.clone();
    //}
}

fn reduce_line(line: String) -> String {
    let mut last_result = line.clone();
    println!("Start:    {}", last_result);
    let mut count = 0;
    loop {
        let explode = explode_line(&last_result, false);
        let split = split_line(&explode, false);
        println!{"result:   {}", split};
        if split == last_result {
            return last_result;
        }
        last_result = split.clone();
    }
}

fn calc_magnitude(line: String) -> u64 {
    let mut result = line.clone();
    
    loop {
        let mut found = false;
        let chars = result.chars().collect::<Vec<char>>();
        result = "".to_owned();
        let mut i = 0;
        while i < chars.len() {
            match chars[i] {
                ',' => {
                    if chars[i-1].to_digit(10).is_some() && chars[i+1].to_digit(10).is_some() {
                        let mut start_pos = i-1;
                        while chars[start_pos].to_digit(10).is_some() {
                            start_pos -= 1;
                        }
                        let mut end_pos = i+1;
                        while chars[end_pos].to_digit(10).is_some() {
                            end_pos += 1;
                        }
                        let mut number_string = format!("{}", chars[start_pos+1]);
                        for j in start_pos+2..i {
                            number_string.push(chars[j]);
                        }
                        //println!("Number String: {}", number_string);
                        let left_number = number_string.parse::<u64>().unwrap();
                        // right number
                        number_string = format!("{}", chars[i+1]);
                        for j in i+2..end_pos {
                            number_string.push(chars[j]);
                        }
                        //println!("Number String: {}", number_string);
                        let right_number = number_string.parse::<u64>().unwrap();
                        result = chars[0..start_pos].iter().collect::<String>();
                        result.push_str(format!("{}", left_number * 3 + right_number * 2).as_str());
                        i = end_pos+1;
                        found = true;
                    } else {
                        result.push(chars[i]);
                        i += 1;
                    }
                },
                _ => {
                    result.push(chars[i]);
                    i += 1;
                }
            }
        }
        if !found {
            return result.parse::<u64>().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_single() {
        assert_eq!(
            explode_line(&"[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_owned(), true),
            "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]".to_owned()
        );
        assert_eq!(
            explode_line(&"[[[[0,7],4],[7,[[8,4],9]]],[1,1]]".to_owned(), true),
            "[[[[0,7],4],[15,[0,13]]],[1,1]]".to_owned()
        );
        assert_eq!(
            split_line(&"[[[[0,7],4],[15,[0,13]]],[1,1]]".to_owned(), true),
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".to_owned()
        );
        assert_eq!(
            split_line(&"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".to_owned(), true),
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".to_owned()
        );
        assert_eq!(
            explode_line(&"[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".to_owned(), true),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_owned()
        );
    }

    #[test]
    fn test_reduce_all() {
        // assert_eq!(
        //     reduce_line("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_owned()),
        //     "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_owned()
        // );
        assert_eq!(
            reduce_line("[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]".to_owned()),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]".to_owned()
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            reduce_line(add_lines(
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".to_owned(),
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".to_owned()
            )),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]".to_owned()
        );
    }

    #[test]
    fn test_calc_magnitude() {
        assert_eq!(
            calc_magnitude("[[1,2],[[3,4],5]]".to_owned()),
            10
        );
    }
}
