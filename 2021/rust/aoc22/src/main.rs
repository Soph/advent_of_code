use std::fmt::{Debug, Formatter};
use std::fs;
use structopt::StructOpt;
use std::str::FromStr;
use std::convert::Infallible;
use std::collections::HashSet;
use std::collections::HashMap;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    path: String,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd)]
struct Cube {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
    on: bool,
}

impl Cube {
    fn points_part1(&self) -> Vec<Point> {
        let mut points = Vec::new();
        if self.x1 >= -50 && self.x1 <= 50 {
            if self.y1 >= -50 && self.y1 <= 50 {
                if self.z1 >= -50 && self.z1 <= 50 {
                    for x in self.x1..=self.x2 {
                        for y in self.y1..=self.y2 {
                            for z in self.z1..=self.z2 {
                                if x >= -50 && x <= 50 && y >= -50 && y <= 50 && z >= -50 && z <= 50 {
                                    points.push(Point { x: x, y: y, z: z });
                                }
                            }
                        }
                    }
                }
            }
        }    
        return points;
    }

    fn intersection(&self, cube: &Cube) -> Cube {
        let x1 = self.x1.max(cube.x1);
        let x2 = self.x2.min(cube.x2);
        let y1 = self.y1.max(cube.y1);
        let y2 = self.y2.min(cube.y2);
        let z1 = self.z1.max(cube.z1);
        let z2 = self.z2.min(cube.z2);
        return Cube {
            x1: x1,
            x2: x2,
            y1: y1,
            y2: y2,
            z1: z1,
            z2: z2,
            on: self.on && cube.on,
        };
    }

    fn contains(&self, cube: &Cube) -> bool {
        self.intersection(cube) == *cube
    }

    fn is_overlap(&self, cube: &Cube) -> bool {
        if self.x1 >= cube.x1 && self.x1 <= cube.x2 {
            if self.y1 >= cube.y1 && self.y1 <= cube.y2 {
                if self.z1 >= cube.z1 && self.z1 <= cube.z2 {
                    return true;
                }
            }
        }
        if self.x2 >= cube.x1 && self.x2 <= cube.x2 {
            if self.y2 >= cube.y1 && self.y2 <= cube.y2 {
                if self.z2 >= cube.z1 && self.z2 <= cube.z2 {
                    return true;
                }
            }
        }
        return false;   
    }

    fn size(&self) -> i64 {
        return (self.x2 - self.x1) * (self.y2 - self.y1) * (self.z2 - self.z1);
    }

    fn pretty_print(&self) -> String {
        format!("{} x={}..{},y={}..{},z={}..{}", if self.on { "on" } else { "off" }, self.x1, self.x2, self.y1, self.y2, self.z1, self.z2)
    }
}

impl FromStr for Cube {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Cube, Infallible> {
        let mut s = s.split(' ');
        let on = if s.next().unwrap() == "on" { true } else { false };
        let v = s.next().unwrap().split(',');
        let mut numbers: Vec<(i64,i64)> = vec![];
        for value in v {
            let mut nums = value.split('=').last().unwrap().split("..");
            numbers.push((nums.next().unwrap().parse::<i64>().unwrap(), nums.next().unwrap().parse::<i64>().unwrap()));
        }

        Ok(Cube {
            x1: numbers[0].0,
            x2: numbers[0].1,
            y1: numbers[1].0,
            y2: numbers[1].1,
            z1: numbers[2].0,
            z2: numbers[2].1,
            on: on
        })
    }
}

impl Debug for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.pretty_print())
    }
}

