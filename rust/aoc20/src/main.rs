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

fn main() {
    let args = Cli::from_args();
    let data = read_and_parse(&args.path);

    let mut test = data[0].clone();
	print_tile(&test);
	flip_horizontal(&mut test);
	print_tile(&test);
	test = data[0].clone();
	flip_vertical(&mut test);
	print_tile(&test);


	let mut corners: Vec<Tile> = Vec::new();
	let mut borders: Vec<Tile> = Vec::new();
	let mut valid_edges: HashMap<Vec<bool>, Vec<Tile>> = HashMap::new();
	for tile in &data {
    	let mut matching_edges = 0;
    	for edge in &tile.edges {
    		for tile_to_match in &data {
    			if tile_to_match.name == tile.name {
    				continue;
    			}
    			if match_edge(edge.clone(), tile_to_match.clone()) > 0 {
    				matching_edges += 1;
    				valid_edges.entry(edge.clone()).or_insert(Vec::new()).push(tile_to_match.clone());
    			}
    		}
    	}
    	if matching_edges == 2 {
    		corners.push(tile.clone());
    	}
   		println!("{} {}", tile.name, matching_edges);
    }

    let mut result = 1;
    for corner in &corners {
    	result *= corner.number;
    }
    println!("{}", result);

	let result = build_tile_set(&corners, &valid_edges, &data);
	for row in result {
		for tile in row {
			println!("{}", tile.name);
		}
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

fn flip_horizontal(tile: &mut Tile) {
	tile.pixels.reverse();

	build_edges(tile);
}

fn flip_vertical(tile: &mut Tile) {
	let mut result = Vec::new();

	for line in &tile.pixels {
		let mut reverse_line = line.clone();
		reverse_line.reverse();
		result.push(reverse_line);
	}
	tile.pixels = result;
	build_edges(tile);
}

fn match_edge(edge: Vec<bool>, tile: Tile) -> u8 {
	let mut result: u8 = 0;
	for tile_edge in &tile.edges {
		if edge == *tile_edge {
			result += 1;
		}
	}

	let mut rotated_tile = tile.clone();
	flip_vertical(&mut rotated_tile);
	for tile_edge in &rotated_tile.edges {
		if edge == *tile_edge {
			result += 1;
		}
	}

	rotated_tile = tile.clone();
	flip_horizontal(&mut rotated_tile);
	for tile_edge in &rotated_tile.edges {
		if edge == *tile_edge {
			result += 1;
		}
	}

	result
}

fn build_tile_set(corner: &Tile, valid_edges: &HashMap<Vec<bool>, corners: &Vec<Tile>, Vec<Tile>>, tiles: &Vec<Tile>) -> Vec<Vec<Tile>> {
	let columns = (tiles.len() as f64).sqrt() as usize;
	

}

fn find_next(current: Vec<Vec<Tile>>, next_position: (usize, usize), tiles: &Vec<Tile>) -> Option<Vec<Vec<Tile>>> {
	for tile in tiles {
		if next_position.0 == 0 {
			// first item in row

		} else if next_position.0 == columns - 1 {
			// last item in row
		} else {

		}
	}
}


fn matching_tiles(edges: Vec<Vec<bool>>, positions: Vec<u8>, tiles: &Vec<Tile>) -> Vec<Tile> {
	let mut result: Vec<Tile> = Vec::new();
	for tile in tiles {
		let mut flip_horizontal_tile = tile.clone();
		flip_horizontal(&mut flip_horizontal_tile);
		let mut flip_vertical_tile = tile.clone();
		flip_vertical(&mut flip_vertical_tile);
		let rotated_tiles = vec!(tile.clone(), flip_vertical_tile, flip_horizontal_tile);
		let mut found = false;
		for rotated_tile in rotated_tiles {
			let mut valid = false;
			for i in 0..edges.len() {
				if tile.edges[positions[i] as usize] == edges[i] {
					valid = true;
				} else {
					valid = false;
				}
			}
			if valid {
				result.push(tile.clone());
				found = true;
			}
		}
		if found {
			continue;
		}
	}
	result
}

fn build_tile_set(corners: &Vec<Tile>, valid_edges: &HashMap<Vec<bool>, Vec<Tile>>, tiles: &Vec<Tile>) -> Vec<Vec<Tile>> {
	let mut result: Vec<Vec<Tile>> = Vec::new();
	let columns = (tiles.len() as f64).sqrt() as usize;
	let mut used: HashSet<Tile> = HashSet::new();
	for corner in corners {
		result = Vec::new();
		for _ in 0..columns {
			result.push(Vec::new());
		}
		let mut y = 0;
		match find_corner_orientation(&valid_edges, &corner, (1,2)) {
			Some(found_corner) => {
				result[y].push(found_corner.clone());
				used.insert(found_corner.clone());
				for tile in tiles {	
					match valid_edges.get(&corner.edges[1]) {
						Some(edge_tiles) => {
							if edge_tiles.contains(tile) {
								match find_orientation(&corner.edges[1], &tile, 3) {
									Some(valid_tile) => {
										result[y].push(valid_tile.clone());
										used.insert(valid_tile.clone());
									}
									None => (),
								}
							}
						},
						None => (),
					}
				}
			},
			None => {
				continue;
			}
		}
	}
	result
}

fn find_corner_orientation(valid_edges: &HashMap<Vec<bool>, Vec<Tile>>, tile: &Tile, valid: (usize, usize)) -> Option<Tile> {
	let mut flip_horizontal_tile = tile.clone();
	flip_horizontal(&mut flip_horizontal_tile);
	let mut flip_vertical_tile = tile.clone();
	flip_vertical(&mut flip_vertical_tile);
	let tiles = vec!(tile.clone(), flip_vertical_tile, flip_horizontal_tile);
	for check_tile in tiles {
		if valid_edges.contains_key(&check_tile.edges[valid.0]) && valid_edges.contains_key(&check_tile.edges[valid.1]) {
			return Some(check_tile.clone());
		}
	}
	return None;
}

fn find_orientation(valid_edge: &Vec<bool>, tile: &Tile, position: usize) -> Option<Tile> {
	let mut flip_horizontal_tile = tile.clone();
	flip_horizontal(&mut flip_horizontal_tile);
	let mut flip_vertical_tile = tile.clone();
	flip_vertical(&mut flip_vertical_tile);
	let tiles = vec!(tile.clone(), flip_vertical_tile, flip_horizontal_tile);
	for check_tile in tiles {
		if check_tile.edges[position] == *valid_edge {
			return Some(check_tile.clone());
		}
	}
	return None;
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