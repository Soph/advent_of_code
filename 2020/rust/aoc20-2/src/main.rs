use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Tile {
    name: String,
    number: u64,
    edges: Vec<Vec<bool>>,
    pixels: Vec<Vec<bool>>,
}

const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

fn main() {
    let args = Cli::from_args();
    let mut data = read_and_parse(&args.path);
    let columns = (data.len() as f64).sqrt() as usize;

    generate_variants(&mut data);
    let number_tiles_map = generate_number_tiles_map(&data);
    let edges = generate_edge_map(&data);

    let categorized = categorize(&edges);
    let grid = build_grid(&number_tiles_map, &categorized, &edges, columns);

    for row in &grid {
        for tile in row {
            print!("{} | ", tile.number);
        }
        println!("");
    }

    let full = assemble(&grid);

    let mut variants: Vec<Vec<Vec<bool>>> = Vec::new();
    let rotated90 = rotate_pixels_90(&full);
    let rotated180 = rotate_pixels_90(&rotated90);
    let rotated270 = rotate_pixels_90(&rotated180);
    let flipped = flip_pixels_horizontal(&full);
    let flippedrotated90 = rotate_pixels_90(&flipped);
    let flippedrotated180 = rotate_pixels_90(&flippedrotated90);
    let flippedrotated270 = rotate_pixels_90(&flippedrotated180);

    variants.push(full.clone());
    variants.push(rotated90);
    variants.push(rotated180);
    variants.push(rotated270);
    variants.push(flippedrotated90);
    variants.push(flippedrotated180);
    variants.push(flippedrotated270);

    for variant in variants {
        let found = search(&variant);
        if found > 0 {
            for row in &variant {
                for pixel in row {
                    match pixel {
                        true => print!("#"),
                        false => print!("."),
                    }
                }
                println!("");
            }
            println!("Found: {}", found);
            let mut count_pixels = 0;
            for row in variant {
                for pixel in row {
                    if pixel {
                        count_pixels += 1;
                    }
                }
            }
            println!("Count Pixel: {}", count_pixels);
            println!("Result: {}", count_pixels - found * 15);
            break;
        }
    }
}

fn read_and_parse(path: &str) -> Vec<Tile> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents
        .split("\n\n")
        .map(ToOwned::to_owned)
        .map(|s| parse_tile(s))
        .collect()
}

fn parse_tile(tile_strings: String) -> Tile {
    let parts: Vec<&str> = tile_strings.split("\n").collect();
    let pixels: Vec<Vec<bool>> = parts[1..]
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '#' => true,
                    _ => false,
                })
                .collect()
        })
        .collect();

    let number: u64 = parts[0]
        .split(" ")
        .last()
        .unwrap()
        .split(":")
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let mut result = Tile {
        name: parts[0].to_string(),
        pixels: pixels,
        number: number,
        edges: Vec::new(),
    };

    build_edges(&mut result);
    result
}

fn build_edges(tile: &mut Tile) {
    let mut edges = Vec::new();
    edges.push(tile.pixels[0].clone());
    edges.push(
        tile.pixels
            .iter()
            .map(|r| r.iter().last().unwrap().clone())
            .collect(),
    );
    edges.push(tile.pixels.iter().last().unwrap().clone());
    edges.push(tile.pixels.iter().map(|r| r[0]).collect());

    tile.edges = edges;
}

fn flip_horizontal(tile: &Tile) -> Tile {
    let mut flipped = tile.clone();
    flipped.pixels.reverse();

    build_edges(&mut flipped);

    flipped
}

fn flip_pixels_horizontal(pixels: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut result = pixels.clone();
    result.reverse();
    result
}

fn rotate_90(tile: &Tile) -> Tile {
    let mut rotated = tile.clone();
    rotated.pixels = rotate_pixels_90(&tile.pixels);
    build_edges(&mut rotated);

    rotated
}

fn rotate_pixels_90(pixels: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    for _ in 0..pixels.len() {
        result.push(Vec::new());
    }
    for x in 0..pixels.len() {
        for y in (0..pixels.len()).rev() {
            result[x].push(pixels[y][x]);
        }
    }

    result
}

