use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("Part1 (example): {}", run1(4, 8));
    println!("Part1: {}", run1(9, 4));

    run2(4, 8);
    run2(9, 4);
}


fn run1(player1: u128, player2: u128) -> u128 {
    let mut score1: u128 = 0;
    let mut pos1 = player1;
    let mut score2: u128 = 0;
    let mut pos2 = player2;
    let mut dice = 0;
    let mut throws = 0;

    while score1 < 1000 && score2 < 1000 {
        for _ in 0..3 {
            throws += 1;
            dice = dice % 100 + 1;
            pos1 += dice;
        }
        pos1 = (pos1 - 1) % 10 + 1;
        score1 += pos1;
        if score1 >= 1000 {
            return score2 * throws;
        }

        for _ in 0..3 {
            throws += 1;
            dice = dice % 100 + 1;
            pos2 += dice;
        }
        pos2 = (pos2 - 1) % 10 + 1;
        score2 += pos2;
        if score2 >= 1000 {
            return score1 * throws;
        }
    }
    0
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd)]
struct Play {
    score_a: u128,
    score_b: u128,
    pos_a: u128,
    pos_b: u128,
}

impl Play {
    fn string(&self) -> String {
        format!("{}-{}-{}-{}", self.pos_a, self.pos_b, self.score_a, self.score_b)
    }
}

fn run2(start_a: u128, start_b: u128) {
    let mut plays: HashMap<Play, u128> = HashMap::new();
    let mut wins: (u128, u128) = (0,0);

    let start = Play {
        pos_a: start_a,
        pos_b: start_b,
        score_a: 0,
        score_b: 0,
    };
    plays.insert(start, 1);

    loop {
        for n in 0..=1 {
            let mut new_plays: HashMap<Play, u128> = HashMap::new();
            for (play, play_count) in plays {
                for i in 1..=3 {
                    for j in 1..=3 {
                        for k in 1..=3 {
                            let mut new_play = play.clone();
                            if n == 0 {
                                new_play.pos_a += i+j+k;
                                new_play.pos_a = (new_play.pos_a - 1) % 10 + 1;
                                new_play.score_a += new_play.pos_a;
                            } else {
                                new_play.pos_b += i+j+k;
                                new_play.pos_b = (new_play.pos_b - 1) % 10 + 1;
                                new_play.score_b += new_play.pos_b;
                            }
                            if new_play.score_a >= 21 {
                                wins.0 += play_count;
                            } else if new_play.score_b >= 21 {
                                wins.1 += play_count;
                            } else {
                                *new_plays.entry(new_play.clone()).or_insert(0) += play_count;
                            }          
                        }
                    }
                }
            }
            plays = new_plays.clone();
        }
        //println!("{:?}", plays);
        if plays.len() == 0 {
            println!("Player A: {}, Player B: {}", wins.0, wins.1);
            return;
        } else {
            println!("Player A: {}, Player B: {}, Plays: {}", wins.0, wins.1, plays.len());
        }
    }
}
