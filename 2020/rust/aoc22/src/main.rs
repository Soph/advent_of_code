use std::collections::HashSet;
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
    let mut data = read_and_parse(&args.path);

    play1(&mut data.0, &mut data.1);

    println!("PART 2");
    println!("=================================");
    let mut history = vec![HashSet::new(), HashSet::new()];

    data = read_and_parse(&args.path);
    let winner = play2(&mut data.0, &mut data.1, &mut history);

    print_decks(&data.0, &data.1);

    match winner {
        1 => {
            calc_deck(&data.0);
        }
        _ => {
            calc_deck(&data.1);
        }
    }
}

fn read_and_parse(path: &str) -> (Vec<u32>, Vec<u32>) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let players: Vec<&str> = contents.split("\n\n").collect();

    (parse_deck(players[0]), parse_deck(players[1]))
}

fn parse_deck(deck: &str) -> Vec<u32> {
    deck.split("\n").collect::<Vec<&str>>()[1..]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn play1(deck1: &mut Vec<u32>, deck2: &mut Vec<u32>) {
    loop {
        print_decks(deck1, deck2);

        let left = deck1.remove(0);
        let right = deck2.remove(0);

        if left > right {
            deck1.push(left);
            deck1.push(right);
        } else {
            deck2.push(right);
            deck2.push(left);
        }

        if deck1.len() == 0 {
            println!("Player 2 won!");
            println!("Winning Deck Score: {}", calc_deck(deck2));
            break;
        }
        if deck2.len() == 0 {
            println!("Player 2 won!");
            println!("Winning Deck Score: {}", calc_deck(deck1));
            break;
        }
    }
}

fn play2(d1: &Vec<u32>, d2: &Vec<u32>, history: &mut Vec<HashSet<Vec<u32>>>) -> u8 {
    let mut deck1 = d1.clone();
    let mut deck2 = d2.clone();
    let mut winner;
    loop {
        if history[0].contains(&deck1.clone()) && history[1].contains(&deck2.clone()) {
            return 1;
        }
        history[0].insert(deck1.clone());
        history[1].insert(deck2.clone());

        print_decks(&deck1, &deck2);

        let left = deck1.remove(0);
        let right = deck2.remove(0);

        if left <= deck1.len() as u32 && right <= deck2.len() as u32 {
            println!("Recursive Game!");
            let mut recursive_history = vec![HashSet::new(), HashSet::new()];
            let mut sub_deck1 = deck1.clone()[0..left as usize].to_vec();
            let mut sub_deck2 = deck2.clone()[0..right as usize].to_vec();
            winner = play2(&mut sub_deck1, &mut sub_deck2, &mut recursive_history);
        } else {
            if left > right {
                winner = 1;
            } else {
                winner = 2;
            }
        }

        match winner {
            1 => {
                deck1.push(left);
                deck1.push(right);
            }
            _ => {
                deck2.push(right);
                deck2.push(left);
            }
        }

        if deck1.len() == 0 {
            println!("Player 2 won!");
            println!("Winning Deck Score: {}", calc_deck(&deck2));
            return 2;
        }
        if deck2.len() == 0 {
            println!("Player 2 won!");
            println!("Winning Deck Score: {}", calc_deck(&deck1));
            return 1;
        }
    }
}

fn print_decks(deck1: &Vec<u32>, deck2: &Vec<u32>) {
    print!("Deck1: ");
    for card in deck1 {
        print!("{},", card)
    }
    println!("");
    print!("Deck2: ");
    for card in deck2 {
        print!("{},", card)
    }
    println!("");
    println!("");
}

fn calc_deck(deck: &Vec<u32>) -> u32 {
    let mut result = 0;
    for i in 0..deck.len() {
        result += deck[i] * (deck.len() as u32 - i as u32);
    }

    result
}
