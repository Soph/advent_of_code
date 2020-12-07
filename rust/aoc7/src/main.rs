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

struct Bag {
    name: String,
    bags: Vec<(String, u32)>,
}

fn main() {
    let args = Cli::from_args();

    let all_bags = read_and_parse(&args.path);
    let mapped_bags = map_bags(&all_bags);

    // for (key, value) in &mapped_bags {
    //     print!("{} => ", key);
    //     for bag in value {
    //         print!("{}, ", bag);
    //     }
    //     println!("");
    // }

    let mut valid_bags: HashSet<String> = HashSet::new();
    bags_contained(&mapped_bags, &"shiny gold".to_string(), &mut valid_bags);

    valid_bags.remove(&"shiny gold".to_string());

    println!("Count: {}", &valid_bags.len());

    let mut bags = bag_count(&all_bags, &"shiny gold".to_string());
    bags.sort();
    bags.dedup();
    println!("Count Option 2: {}", bags.len());

    let mut weight = weights(&all_bags, &"shiny gold".to_string());
    weight -= 1; // because "shiny gold" has no own weight
    println!("Required wrapped bags: {}", weight);
}

// Parsing
fn read_and_parse(path: &str) -> Vec<Bag> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.split("\n").map(|s| parse_bag(&s)).collect()
}

fn parse_bag(data: &str) -> Bag {
    let parts: Vec<String> = data.split(" contain ").map(ToOwned::to_owned).collect();
    let bag_name = normalize_bag_name(&parts[0]);
    if parts[1] == "no other bags." {
        return Bag {
            name: bag_name.clone(),
            bags: vec![],
        };
    }

    let contains: Vec<(String, u32)> = parts[1]
        .replace(".", "")
        .split(", ")
        .map(|s| {
            (
                normalize_bag_name(&s[2..s.len()].to_string()),
                s[0..1].parse::<u32>().unwrap(),
            )
        })
        .collect();

    Bag {
        name: bag_name.clone(),
        bags: contains,
    }
}

fn normalize_bag_name(name: &String) -> String {
    name.replace(" bags", "").replace(" bag", "")
}

// Part 1
fn bags_contained(
    bags: &HashMap<String, HashSet<String>>,
    name: &String,
    result: &mut HashSet<String>,
) {
    match bags.get(name) {
        None => {
            result.insert(name.clone());
        }
        Some(bag_bags) => {
            result.insert(name.clone());
            for bag in bag_bags {
                bags_contained(bags, bag, result);
            }
        }
    }
}

fn map_bags(bags: &Vec<Bag>) -> HashMap<String, HashSet<String>> {
    let mut bag_mapping: HashMap<String, HashSet<String>> = HashMap::new();

    for bag in bags {
        for name in &bag.bags {
            bag_mapping
                .entry(name.0.clone())
                .or_insert(HashSet::new())
                .insert(bag.name.clone());
        }
    }

    bag_mapping
}

// Part 1 version 2
fn bag_count(bags: &Vec<Bag>, name: &String) -> Vec<String> {
    let mut found_bags: Vec<String> = vec![];
    for bag in bags
        .iter()
        .filter(|b| match b.bags.iter().find(|c| c.0 == *name) {
            None => false,
            _ => true,
        })
    {
        found_bags.push(bag.name.clone());
        found_bags.append(&mut bag_count(bags, &bag.name));
    }

    found_bags
}

// Part 2
fn weights(bags: &Vec<Bag>, name: &String) -> u32 {
    let mut weight: u32 = 1;
    match bags.iter().find(|b| b.name == *name) {
        None => (),
        Some(bag) => {
            for sub_bag in &bag.bags {
                let sub_weight = weights(bags, &sub_bag.0);
                weight += sub_bag.1 * sub_weight;
                // println!("{} contains {} times {} each weighting {}", bag.name, sub_bag.1, sub_bag.0, sub_weight);
            }
        }
    }
    return weight;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let bag = parse_bag(&"muted white bags contain 4 dark orange bags, 3 bright white bags.");
        assert_eq!(bag.name, "muted white");
        assert_eq!(bag.bags[0].0, "dark orange");
        assert_eq!(bag.bags[1].0, "bright white");
    }
}
