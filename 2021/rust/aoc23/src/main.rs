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
        self.grid == other.grid
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
            //println!("Checking: {}, {:?}", amphipod.name, amphipod.position);
            let position = amphipod.position.clone();
            let name = amphipod.name.clone();
            let target_x = Playfield::destination_x(name);
            // 0123456789012
            // #...........#
            // ###A#B#C#D###
            // ###A#B#C#D###

            // in the corners, someone blocking
            if (position.y == 0 && position.x == 1 && self.grid[0][2] != '.') || (position.y == 0 && position.x == 11 && self.grid[0][10] != '.') {
                continue;
            }

            // we are in a room, but the left/right in 0 are blocked
            if position.y > 0 && self.grid[0][position.x+1] != '.'  && self.grid[0][position.x-1] != '.' {
                continue;
            }

            // everyone below us, belongs here.
            if position.x == target_x && self.grid[position.y..self.grid.len()].iter().map(|r| r[position.x]).filter(|c| c != &name).collect::<Vec<char>>().len() == 0 {
                continue;
            }

            // is something above us?
            if position.y > 2 && self.grid[0..position.y].iter().map(|r| r[position.x]).filter(|c| c != &'.' ).collect::<Vec<char>>().len() != 0 {
                continue;
            }

            // is room for letter free or only occupied by right letter?
            if self.grid[1..].iter().map(|r| r[target_x]).filter(|c| c != &'.' && c != &name ).collect::<Vec<char>>().len() == 0 {
                for i in (1..self.grid.len()).rev() {
                    if self.grid[i][target_x] == '.' {
                        let target = Position::new(target_x, i);
                        let steps = self.check_path(&position, &target);
                        //println!("Can move to {:?} in {} steps!", target, steps);
                        if steps > 0 {
                            valid_moves.push(Move {
                                from: position.clone(),
                                to: Position::new(target_x, i),
                                energy: amphipod.move_energy * steps,
                            });
                            break;
                        }
                    }
                }
            }
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
        //println!("Checking Path from {:?} to {:?}", start, end);
        //self.print_playfield();
        loop {
            if updated_position.x != end.x { // not in target room
                if updated_position.y > 0 {
                    //println!("Checking {:?}: {}", updated_position, self.grid[updated_position.y-1][updated_position.x]);
                    if self.grid[updated_position.y-1][updated_position.x] == '.' {
                        updated_position.y -= 1;
                        steps += 1;
                    } else {
                        return 0;
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
                if updated_position.y < self.grid.len() && self.grid[updated_position.y+1][updated_position.x] == '.' {
                    updated_position.y = updated_position.y+1;
                    steps += 1;
                } else {
                    return 0;
                }
            }
            if updated_position == end.clone() {
                //println!("Moved: {}", steps);
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
        new_playfield.grid[move_.to.y][move_.to.x] = new_playfield.grid[move_.from.y][move_.from.x];
        new_playfield.grid[move_.from.y][move_.from.x] = '.';
        new_playfield.total_energy += move_.energy;
        new_playfield.amphipods = Playfield::generate_amphipods(&new_playfield.grid);
        return new_playfield;
    }

    //   0123456789012
    // 0 #...........#
    // 1 ###A#B#C#D###
    // 2 ###A#B#C#D###    
    fn is_done(&self) -> bool {
        for y in 1..self.grid.len() {
            if !(self.grid[y][3] == 'A' && self.grid[y][5] == 'B' && self.grid[y][7] == 'C' && self.grid[y][9] == 'D') {
                return false;
            }
        }
        return true;
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
        vec!['#', '#', '#', 'D', '#', 'C', '#', 'B', '#', 'A', '#', '#', '#'],
        vec!['#', '#', '#', 'D', '#', 'B', '#', 'A', '#', 'C', '#', '#', '#'],
        vec!['#', '#', '#', 'A', '#', 'D', '#', 'C', '#', 'A', '#', '#', '#'],
    ];

    let playfield_grid: Vec<Vec<char>> = vec![
        //    0    1    2    3    4    5    6    7    8    9    0    1    2
        vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        vec!['#', '#', '#', 'D', '#', 'A', '#', 'A', '#', 'D', '#', '#', '#'],
        vec!['#', '#', '#', 'D', '#', 'C', '#', 'B', '#', 'A', '#', '#', '#'],
        vec!['#', '#', '#', 'D', '#', 'B', '#', 'A', '#', 'C', '#', '#', '#'],
        vec!['#', '#', '#', 'C', '#', 'C', '#', 'B', '#', 'B', '#', '#', '#'],
    ];

    let playfield = Playfield::new(playfield_grid);

    let mut playfields = vec![playfield];
    let mut min_energy = std::u64::MAX;

    let mut known: HashMap<Playfield, u64> = HashMap::new();
    loop {
        let mut new_playfields = vec![];
        for playfield in playfields {
            let new_candidates = playfield.generate_new_playfields();
            let finished: Vec<Playfield> = new_candidates.iter().filter(|p| p.is_done()).map(|p| p.clone()).collect();
            if finished.len() > 0 {
                println!("Finished! {}", finished[0].total_energy);
                for finished_playfield in finished {
                    if finished_playfield.total_energy < min_energy {
                        finished_playfield.print_playfield();
                        min_energy = finished_playfield.total_energy;
                    }
                }
            }
            for new_playfield in new_candidates {
                let total_energy = new_playfield.total_energy;
                if known.get(&new_playfield).is_none() {
                    known.insert(new_playfield.clone(), total_energy);
                    new_playfields.push(new_playfield);
                } else if known.get(&new_playfield).unwrap() > &total_energy {
                    known.insert(new_playfield.clone(), total_energy);
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

        let playfield_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', '.', '.', '.', '.', '.', 'C', '.', '.', '.', '.', '.', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', '.', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#'],
        ];
        let playfield = Playfield::new(playfield_grid);
        assert_eq!(playfield.is_done(), false);

        let playfield_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#'],
        ];
        let playfield = Playfield::new(playfield_grid);
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

        assert_eq!(playfield.possible_moves().len(), 7);
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
        println!("{:?}", playfield.possible_moves());
        let new_playfields = playfield.generate_new_playfields();

        assert_eq!(new_playfields.len(), 7);
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

        assert_eq!(new_playfields.len(), 7);
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
        for p in &new_playfields {
            println!("{}", p.playfield_string());
        }
        assert_eq!(new_playfields.len(), 10);
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

    #[test]
    fn test_part2() {
        let playfield_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', 'A', '.', '.', '.', '.', '.', '.', 'B', '.', 'B', 'D', '#'],
            vec!['#', '#', '#', 'B', '#', 'C', '#', '.', '#', '.', '#', '#', '#'],
            vec!['#', '#', '#', 'D', '#', 'C', '#', '.', '#', '.', '#', '#', '#'],
            vec!['#', '#', '#', 'D', '#', 'B', '#', 'A', '#', 'C', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'D', '#', 'C', '#', 'A', '#', '#', '#'],
        ];
        let playfield = Playfield::new(playfield_grid);

        assert_eq!(playfield.amphipods.len(), 16);
        let new_playfields = playfield.generate_new_playfields();

        for playfield in &new_playfields {
            playfield.print_playfield();
        }

        assert_eq!(new_playfields.len(), 15);

        let playfield_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', 'C', 'C', '.', '.', '.', '.', '.', '.', '.', 'C', 'C', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', '.', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', '.', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', '.', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'B', '#', '.', '#', 'D', '#', '#', '#'],
        ];
        let playfield = Playfield::new(playfield_grid);

        assert_eq!(playfield.amphipods.len(), 16);
        let new_playfields = playfield.generate_new_playfields();

        for playfield in &new_playfields {
            playfield.print_playfield();
        }

        assert_eq!(new_playfields.len(), 8);
        assert_eq!(new_playfields.iter().map(|p| p.total_energy).max().unwrap(), 900);
    }

    #[test]
    fn test_part_calc() {
        let playfield_grid: Vec<Vec<char>> = vec![
            //    0    1    2    3    4    5    6    7    8    9    0    1    2
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '#', '#', 'B', '#', 'C', '#', 'B', '#', 'D', '#', '#', '#'],
            vec!['#', '#', '#', 'D', '#', 'C', '#', 'B', '#', 'A', '#', '#', '#'],
            vec!['#', '#', '#', 'D', '#', 'B', '#', 'A', '#', 'C', '#', '#', '#'],
            vec!['#', '#', '#', 'A', '#', 'D', '#', 'C', '#', 'A', '#', '#', '#'],
        ];
        let mut playfield = Playfield::new(playfield_grid);
        // struct Move {
        //     from: Position,
        //     to: Position,
        //     energy: u64,
        // }
        assert_eq!(playfield.check_path(&Position { x: 9, y: 1 }, &Position { x: 11, y: 0 }), 3);
        playfield = playfield.apply_move(Move {
            from: Position { x: 9, y: 1 },
            to: Position { x: 11, y: 0 },
            energy: 3000,
        });
        assert_eq!(playfield.check_path(&Position { x: 9, y: 2 }, &Position { x: 1, y: 0 }), 10);
        playfield = playfield.apply_move(Move {
            from: Position { x: 9, y: 2 },
            to: Position { x: 1, y: 0 },
            energy: 10,
        });
        assert_eq!(playfield.check_path(&Position { x: 7, y: 1 }, &Position { x: 10, y: 0 }), 4);
        playfield = playfield.apply_move(Move {
            from: Position { x: 7, y: 1 },
            to: Position { x: 10, y: 0 },
            energy: 40,
        });
        assert_eq!(playfield.check_path(&Position { x: 7, y: 2 }, &Position { x: 8, y: 0 }), 3);
        playfield = playfield.apply_move(Move {
            from: Position { x: 7, y: 2 },
            to: Position { x: 8, y: 0 },
            energy: 30,
        });
        assert_eq!(playfield.check_path(&Position { x: 7, y: 3 }, &Position { x: 2, y: 0 }), 8);
        playfield = playfield.apply_move(Move {
            from: Position { x: 7, y: 3 },
            to: Position { x: 2, y: 0 },
            energy: 8,
        });
        assert_eq!(playfield.check_path(&Position { x: 5, y: 1 }, &Position { x: 7, y: 3 }), 6);
        playfield = playfield.apply_move(Move {
            from: Position { x: 5, y: 1 },
            to: Position { x: 7, y: 3 },
            energy: 600,
        });

        assert_eq!(playfield.total_energy, 3688);
    }
}
