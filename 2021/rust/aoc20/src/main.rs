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
    let (pixel_data, image_data) = read_and_parse(&args.path);

    let processed = process2(&image_data, &pixel_data, 2);
    print(&processed);

    println!("Part1: {}", count(&processed));

    let processed2 = process2(&image_data, &pixel_data, 50);
    print(&processed2);

    println!("Part2: {}", count(&processed2));
}

fn read_and_parse(path: &str) -> (Vec<char>, Vec<Vec<char>>) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut parts = contents.split("\n\n");
    let pixel_data = parts.next().unwrap().chars().collect();
    let image_data = parts
        .next()
        .unwrap()
        .split("\n")
        .map(|s| s.chars().collect())
        .collect();

    (pixel_data, image_data)
}

fn process2(image_data: &Vec<Vec<char>>, pixel_data: &Vec<char>, times: usize) -> Vec<Vec<char>> {
    let mut defaults = vec!['0'; times + 1];
    if pixel_data[0] == '#' {
        if pixel_data[511] == '#' {
            for i in 1..=times {
                defaults[i] = '1';
            }
        } else {
            for i in 0..=times {
                if i % 2 == 0 {
                    defaults[i] = '0';
                } else {
                    defaults[i] = '1';
                }
            }
        }
    }

    println!("{:?}", defaults);

    let mut current_image_data = image_data.clone();
    for n in 0..times {
        let mut result = extend(&current_image_data);
        println!("after extend");
        print(&result);
        for y in 0..result.len() {
            for x in 0..result[0].len() {
                let mut number_data: String = "".to_owned();
                let min_y: isize = y as isize - 2;
                let min_x: isize = x as isize - 2;

                for i in min_y..=min_y + 2 {
                    for j in min_x..=min_x + 2 {
                        if i < 0
                            || i >= current_image_data.len() as isize
                            || j < 0
                            || j >= current_image_data[0].len() as isize
                        {
                            number_data.push(defaults[n]);
                        } else if current_image_data[i as usize][j as usize] == '#' {
                            number_data.push('1');
                        } else {
                            number_data.push('0');
                        }
                    }
                }
                let index = isize::from_str_radix(&*number_data, 2).unwrap();
                result[y][x] = pixel_data[index as usize];
            }
        }
        current_image_data = result.clone();
    }
    current_image_data
}

fn extend(image_data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut extended_image_data: Vec<Vec<char>> = vec![];
    let extend_by = 2;

    for _ in 0..extend_by {
        extended_image_data.push(vec!['.'; image_data[0].len() + 2 * extend_by]);
    }
    for i in 0..image_data.len() {
        let mut line = vec!['.'; image_data[0].len() + 2 * extend_by];
        for j in 0..image_data[0].len() {
            line[j + extend_by] = image_data[i][j];
        }
        extended_image_data.push(line);
    }
    for _ in 0..extend_by {
        extended_image_data.push(vec!['.'; image_data[0].len() + 2 * extend_by]);
    }

    return extended_image_data;
}

fn print(image_data: &Vec<Vec<char>>) {
    for y in 0..image_data.len() {
        for x in 0..image_data[y].len() {
            print!("{}", image_data[y][x]);
        }
        print!("\n");
    }
    print!("\n");
}

fn count(image_data: &Vec<Vec<char>>) -> u64 {
    let mut count = 0;

    for y in 0..image_data.len() {
        for x in 0..image_data[y].len() {
            if image_data[y][x] == '#' {
                count += 1;
            }
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_extend() {
        let original_image_data = vec![
            vec!['.', '#', '.'],
            vec!['#', '.', '#'],
            vec!['.', '#', '.'],
        ];
        let extended_image_data = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.'],
            vec!['.', '#', '.', '#', '.'],
            vec!['.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];

        assert_eq!(extend(&original_image_data), extended_image_data);
    }
}