fn generate_variants(data: &mut Vec<Tile>) {
    let source = data.clone();

    for tile in source {
        let rotated90 = rotate_90(&tile);
        let rotated180 = rotate_90(&rotated90);
        let rotated270 = rotate_90(&rotated180);
        let flipped = flip_horizontal(&tile);
        let flippedrotated90 = rotate_90(&flipped);
        let flippedrotated180 = rotate_90(&flippedrotated90);
        let flippedrotated270 = rotate_90(&flippedrotated180);

        data.push(tile);
        data.push(rotated90);
        data.push(rotated180);
        data.push(rotated270);
        data.push(flipped);
        data.push(flippedrotated90);
        data.push(flippedrotated180);
        data.push(flippedrotated270);
    }
}

fn generate_edge_map(tiles: &Vec<Tile>) -> HashMap<Vec<bool>, Vec<Tile>> {
    let mut result: HashMap<Vec<bool>, Vec<Tile>> = HashMap::new();
    for tile in tiles {
        for edge in &tile.edges {
            result
                .entry(edge.clone())
                .or_insert(Vec::new())
                .push(tile.clone());
        }
    }

    result
}

fn categorize(edge_map: &HashMap<Vec<bool>, Vec<Tile>>) -> HashMap<u8, Vec<u64>> {
    let mut uniq: HashMap<u64, u8> = HashMap::new();
    for (_, tiles) in edge_map {
        let mut unique_numbers: HashSet<u64> = HashSet::new();
        for tile in tiles {
            unique_numbers.insert(tile.number);
        }
        if unique_numbers.len() > 1 {
            // print_edge(&edge);
            for number in unique_numbers {
                // print!("{},", number);
                *uniq.entry(number).or_insert(0) += 1;
            }
            // println!("");
        }
    }

    let mut result: HashMap<u8, Vec<u64>> = HashMap::new();

    for (key, value) in &uniq {
        result.entry(*value).or_insert(Vec::new()).push(*key);
    }

    result
}

fn generate_number_tiles_map(tiles: &Vec<Tile>) -> HashMap<u64, Vec<Tile>> {
    let mut result: HashMap<u64, Vec<Tile>> = HashMap::new();

    for tile in tiles {
        result
            .entry(tile.number)
            .or_insert(Vec::new())
            .push(tile.clone());
    }

    result
}

fn build_grid(
    number_tiles_map: &HashMap<u64, Vec<Tile>>,
    categorized: &HashMap<u8, Vec<u64>>,
    edge_map: &HashMap<Vec<bool>, Vec<Tile>>,
    columns: usize,
) -> Vec<Vec<Tile>> {
    for corner in &categorized[&4] {
        for tile in
            find_corner_orientations(&number_tiles_map[&corner], &vec![RIGHT, BOTTOM], &edge_map)
        {
            match build_from_corner(tile, number_tiles_map, categorized, columns) {
                Some(result) => {
                    return result;
                }
                None => (),
            }
        }
    }

    return vec![];
}

fn search(grid: &Vec<Vec<bool>>) -> u32 {
    let pattern: Vec<Vec<bool>> = vec![
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, true, false,
        ],
        vec![
            true, false, false, false, false, true, true, false, false, false, false, true, true,
            false, false, false, false, true, true, true,
        ],
        vec![
            false, true, false, false, true, false, false, true, false, false, true, false, false,
            true, false, false, true, false, false, false,
        ],
    ];

    let mut found: u32 = 0;

    for y in 1..grid.len() - 1 {
        // checkin middle pattern first, so top and bottom row can't match
        let mut offset = 0;
        for _ in 0..grid.len() / pattern.len() {
            match match_pattern(&grid[y], &pattern[1], offset) {
                Some(a) => {
                    if match_pattern_position(&grid[y + 1], &pattern[2], a)
                        && match_pattern_position(&grid[y - 1], &pattern[0], a)
                    {
                        found += 1;
                        offset = a + pattern.len(); // there might be more
                        println!("Found, but maybe more after {}", offset);
                    }
                }
                None => {
                    break;
                }
            }
        }
    }

    found
}

fn match_pattern_position(line: &Vec<bool>, pattern: &Vec<bool>, offset: usize) -> bool {
    if offset + pattern.len() >= line.len() {
        return false;
    }
    for s in 0..pattern.len() {
        match pattern[s] {
            true => {
                if line[offset + s] {
                    if s == pattern.len() - 1 {
                        return true;
                    }
                } else {
                    break;
                }
            }
            false => {
                if s == pattern.len() - 1 {
                    return true;
                }
            }
        }
    }
    false
}

