use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd)]
struct Move {
    from: Position,
    to: Position,
    energy: u64,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd)]
struct Amphipod {
    name: char,
    position: Position,
    move_energy: u64,
}

impl Amphipod {
    fn new(name: char, position: Position) -> Amphipod {
        let energy = match name {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            _ => 1000,
        };
        Amphipod {
            name: name,
            position: position,
            move_energy: energy,
        }
    }
}


#[derive(Clone, Debug, PartialOrd)]
struct Playfield {
    grid: Vec<Vec<char>>,
    total_energy: u64,
    amphipods: Vec<Amphipod>,
}

impl Hash for Playfield {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.playfield_string().hash(state);
    }
}

impl PartialEq for Playfield {
    fn eq(&self, other: &Self) -> bool {
        self.playfield_string() == other.playfield_string()
    }
}
impl Eq for Playfield {}

impl Playfield {
    fn destination_x(name: char) -> usize {
        return match name {
            'A' => 3,
            'B' => 5,
            'C' => 7,
            'D' => 9,
            _ => panic!("Unknown amphipod {}", name),
        }
    }

    fn possible_moves(&self) -> Vec<Move> {
        let mut valid_moves = vec![];
        for amphipod in &self.amphipods {
            // println!("Checking: {}, {:?}", amphipod.name, amphipod.position);
            let position = amphipod.position.clone();
            let name = amphipod.name.clone();
            let target_x = Playfield::destination_x(name);
            // 0123456789012
            // #...........#
            // ###A#B#C#D###
            // ###A#B#C#D###            
            if position.y == 2 && self.grid[1][position.x] != '.' // someone above
                || position.y == 0 && position.x == 1 && self.grid[0][2] != '.' // someone right
                || position.y == 0 && position.x == 11 && self.grid[0][10] != '.' // someone left
                || (
                    position.y == 2 && self.grid[0][position.x+1] != '.' // someone right to exit
                    && position.y == 2 && self.grid[0][position.x-1] != '.' // someone left to exit
                 ) // can't leave room
                || position.y == 2 && position.x == target_x // we are home already, don't move
                || position.y == 1 && position.x == target_x && self.grid[2][position.x] == name // we are home already, and below is the right amphipod too
            {
                // can't move, let's STAAAYYYYYY!
                // println!("Can't move, let's STAAAYYYYYY! - {}", name);
                continue;
            }
    
            // let's see if we can move to our target room
            if self.grid[2][target_x] == '.' && self.grid[1][target_x] == '.' { // move bottom into room possible
                let target = Position::new(target_x, 2);
                let steps = self.check_path(&position, &target);
                if steps > 0 {
                    valid_moves.push(Move {
                        from: position.clone(),
                        to: target,
                        energy: steps * amphipod.move_energy,
                    });
                }      
            } else if self.grid[1][target_x] == '.' && self.grid[2][target_x] == name { // move top into room possible
                let target = Position::new(target_x, 1);
                //println!("Checking to move from {:?} to {:?}", position, target);
                let steps = self.check_path(&position, &target);
                if steps > 0 {
                    valid_moves.push(Move {
                        from: position.clone(),
                        to: target,
                        energy: steps * amphipod.move_energy,
                    });
                }  
            } else {
                for parking_position in self.free_parking_positions() {
                    let steps = self.check_path(&position, &parking_position);
                    if steps > 0 {
                        valid_moves.push(Move {
                            from: position.clone(),
                            to: parking_position.clone(),
                            energy: steps * amphipod.move_energy,
                        });
                    }
                }
            }
        }
        valid_moves
    }

    fn free_parking_positions(&self) -> Vec<Position> {
        let parking_positions = vec![
            Position::new(1, 0),
            Position::new(2, 0),
            Position::new(4, 0),
            Position::new(6, 0),
            Position::new(8, 0),
            Position::new(10, 0),
            Position::new(11, 0),
        ];
        let mut result = vec![];
        for parking_position in parking_positions {
            if self.grid[parking_position.y][parking_position.x] == '.' {
                result.push(parking_position.clone());
            }
        }
        return result;
    }

