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
    bags: Vec<String>,
}

fn main() {
    let args = Cli::from_args();

    let all_bags = read_and_parse(&args.path, &"shiny gold".to_string());
    println!("finished parsing");

    for (key, value) in &all_bags {
        print!("{} => ", key);
        for bag in value {
            print!("{}, ", bag);
        }
        println!("");
    }

    let mut valid_bags: HashSet<String> = HashSet::new();
    bags_contained_count(&all_bags, &"shiny gold".to_string(), &mut valid_bags);

    valid_bags.remove(&"shiny gold".to_string());

    println!("Count: {}", &valid_bags.len());
}

fn bags_contained_count(
    bags: &HashMap<String, HashSet<String>>,
    name: &String,
    result: &mut HashSet<String>,
) {
    match bags.get(name) {
        None => {
            println!("{} is not allowed in any other bag", name);
            result.insert(name.clone());
        }
        Some(bag_bags) => {
            result.insert(name.clone());
            for bag in bag_bags {
                bags_contained_count(bags, bag, result);
            }
        }
    }
}

fn read_and_parse(path: &str, main_bag: &String) -> HashMap<String, HashSet<String>> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut bag_mapping: HashMap<String, HashSet<String>> = HashMap::new();

    for row in contents.split("\n") {
        let bag = parse_bag(&row);
        if bag.name != *main_bag {
            for name in bag.bags {
                bag_mapping
                    .entry(name)
                    .or_insert(HashSet::new())
                    .insert(bag.name.clone());
            }
        }
    }

    bag_mapping
}

fn parse_bag(data: &str) -> Bag {
    let parts: Vec<String> = data.split(" contain ").map(ToOwned::to_owned).collect();
    let bag_name = normalize_bag_name(&parts[0]);
    if parts[1] == "no other bag" {
        return Bag {
            name: bag_name.clone(),
            bags: vec![],
        };
    }

    let contains: Vec<String> = parts[1]
        .replace(".", "")
        .split(", ")
        .map(|s| normalize_bag_name(&s[2..s.len()].to_string()))
        .collect();

    println!("{} contains {}", bag_name, contains.join(","));
    Bag {
        name: bag_name.clone(),
        bags: contains,
    }
}

fn normalize_bag_name(name: &String) -> String {
    name.replace(" bags", "").replace(" bag", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let bag = parse_bag(&"muted white bags contain 4 dark orange bags, 3 bright white bags.");
        assert_eq!(bag.name, "muted white");
        assert_eq!(bag.bags[0], "dark orange");
        assert_eq!(bag.bags[1], "bright white");
    }
}