fn match_pattern(line: &Vec<bool>, pattern: &Vec<bool>, offset: usize) -> Option<usize> {
    for x in offset..line.len() {
        if match_pattern_position(line, pattern, x) {
            return Some(x);
        }
    }

    None
}

fn assemble(grid: &Vec<Vec<Tile>>) -> Vec<Vec<bool>> {
    let mut result: Vec<Vec<bool>> = Vec::new();
    let mut y = 0;
    for row in grid {
        for j in 1..row[0].pixels.len() - 1 {
            result.push(Vec::new());
            for i in 0..row.len() {
                for x in 1..row[i].pixels.len() - 1 {
                    result[y].push(row[i].pixels[j][x]);
                }
            }
            y += 1;
        }
    }

    result
}

fn build_from_corner(
    tile: Tile,
    number_tiles_map: &HashMap<u64, Vec<Tile>>,
    categorized: &HashMap<u8, Vec<u64>>,
    columns: usize,
) -> Option<Vec<Vec<Tile>>> {
    let mut result: Vec<Vec<Tile>> = Vec::new();
    let mut used: HashSet<u64> = HashSet::new();
    for _ in 0..columns {
        result.push(Vec::new());
    }
    used.insert(tile.number.clone());
    result[0].push(tile);
    for y in 0..columns {
        for x in 0..columns {
            if y == 0 && x == 0 {
                continue;
            }
            let next_tile_numbers: Vec<u64>;
            let mut edge_match: HashMap<usize, Vec<bool>> = HashMap::new();
            if (y == 0 && x == columns - 1)
                || (y == columns - 1 && x == 0)
                || (y == columns - 1 && x == columns - 1)
            {
                // corner
                next_tile_numbers = categorized[&4].clone();
            } else if y == 0 || y == columns - 1 || x == 0 || x == columns - 1 {
                // edge
                next_tile_numbers = categorized[&6].clone();
            } else {
                // in the middle
                next_tile_numbers = categorized[&8].clone();
            }

            if y == 0 {
                // top row
                edge_match.insert(LEFT, result[0][x - 1].edges[RIGHT].clone());
            } else if x == 0 {
                // left border
                edge_match.insert(TOP, result[y - 1][0].edges[BOTTOM].clone());
            } else {
                edge_match.insert(LEFT, result[y][x - 1].edges[RIGHT].clone());
                edge_match.insert(TOP, result[y - 1][x].edges[BOTTOM].clone());
            }

            let mut next_tiles: Vec<Tile> = Vec::new();
            for tile_number in next_tile_numbers {
                if used.contains(&tile_number) {
                    continue;
                }
                for tile in &number_tiles_map[&tile_number] {
                    next_tiles.push(tile.clone());
                }
            }
            match find_edge_match(&next_tiles, &edge_match) {
                Some(tile) => {
                    result[y].push(tile.clone());
                    used.insert(tile.number);
                }
                None => {
                    /* for row in result {
                        for tile in row {
                            print!("{} | ", tile.number);
                        }
                        println!("");
                    } */
                    return None;
                }
            }
        }
    }

    Some(result)
}

fn find_edge_match(tiles: &Vec<Tile>, edge_match: &HashMap<usize, Vec<bool>>) -> Option<Tile> {
    for tile in tiles {
        let mut matches = 0;
        //println!("Tile: {}", tile.number);
        for (position, edge) in edge_match {
            /*             print_edge(&edge);
                       print_edge(&tile.edges[*position]);
                       println!("");
            */
            if tile.edges[*position] == *edge {
                matches += 1;
            }
        }
        if matches == edge_match.len() {
            return Some(tile.clone());
        }
    }

    None
}

fn find_corner_orientations(
    tiles: &Vec<Tile>,
    sides: &Vec<usize>,
    edge_map: &HashMap<Vec<bool>, Vec<Tile>>,
) -> Vec<Tile> {
    let mut result: Vec<Tile> = Vec::new();

    for tile in tiles {
        let mut matches = 0;
        for side in sides {
            if edge_map.contains_key(&tile.edges[*side]) {
                matches += 1;
            }
        }
        if matches == sides.len() {
            result.push(tile.clone());
        }
    }

    result
}