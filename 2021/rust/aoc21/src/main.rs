fn main() {
    println!("Part1 (example): {}", run1(4, 8));
    println!("Part1: {}", run1(9, 4));
}


fn run1(player1: u64, player2: u64) -> u64 {
    let mut score1: u64 = 0;
    let mut pos1 = player1;
    let mut score2: u64 = 0;
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

        println!("{}: {}, {}: {}", pos1, score1, pos2, score2);
    }
    0
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
struct Play {
    player1: u64,
    player2: u64,
    score1: u64,
    score2: u64,
    pos1: u64,
    pos2: u64,
    dice: u64,
    throws: u64,
    win: u8,
}

fn play2(play: &Play, cache: ) -> u64 {
    let mut local_play = play.clone();
    while local_play.score1 < 21 && local_play.score2 < 21 {
        for _ in 0..3 {
            local_play.throws += 1;
            local_play.dice = local_play.dice % 100 + 1;
            local_play.pos1 += local_play.dice;
        }
        local_play.pos1 = (local_play.pos1 - 1) % 10 + 1;
        local_play.score1 += local_play.pos1;
        if local_play.score1 >= 1000 {
            return local_play.score2 * local_play.throws;
        }

        for _ in 0..3 {
            local_play.throws += 1;
            local_play.dice = local_play.dice % 100 + 1;
            local_play.pos2 += local_play.dice;
        }
        local_play.pos2 = (local_play.pos2 - 1) % 10 + 1;
        local_play.score2 += local_play.pos2;
        if local_play.score2 >= 1000 {
            return local_play.score1 * local_play.throws;
        }
    }
    0
}