    fn check_path(&self, start: &Position, end: &Position) -> u64 {
        let mut updated_position = start.clone();
        let mut steps = 0;
        loop {
            if updated_position.x != end.x { // not in target room
                if updated_position.y == 2 {
                    if self.grid[1][updated_position.x] == '.' {
                        steps += 1;
                        updated_position.y -= 1;
                    } else {
                        return 0; // can't reach
                    }
                } else if updated_position.y == 1 {
                    if self.grid[0][updated_position.x] == '.' {
                        steps += 1;
                        updated_position.y -= 1;
                    } else {
                        return 0; // can't reach
                    }
                } else if updated_position.y == 0 {
                    if start.x > end.x { // left
                        if self.grid[0][updated_position.x-1] == '.' {
                            //println!("Move {:?}", updated_position);
                            steps += 1;
                            updated_position.x -= 1;
                            //println!("To {:?}", updated_position);
                        } else {
                            return 0; // can't reach
                        }
                    } else { // right
                        if self.grid[0][updated_position.x+1] == '.' {
                            //println!("Move {:?}", updated_position);
                            steps += 1;
                            updated_position.x += 1;
                            //println!("To {:?}", updated_position);
                        } else {
                            return 0; // can't reach
                        }
                    }
                }                 
            } else { // target x reached
                if updated_position.y == 0 {
                    if self.grid[1][updated_position.x] == '.' {
                        steps += 1;
                        updated_position.y += 1;
                    } else {
                        return 0; // can't reach
                    }
                } else if updated_position.y == 1 {
                    if self.grid[2][updated_position.x] == '.' { // can we move into the bottom of the room?
                        steps += 1;
                        updated_position.y += 1;
                    }
                }
            }
            if updated_position == end.clone() {
                return steps;
            }
        }
    }

    fn generate_new_playfields(&self) -> Vec<Playfield> {
        let mut new_playfields = vec![];
        for move_ in self.possible_moves() {
            let new_playfield = self.apply_move(move_);
            new_playfields.push(new_playfield);
        }
        return new_playfields;
    }

    fn apply_move(&self, move_: Move) -> Playfield {
        let mut new_playfield = self.clone();
        let letter = new_playfield.grid[move_.from.y][move_.from.x];
        new_playfield.grid[move_.from.y][move_.from.x] = '.';
        new_playfield.grid[move_.to.y][move_.to.x] = letter;
        new_playfield.total_energy += move_.energy;
        new_playfield.amphipods = Playfield::generate_amphipods(&new_playfield.grid);
        return new_playfield;
    }

    //   0123456789012
    // 0 #...........#
    // 1 ###A#B#C#D###
    // 2 ###A#B#C#D###    
    fn is_done(&self) -> bool {
        self.grid[2][3] == 'A' && self.grid[1][3] == 'A' 
            && self.grid[2][5] == 'B' && self.grid[1][5] == 'B'
            && self.grid[2][7] == 'C' && self.grid[1][7] == 'C'
            && self.grid[2][9] == 'D' && self.grid[1][9] == 'D'
    }

    fn playfield_string(&self) -> String{
        let mut string = "".to_owned();
        for i in 0..self.grid[0].len() {
            string.push_str(&format!("{}", i % 10));
        }
        string.push('\n');
        for row in &self.grid {
            for c in row {
                string.push_str(&format!("{}", c));
            }
            string.push('\n');
        }
        string.push('\n');

        string
    }

    fn print_playfield(&self) {
        println!("{}", self.playfield_string());
    }

    fn generate_amphipods(grid: &Vec<Vec<char>>) -> Vec<Amphipod> {
        let mut amiphods = vec![];
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                match grid[y][x] {
                    'A' | 'B' | 'C' | 'D' => amiphods.push(Amphipod::new(grid[y][x], Position::new(x, y))),
                    _ => {}
                }
            }
        }
        amiphods
    }

    fn new(grid: Vec<Vec<char>>) -> Playfield {
        let amphipods = Playfield::generate_amphipods(&grid);
        Playfield {
            grid: grid,
            amphipods: amphipods,
            total_energy: 0,
        }
    }
}

