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

struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn main() {
    let args = Cli::from_args();
    let data = read_and_parse(&args.path);

    process(&data);
}

fn read_and_parse(path: &str) -> Vec<Food> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.split("\n").map(|s| parse_food(s)).collect()
}

fn parse_food(line: &str) -> Food {
    // mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    let parts: Vec<&str> = line.split(" (contains ").collect();
    let mut ingredients: HashSet<String> = HashSet::new();
    for ingredient in parts[0].split(" ").map(ToOwned::to_owned) {
        ingredients.insert(ingredient);
    }
    let mut allergens: HashSet<String> = HashSet::new();
    for allergen in parts[1].replace(")", "").split(", ").map(ToOwned::to_owned) {
        allergens.insert(allergen);
    }
    Food {
        ingredients: ingredients,
        allergens: allergens,
    }
}

fn reduce_allergens(
    allergens: &HashMap<String, HashSet<String>>,
) -> HashMap<String, HashSet<String>> {
    let mut result: HashMap<String, HashSet<String>> = HashMap::new();
    let mut identified: HashSet<String> = HashSet::new();

    loop {
        for (allergen, ingredients) in allergens {
            let mut left_ingredients: HashSet<String> = HashSet::new();
            for ingredient in ingredients {
                if !identified.contains(ingredient) {
                    left_ingredients.insert(ingredient.clone());
                }
            }
            if left_ingredients.len() == 1 {
                identified.insert(left_ingredients.iter().next().unwrap().to_string());
                result.insert(allergen.clone(), left_ingredients.clone());
            }
        }

        if result.len() == allergens.len() {
            return result;
        }
    }
}

fn process(foods: &Vec<Food>) {
    let mut ingredients: HashMap<String, u32> = HashMap::new();
    let mut allergens: HashMap<String, HashSet<String>> = HashMap::new();

    for food in foods {
        for allergen in &food.allergens {
            let ingredients = allergens.entry(allergen.clone()).or_insert(HashSet::new());
            if ingredients.len() == 0 {
                for ingredient in &food.ingredients {
                    ingredients.insert(ingredient.clone());
                }
            } else {
                for ingredient in ingredients.clone().iter() {
                    if !food.ingredients.contains(ingredient) {
                        ingredients.remove(ingredient);
                    }
                }
            }
        }
        for ingredient in &food.ingredients {
            *ingredients.entry(ingredient.clone()).or_insert(0) += 1;
        }
    }

    let mut ingredients_containing_allergens: HashSet<String> = HashSet::new();
    for (_, value) in &allergens {
        for ingredient in value.into_iter() {
            ingredients_containing_allergens.insert(ingredient.clone());
        }
    }

    let mut non_allergen_ingredients = ingredients.clone();
    for ingredient in ingredients_containing_allergens {
        non_allergen_ingredients.remove(&ingredient);
    }

    let mut sum: u32 = 0;
    for (_, count) in non_allergen_ingredients {
        sum += count;
    }

    println!("Result1: {}", sum);

    let reduced_allergens = reduce_allergens(&allergens);

    let mut allergen_keys: Vec<String> = reduced_allergens.keys().cloned().collect();
    allergen_keys.sort();
    for allergen in &allergen_keys {
        print!("{} => ", allergen);
        for food in reduced_allergens[&allergen.clone()].clone() {
            print!("{},", food);
        }
        println!("");
    }

    print!("Result2: ");
    for allergen in &allergen_keys {
        for ingredient in reduced_allergens[allergen].clone() {
            print!("{},", ingredient);
        }
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_food() {
        let result = parse_food("mxmxvkd kfcds sqjhc nhms (contains dairy, dairy)");
        assert_eq!(result.ingredients.contains("mxmxvkd"), true);
        assert_eq!(result.allergens.contains("dairy"), true);
    }
}
