use std::fs;
use structopt::StructOpt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

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

    for row in grid {
        for tile in row {
            print!("{} | ", tile.number);
        }
        println!("");
    }
}

fn read_and_parse(path: &str) -> Vec<Tile> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
	
	contents.split("\n\n").map(ToOwned::to_owned).map(|s| parse_tile(s)).collect()	
}

fn parse_tile(tile_strings: String) -> Tile {
	let parts: Vec<&str> = tile_strings.split("\n").collect();
	let pixels: Vec<Vec<bool>> = parts[1..].iter().map(|s| s.chars().map(|c| match c { '#' => true, _ => false, }).collect() ).collect();

	let number: u64 = parts[0].split(" ").last().unwrap().split(":").next().unwrap().parse().unwrap();
	let mut result = Tile { name: parts[0].to_string(), pixels: pixels, number: number, edges: Vec::new() };

	build_edges(&mut result);
	result
}

fn build_edges(tile: &mut Tile) {
	let mut edges = Vec::new();
	edges.push(tile.pixels[0].clone());
	edges.push(tile.pixels.iter().map(|r| r.iter().last().unwrap().clone()).collect());
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

fn flip_vertical(tile: &Tile) -> Tile {
    let mut flipped = tile.clone();
	let mut result = Vec::new();
	for line in &flipped.pixels {
		let mut reverse_line = line.clone();
		reverse_line.reverse();
		result.push(reverse_line);
	}
	flipped.pixels = result;
	build_edges(&mut flipped);

    flipped
}

fn rotate_90(tile: &Tile) -> Tile {
    print_tile(&tile);
    let mut rotated = tile.clone();
	let mut result = Vec::new();
    for _ in 0..tile.pixels.len() {
        result.push(Vec::new());
    }
    for x in (0..tile.pixels.len()).rev() {
        for y in 0..tile.pixels.len() {
            result[x].push(tile.pixels[y][x]);
        }
    }
	rotated.pixels = result;
	print_tile(&rotated);
    build_edges(&mut rotated);

    rotated
}

fn rotate_270(tile: &Tile) -> Tile {
    let mut rotated = tile.clone();
	let mut result = Vec::new();
    for _ in 0..tile.pixels.len() {
        result.push(Vec::new());
    }
    for x in 0..tile.pixels.len() {
        for y in (0..tile.pixels.len()).rev() {
            result[x].push(tile.pixels[y][x]);
        }
    }
	rotated.pixels = result;
	build_edges(&mut rotated);

    rotated
}

fn generate_variants(data: &mut Vec<Tile>) {
    let source = data.clone();

    for tile in source {
        data.push(flip_horizontal(&tile));
        data.push(flip_vertical(&tile));
        let rotated270 = rotate_270(&tile);
        data.push(flip_horizontal(&rotated270));
        data.push(flip_vertical(&rotated270));
        data.push(rotated270);
        let rotated90 = rotate_90(&tile);
        data.push(flip_horizontal(&rotated90));
        data.push(flip_vertical(&rotated90));
        data.push(rotated90);
    }
}

fn generate_edge_map(tiles: &Vec<Tile>) -> HashMap<Vec<bool>, Vec<Tile>> {
    let mut result: HashMap<Vec<bool>, Vec<Tile>> = HashMap::new();
    for tile in tiles {
        for edge in &tile.edges {
            result.entry(edge.clone()).or_insert(Vec::new()).push(tile.clone());
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
        result.entry(tile.number).or_insert(Vec::new()).push(tile.clone());
    }

    result
}

fn build_grid(number_tiles_map: &HashMap<u64, Vec<Tile>>, categorized: &HashMap<u8, Vec<u64>>, edge_map: &HashMap<Vec<bool>, Vec<Tile>>, columns: usize) -> Vec<Vec<Tile>> {
    let mut result: Vec<Vec<Tile>>;

    for corner in &categorized[&4] {
        for tile in find_corner_orientations(&number_tiles_map[&corner], &vec!(RIGHT, BOTTOM), &edge_map) {
            match build_from_corner(tile, number_tiles_map, categorized, edge_map, columns) {
                Some(result) => {
                    return result;
                },
                None => (),
            }
        }
    }

    return vec!();
}

fn build_from_corner(tile: Tile, number_tiles_map: &HashMap<u64, Vec<Tile>>, categorized: &HashMap<u8, Vec<u64>>, edge_map: &HashMap<Vec<bool>, Vec<Tile>>, columns: usize) -> Option<Vec<Vec<Tile>>> {
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
            if (y == 0 && x == columns - 1) || (y == columns - 1 && x == 0) || (y == columns - 1 && x == columns - 1) {
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
                edge_match.insert(LEFT, result[0][x-1].edges[RIGHT].clone());
            } else if x == 0 {
                // left border
                edge_match.insert(TOP, result[y-1][0].edges[BOTTOM].clone());
            } else {
                edge_match.insert(LEFT, result[y][x-1].edges[RIGHT].clone());
                edge_match.insert(TOP, result[y-1][x].edges[BOTTOM].clone());
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
                },
                None => {
                    for row in result {
                        for tile in row {
                            print!("{} | ", tile.number);
                        }
                        println!("");
                    }
                    return None;
                },
            }
        }
    }

    Some(result)
}

fn find_edge_match(tiles: &Vec<Tile>, edge_match: &HashMap<usize, Vec<bool>>) -> Option<Tile> {
    for tile in tiles {
        let mut matches = 0;
        println!("Tile: {}", tile.number);
        for (position, edge) in edge_match {
            print_edge(&edge);
            print_edge(&tile.edges[*position]);
            println!("");
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

fn find_corner_orientations(tiles: &Vec<Tile>, sides: &Vec<usize>, edge_map: &HashMap<Vec<bool>, Vec<Tile>>) -> Vec<Tile> {
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

fn print_edge(edge: &Vec<bool>) {
    for value in edge {
        print!("{}", match value { true => '#', _ => '.' });
    }
    println!("");
}

fn print_tile(tile: &Tile) {
	println!("{}", tile.name);
	for line in &tile.pixels {
		for row in &*line {
			print!("{}", match row { true => "#", false => ".", });
		}
		println!("");
	}
}