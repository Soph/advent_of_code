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

    // for i in 1..lines.len() {
    //     result = reduce_line(add_lines(result, lines[i].clone()));
    //     println!("Addition Result: {}", result);
    // }

    result = reduce_line(add_lines(result, lines[1].clone()));
    println!("Addition Result: {}", result);

    println!("{}", result);
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
                    if depth < 5 {
                        result.push(chars[i]);
                    } else {
                        let mut numbers = false;
                        println!("Check Number? {}", result);
                        for j in i..chars.len() {
                            println!("{}", chars[j]);
                            if chars[j] == ',' && chars[j+1].to_digit(10).is_some() {
                                numbers = true;
                                break;
                            }
                            if chars[j] == '[' {
                                break;
                            }
                        }
                        if chars[i-1] != ',' && numbers {
                            while chars[i+1] == '[' {
                                result.push('[');
                                depth += 1;
                                i += 1;
                            }
                            let mut pos = i;                        
                            println!("Char: {}", chars[i..i+4].iter().collect::<String>());
                            exploded = true;
    
                            pos += 1;
                            // first number
                            let mut number_string = format!("{}", chars[pos]);
                            pos += 1;
                            if chars[pos].to_digit(10).is_some() {
                                number_string.push(chars[pos]);
                                pos += 1;
                            }
                            if chars[pos].to_digit(10).is_some() {
                                // should not happen
                                println!("depth: {}", depth);
                                println!("{}", result);
                                println!("{}", line);
                                println!("{}", number_string);
                                process::exit(1);
                            }
                            if number_string.parse::<u64>().is_err() {
                                println!("depth: {}", depth);
                                println!("{}", result);
                                println!("{}", line);
                                println!("{}", number_string);
                            }
                            let left_number = number_string.parse::<u64>().unwrap();
                            pos += 1; // skip ","

                            number_string = format!("{}", chars[pos]);
                            pos += 1;
                            if chars[pos].to_digit(10).is_some() {
                                number_string.push(chars[pos]);
                                pos += 1;
                            }
                            pos += 1; // skip "]"
                            if number_string.parse::<u64>().is_err() {
                                println!("depth: {}", depth);
                                println!("{}", result);
                                println!("{}", line);
                                println!("{}", number_string);
                            }
        
                            let right_number = number_string.parse::<u64>().unwrap();
                            // search left
                            println!(
                                "left before left: {}",
                                chars[0..i].iter().collect::<String>().as_str()
                            );
                            let mut right_string = "".to_owned();
                            for j in 1..=i {
                                let char_pos = i - j;
                                println!("check: {}: {}", char_pos, chars[char_pos]);
                                if chars[char_pos].to_digit(10).is_some() {
                                    println!("{}", right_string);
                                    result = format!(
                                        "{}{}{}",
                                        &result[0..char_pos],
                                        chars[char_pos].to_digit(10).unwrap() as u64 + left_number,
                                        right_string
                                    );
                                    break;
                                } else {
                                    right_string = format!("{}{}", chars[char_pos], &right_string);
                                }
                            }
                            result.push('0');
                            println!("result after left search: {}", result);
                            // search right
                            i = pos;
                            println!(
                                "rest before right: {}",
                                chars[pos..].iter().collect::<String>().as_str()
                            );
                            for j in i..chars.len() {
                                if chars[j].to_digit(10).is_some() {
                                    result.push_str(&format!(
                                        "{}",
                                        chars[j].to_digit(10).unwrap() as u64 + right_number
                                    ));
                                    break;
                                } else {
                                    pos += 1;
                                    result.push(chars[j]);
                                }
                            }
                            i = pos;
                            println!("result after right search: {}", result);
                            println!(
                                "left after right search: {}",
                                chars[pos..].iter().collect::<String>().as_str()
                            );    
                        } else {
                            result.push(chars[i]);
                            println!("not a non digit only number skipping, {}", result);
                            i += 1;
                        }
                    }
                },
                ',' => {
                    
                },
                ']' => {
                    depth -= 1;
                    result.push(chars[i]);
                },
                _ => result.push(chars[i]),
            }
            if exploded {
                if i < chars.len() {
                    result.push_str(chars[i + 1..].iter().collect::<String>().as_str());
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
    let mut result = line.clone();
    loop {
        let chars = result.chars().collect::<Vec<char>>();
        result = "".to_owned();
        let mut split = false;
        for mut i in 0..chars.len() {
            match chars[i] {
                '0'..='9' => {
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
                }
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
    }
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
        count += 1;
        if count > 10 {
            return last_result;
        }
        last_result = split.clone();
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
}
