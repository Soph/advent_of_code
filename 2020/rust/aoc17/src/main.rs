use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

struct PointVec {
    x: usize,
    y: usize,
    z: usize,
    w: usize,
}

fn main() {
    let args = Cli::from_args();
    let data = read_and_parse(&args.path);

    run(&data, false);
    run(&data, true);
}

fn read_and_parse(path: &String) -> Vec<Vec<bool>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n")
        .map(ToOwned::to_owned)
        .map(|s| {
            s.chars()
                .map(|s| match s {
                    '#' => true,
                    _ => false,
                })
                .collect()
        })
        .collect()
}

fn run(data: &Vec<Vec<bool>>, use_w: bool) -> u64 {
    let mut cube: Vec<Vec<Vec<Vec<bool>>>> = vec!();
    let max_edge = 6 * 5 + cube.len() as i32; // let's see if this adds up

    initialize(&mut cube, data, max_edge as usize);
    let mut count_w_start = cube.len()/2;
    let mut count_w_end = cube.len()/2 + 1;
    if use_w {
        count_w_start = 0;
        count_w_end = cube.len();
    }

    for _ in 0..6 {
        let mut updated: Vec<PointVec> = Vec::new();
        for w in count_w_start..count_w_end {
            for z in 0..cube.len() {
                for y in 0..cube.len() {
                    for x in 0..cube.len() {
                        if handle_position(&mut cube, PointVec{ x: x, y: y, z: z, w: w }) {
                            updated.push(PointVec{ x: x, y: y, z: z, w: w });
                        }
                    }
                }
            }
        }
        for update in updated {
            cube[update.w][update.z][update.y][update.x] = !cube[update.w][update.z][update.y][update.x];
        }
    }
    println!("Final: ");
    print_all(&cube, false);
    0
}

fn handle_position(cube: &mut Vec<Vec<Vec<Vec<bool>>>>, point: PointVec) -> bool {
    let current = cube[point.w][point.z][point.y][point.x];

    let mut active: u32 = 0;
    for w in -1..=1 {
        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0{
                        continue;
                    }
                    if (point.w as i32 + w) < 0 || (point.z as i32 + z) < 0 || (point.y as i32 + y) < 0 || (point.x as i32 + x) < 0 {
                        continue;
                    }
                    if (point.w as i32 + w) >= cube.len() as i32 || (point.z as i32 + z) >= cube.len() as i32 || (point.y as i32 + y) >= cube.len() as i32 || (point.x as i32 + x) >= cube.len() as i32 {
                        continue;
                    }

                    if cube[(point.w as i32 + w) as usize][(point.z as i32 + z) as usize][(point.y as i32 + y) as usize][(point.x as i32 + x) as usize] {
                        // println!("{},{},{},{} checking {},{},{},{} : {})", point.x, point.y, point.z, point.w, (point.x as i32 + x), (point.y as i32 + y), (point.z as i32 + z), (point.x as i32 + w), cube[(point.w as i32 + w) as usize][(point.z as i32 + z) as usize][(point.y as i32 + y) as usize][(point.x as i32 + x) as usize]);
                        active += 1;
                    }
                }
            }
        }
    }

    if current && (active < 2 || active > 3) {
        //println!("{},{},{},{} # -> . ({})", point.x, point.y, point.z, point.w, active);
        return true;
    }
    if !current && (active == 3) {
        //println!("{},{},{},{} . -> # ({})", point.x, point.y, point.z, point.w, active);
        return true;
    }

    false
}


fn initialize(field: &mut Vec<Vec<Vec<Vec<bool>>>>, data: &Vec<Vec<bool>>, max_edge: usize) {
    for _ in 0..max_edge {
        let mut z_field: Vec<Vec<Vec<bool>>> = Vec::new();
        for _ in 0..max_edge {
            let mut y_field: Vec<Vec<bool>> = Vec::new();
            for _ in 0..max_edge {
                let mut x_field: Vec<bool> = Vec::new();
                for _ in 0..max_edge {
                    x_field.push(false);
                }
                y_field.push(x_field);
            }
            z_field.push(y_field);
        }
        field.push(z_field);        
    }

    let center = (max_edge as i32 / 2) as usize;
    let offset = center - data.len() / 2;
    println!("");    
    for y in 0..data.len() {
        for x in 0..data.len() {
            match data[y][x] {
                true => print!("#"),
                false => print!(".")
            }
            field[center][center][y+offset][x+offset] = data[y][x];
        }
        println!("");
    }

    print_all(field, false);
}

fn print_all(field: &Vec<Vec<Vec<Vec<bool>>>>, grid: bool) {
    let mut active = 0;
    for w in 0..field.len() {
        for z in 0..field.len() {
            if grid {
                println!("z:{} w:{}", z as i32 - field.len() as i32 / 2, w as i32 - field.len() as i32 / 2);
            }
            for y in 0..field.len() {
                for x in 0..field.len() {
                    match field[w][z][y][x] {
                        true => {
                            active += 1;
                            if grid {
                                print!("#");
                            }
                        },
                        false => {
                            if grid {
                                print!(".");
                            }
                        }
                    }
                }
                if grid {
                    println!("");
                }
            }
            if grid {
                println!("");
                println!("");
            }
        }
    }
    println!("Active: {}", active);
}