fn main() {
    let args = Cli::from_args();
    let mut cubes = read_and_parse(&args.path);

    let mut grid: HashSet<Point> = HashSet::new();
    for cube in &cubes {
        for point in cube.points_part1() {
            if cube.on {
                grid.insert(point);
            } else {
                grid.remove(&point);
            }
        }
    }

    println!("Part1: {}", grid.len());

    println!("Relevant cubes count: {}", cubes.len());
    loop {
        let count = cubes.len();
        let mut reduce = false;
        let mut relevant_cubes: Vec<Cube> = vec![cubes[0].clone()];
        for i in 1..cubes.len() {
            if cubes[i].on {
                let mut overlapping: Vec<Cube> = vec![];
                for j in 0..i {
                    if cubes[j].is_overlap(&cubes[i]) {
                        overlapping.push(cubes[j].clone());
                    }
                }
                if overlapping.len() == 1 && overlapping[0].on {
                    let intersection = overlapping[0].intersection(&cubes[i]);
                    // earlier cube is completely overlapped by later cube, can be removed
                    if intersection == overlapping[0] {
                        reduce = true;
                        println!("{} can be removedd since it's contained in {}", overlapping[0].pretty_print(), cubes[i].pretty_print());
                        relevant_cubes.retain(|x| x.clone() != overlapping[0]);
                    }
                } else if overlapping.len() > 1{
                    relevant_cubes.push(cubes[i].clone());
                }
        
                relevant_cubes.push(cubes[i].clone());
                continue;
            } else {
                let mut overlapping: Vec<Cube> = vec![];
                for j in 0..i {
                    if cubes[j].is_overlap(&cubes[i]) {
                        overlapping.push(cubes[j].clone());
                    }
                }
        
                println!("{:?} has {} overlap with prior cubes", cubes[i], overlapping.len());
        
                if overlapping.len() == 1 && overlapping[0].on {
                    let intersection = overlapping[0].intersection(&cubes[i]);
                    if intersection != cubes[i] {
                        reduce = true;
                        println!("{} can be reduced to {}", cubes[i].pretty_print(), intersection.pretty_print());
                        relevant_cubes.push(intersection); 
                    }
                } else if overlapping.len() > 1{
                    relevant_cubes.push(cubes[i].clone());
                }
            }
        }
        println!("Relevant cubes count: {}", relevant_cubes.len());
    
        let mut removable_disable_cubes: Vec<Cube> = vec![];
    
        for i in 0..cubes.len() {
            if cubes[i].on {
                continue;
            }
            for j in i+1..cubes.len() {
                if cubes[j].contains(&cubes[i]) && ((cubes[i].on && cubes[j].on) || !cubes[i].on) {
                    println!("{} contains {}", cubes[j].pretty_print(), cubes[i].pretty_print());
                    removable_disable_cubes.push(cubes[i].clone());
                }
            }
        }
    
        relevant_cubes.retain(|cube| !removable_disable_cubes.iter().any(|cube2| cube2.contains(cube)));
        cubes = relevant_cubes.clone();
        println!("Relevant cubes count: {}", relevant_cubes.len());
        if !reduce && count == cubes.len() {
            break;
        }
    }

    // let mut sum = cubes[0].size();
    // let mut enabled: Vec<Cube> = vec![cubes[0].clone()];
    // for i in 1..cubes.len() {
    //     if cubes[i].on {
    //         enabled.push(cubes[i].clone());
    //     } else {
    //         let mut new_enabled: Vec<Cube> = vec![];
    //         for cube in &enabled {
    //             if !cubes[i].is_overlap(cube) {
    //                 new_enabled.push(cube.clone());
    //             }
    //         }
    //         enabled = new_enabled;
    //     }
    // }
    // let mut enabled: Vec<Cube> = Vec::new();


    // let mut intersections: HashMap<Cube, Vec<Cube>> = HashMap::new();
    // let enabled_cubes = cubes.iter().filter(|c| c.on).collect::<Vec<&Cube>>();
    // let mut disbaled_cubes = cubes.iter().filter(|c| !c.on).collect::<Vec<&Cube>>();
    // for cube in enabled_cubes {
    //     intersections.insert(cube.clone(), vec![]);
    //     for disabled in disbaled_cubes.iter() {
    //         if cube.is_overlap(disabled) {
    //             let intersection = cube.intersection(disabled);
    //             if intersection == *cube {
    //                 println!("Cube is completely removed by off cube");
    //                 intersections.remove(cube);
    //             } else {
    //                 intersections.get_mut(cube).unwrap().push(intersection);
    //             }
    //         }
    //     }
    // }

    // let mut count_on = 0;
    // for (cube, intersections) in intersections.iter() {
    //     count_on += cube.size() - intersections.iter().map(|c| c.size()).sum::<i64>();
    //     println!("{}: {}", cube.pretty_print(), intersections.len());
    // }

    // println!("Part2: {}", count_on);
}