fn main() {    
    let playfield_grid: Vec<Vec<char>> = vec![
        //    0    1    2    3    4    5    6    7    8    9    0    1    2
        vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        vec!['#', '#', '#', 'B', '#', 'C', '#', 'B', '#', 'D', '#', '#', '#'],
        vec!['#', '#', '#', 'A', '#', 'D', '#', 'C', '#', 'A', '#', '#', '#'],
    ];

    let playfield = Playfield::new(playfield_grid);

    let mut playfields = vec![playfield];
    let mut min_energy = std::u64::MAX;

    let mut known: HashSet<String> = HashSet::new();
    loop {
        let mut new_playfields = vec![];
        for playfield in &playfields {
            let new_candidates = playfield.generate_new_playfields();
            let finished: Vec<Playfield> = new_candidates.iter().filter(|p| p.is_done()).map(|p| p.clone()).collect();
            if finished.len() > 0 {
                println!("Finished! {}", finished[0].total_energy);
                for finished_playfield in finished {
                    if finished_playfield.total_energy < min_energy {
                        min_energy = finished_playfield.total_energy;
                    }
                }
                continue;
            } else {
                for new_playfield in new_candidates {
                    if known.contains(&new_playfield.playfield_string()) {
                        continue;
                    }
                    known.insert(new_playfield.playfield_string());
                    new_playfields.push(new_playfield);
                }
            }
        }
        println!("min cost: {}, playfields: {}", min_energy, new_playfields.len());
        playfields = new_playfields.clone();
        if playfields.len() == 0 {
            break;
        }
    }

    println!("{}", min_energy);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_done() {
        let playfield_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#'],
        ];
        let playfield = Playfield {
            grid: playfield_grid,
            total_energy: 0,
            amphipods: vec![],
        };

        assert_eq!(playfield.is_done(), true);
    }

    #[test]
    fn test_possible_moves() {
        let playfield_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#'],
        ];
        let playfield = Playfield::new(playfield_grid);

        assert_eq!(playfield.possible_moves(), vec![]);

        let playfield_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', '.', '.', '.', '.', '.', '.', '.', 'C', '.', '.', '.', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', '.', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#'],
        ];
        let playfield = Playfield::new(playfield_grid);

        assert_eq!(playfield.possible_moves().len(), 1);
        
    }

    #[test]
    fn test_generate_new_playfields() {
        let playfield_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', '.', '.', '.', '.', '.', '.', '.', 'C', '.', '.', '.', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', '.', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#'],
        ];
        let playfield = Playfield::new(playfield_grid);
        let new_playfields = playfield.generate_new_playfields();

        assert_eq!(new_playfields.len(), 1);
        assert_eq!(new_playfields[0].is_done(), true);
        assert_eq!(new_playfields[0].total_energy, 200);

        let playfield_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', '.', '.', '.', '.', '.', 'C', '.', 'C', '.', '.', '.', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', '.', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', '.', '#', 'D', '#', '#', '#'],
        ];
        let playfield = Playfield::new(playfield_grid);
        let result_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', '.', '.', '.', '.', '.', 'C', '.', '.', '.', '.', '.', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', '.', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#'],
        ];
        let result = Playfield::new(result_grid);
        let new_playfields = playfield.generate_new_playfields();

        assert_eq!(new_playfields.len(), 2);
        assert_eq!(new_playfields.iter().filter(|p| p.is_done()).count(), 0);
        assert_eq!(new_playfields.iter().filter(|p| p.grid == result.grid).count(), 1);

        let playfield_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', '.', '.', '.', '.', '.', '.', '.', 'B', '.', 'B', '.', '#'],
            vec!['#', '#', '#', 'A', '#', 'C', '#', '.', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'C', '#', '.', '#', 'D', '#', '#', '#'],
        ];
        let playfield = Playfield::new(playfield_grid);
        let result_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', '.', '.', '.', '.', '.', '.', '.', 'B', '.', 'B', '.', '#'],
            vec!['#', '#', '#', 'A', '#', '.', '#', '.', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'C', '#', 'C', '#', 'D', '#', '#', '#'],
        ];
        let result = Playfield::new(result_grid);
        let new_playfields = playfield.generate_new_playfields();

        assert_eq!(new_playfields.len(), 6);
        assert_eq!(new_playfields.iter().filter(|p| p.is_done()).count(), 0);
        assert_eq!(new_playfields.iter().filter(|p| p.grid == result.grid).count(), 1);
        
        // #...B.......#
        // ###B#C#.#D###
        //   #A#D#C#A#
        let playfield_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', '.', '.', '.', 'B', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '#', '#', 'B', '#', 'C', '#', '.', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'D', '#', 'C', '#', 'A', '#', '#', '#'],
        ];
        let playfield = Playfield::new(playfield_grid);
        let result_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', '.', '.', '.', 'B', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '#', '#', 'B', '#', '.', '#', 'C', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'D', '#', 'C', '#', 'A', '#', '#', '#'],
        ];
        let result = Playfield::new(result_grid);
        let new_playfields = playfield.generate_new_playfields();
        
        assert_eq!(new_playfields.iter().filter(|p| p.grid == result.grid).next().unwrap().total_energy, 400);
    }
}