fn read_and_parse(path: &str) -> Vec<Cube> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.split("\n").map(|s| s.parse().unwrap()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_overlap() {
        let cube1 = Cube {
            x1: 1,
            x2: 2,
            y1: 1,
            y2: 2,
            z1: 1,
            z2: 2,
            on: true,
        };
        let cube2 = Cube {
            x1: 1,
            x2: 2,
            y1: 1,
            y2: 2,
            z1: 1,
            z2: 2,
            on: true,
        };
        assert_eq!(cube1.is_overlap(&cube2), true);
        let cube1 = Cube {
            x1: 1,
            x2: 2,
            y1: 1,
            y2: 2,
            z1: 1,
            z2: 2,
            on: true,
        };
        let cube2 = Cube {
            x1: -1,
            x2: -2,
            y1: -1,
            y2: -2,
            z1: -1,
            z2: -2,
            on: true,
        };
        assert_eq!(cube1.is_overlap(&cube2), false);
        let cube1 = Cube {
            x1: 1,
            x2: 10,
            y1: 1,
            y2: 10,
            z1: 1,
            z2: 10,
            on: true,
        };
        let cube2 = Cube {
            x1: -5,
            x2: 15,
            y1: 5,
            y2: 15,
            z1: 5,
            z2: 15,
            on: true,
        };
        assert_eq!(cube1.is_overlap(&cube2), true);        
    }

    #[test]
    fn test_intersection() {
        let cube1 = Cube {
            x1: 0,
            x2: 10,
            y1: 0,
            y2: 10,
            z1: 0,
            z2: 10,
            on: true,
        };
        let cube2 = Cube {
            x1: 5,
            x2: 15,
            y1: 5,
            y2: 15,
            z1: 5,
            z2: 15,
            on: true,
        };
        let intersection = Cube {
            x1: 5,
            x2: 10,
            y1: 5,
            y2: 10,
            z1: 5,
            z2: 10,
            on: true,
        };        
        assert_eq!(cube1.intersection(&cube2), intersection);

        let cube1 = Cube {
            x1: -10,
            x2: 10,
            y1: -10,
            y2: 10,
            z1: -10,
            z2: 10,
            on: true,
        };
        let cube2 = Cube {
            x1: -10,
            x2: 10,
            y1: -10,
            y2: 10,
            z1: 0,
            z2: 20,
            on: true,
        };
        let intersection = Cube {
            x1: -10,
            x2: 10,
            y1: -10,
            y2: 10,
            z1: 0,
            z2: 10,
            on: true,
        };        
        assert_eq!(cube1.intersection(&cube2), intersection);
    }

    #[test]
    fn test_size() {
        let cube1 = Cube {
            x1: -1,
            x2: 1,
            y1: -1,
            y2: 1,
            z1: -1,
            z2: 1,
            on: true,
        };        
        assert_eq!(cube1.size(), 8);
        let cube2 = Cube {
            x1: 0,
            x2: 1,
            y1: 0,
            y2: 1,
            z1: 0,
            z2: 1,
            on: true,
        };        
        assert_eq!(cube2.size(), 1);        
    }

    #[test]
    fn test_contains() {
        let cube1 = Cube {
            x1: -1,
            x2: 1,
            y1: -1,
            y2: 1,
            z1: -1,
            z2: 1,
            on: true,
        };        
        let cube2 = Cube {
            x1: 0,
            x2: 1,
            y1: 0,
            y2: 1,
            z1: 0,
            z2: 1,
            on: true,
        };        
        assert_eq!(cube1.contains(&cube2), true);
        assert_eq!(cube2.contains(&cube1), false);
    }
}