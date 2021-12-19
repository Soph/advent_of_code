use std::cmp::Ordering;
use std::fmt;
use std::fs;
use structopt::StructOpt;

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

impl Point {
    fn rotate_x(&self, angle: f64) -> Point {
        let mut new_point = self.clone();
        new_point.y = (self.y as f64 * angle.cos() - self.z as f64 * angle.sin()).round() as i64;
        new_point.z = (self.y as f64 * angle.sin() + self.z as f64 * angle.cos()).round() as i64;
        return new_point;
    }

    fn rotate_y(&self, angle: f64) -> Point {
        let mut new_point = self.clone();
        new_point.x = (self.x as f64 * angle.cos() + self.z as f64 * angle.sin()).round() as i64;
        new_point.z = (-self.x as f64 * angle.sin() + self.z as f64 * angle.cos()).round() as i64;
        return new_point;
    }

    fn rotate_z(&self, angle: f64) -> Point {
        let mut new_point = self.clone();
        new_point.x = (self.x as f64 * angle.cos() - self.y as f64 * angle.sin()).round() as i64;
        new_point.y = (self.x as f64 * angle.sin() + self.y as f64 * angle.cos()).round() as i64;
        return new_point;
    }

    fn offset(&self, point: &Point) -> Point {
        Point {
            x: self.x - point.x,
            y: self.y - point.y,
            z: self.z - point.z,
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x)
    }
}

#[derive(Clone, Hash, Eq, Debug)]
struct Scanner {
    position: Point,
    beacons: Vec<Point>,
}

impl Scanner {
    fn rotations(&self) -> Vec<Scanner> {
        let mut scanners = vec![];

        for i in 0..=3 {
            scanners.push(Scanner {
                position: Point { x: 0, y: 0, z: 0 },
                beacons: self
                    .beacons
                    .iter()
                    .map(|b| b.rotate_x(std::f64::consts::PI / 2.0 * i as f64))
                    .collect(),
            })
        }
        for i in 0..=3 {
            for scanner in scanners.clone() {
                let scanner = Scanner {
                    position: Point { x: 0, y: 0, z: 0 },
                    beacons: scanner
                        .beacons
                        .iter()
                        .map(|b| b.rotate_y(std::f64::consts::PI / 2.0 * i as f64))
                        .collect(),
                };
                if !scanners.contains(&scanner) {
                    scanners.push(scanner);
                }
            }
            let scanner = Scanner {
                position: Point { x: 0, y: 0, z: 0 },
                beacons: self
                    .beacons
                    .iter()
                    .map(|b| b.rotate_y(std::f64::consts::PI / 2.0 * i as f64))
                    .collect(),
            };
            if !scanners.contains(&scanner) {
                scanners.push(scanner);
            }
        }

        for i in 0..=3 {
            for scanner in scanners.clone() {
                let scanner = Scanner {
                    position: Point { x: 0, y: 0, z: 0 },
                    beacons: scanner
                        .beacons
                        .iter()
                        .map(|b| b.rotate_z(std::f64::consts::PI / 2.0 * i as f64))
                        .collect(),
                };
                if !scanners.contains(&scanner) {
                    scanners.push(scanner);
                }
            }
            let scanner = Scanner {
                position: Point { x: 0, y: 0, z: 0 },
                beacons: self
                    .beacons
                    .iter()
                    .map(|b| b.rotate_z(std::f64::consts::PI / 2.0 * i as f64))
                    .collect(),
            };
            if !scanners.contains(&scanner) {
                scanners.push(scanner);
            }
        }

        return scanners;
    }

    fn normalized(&self) -> Scanner {
        let mut min_x = self.beacons[0].x;
        let mut min_y = self.beacons[0].y;
        let mut min_z = self.beacons[0].z;

        for beacon in self.beacons.iter() {
            if beacon.x < min_x {
                min_x = beacon.x;
            }
            if beacon.y < min_y {
                min_y = beacon.y;
            }
            if beacon.z < min_z {
                min_z = beacon.z;
            }
        }

        self.shift(&Point {
            x: -min_x,
            y: -min_y,
            z: -min_z,
        })
    }

    fn shift(&self, offset: &Point) -> Scanner {
        let mut new_beacons = vec![];
        for beacon in self.beacons.iter() {
            new_beacons.push(Point {
                x: beacon.x + offset.x,
                y: beacon.y + offset.y,
                z: beacon.z + offset.z,
            });
        }

        Scanner {
            position: offset.clone(),
            beacons: new_beacons,
        }
    }

    fn rotated_and_normalized(&self) -> Vec<Scanner> {
        self.rotations().iter().map(|s| s.normalized()).collect()
    }

    fn matching_beacons(&self, other: &Scanner) -> Vec<Point> {
        let mut matching: Vec<Point> = vec![];
        for beacon in self.beacons.iter() {
            if other.beacons.contains(beacon) {
                matching.push(beacon.clone())
            }
        }
        matching
    }

    fn align(&self, other: &Scanner) -> Option<Scanner> {
        for other_beacon in other.clone().beacons {
            for rotation in self.rotations() {
                for beacon in rotation.clone().beacons {
                    let offset = other_beacon.offset(&beacon);
                    let shifted = rotation.shift(&offset);
                    let matching = other.matching_beacons(&shifted);
                    if matching.len() >= 12 {
                        return Some(shifted);
                    }
                }
            }
        }
        None
    }
}

impl PartialEq for Scanner {
    fn eq(&self, other: &Self) -> bool {
        let match_count = self.matching_beacons(&other).len();
        match_count == self.beacons.len() || match_count >= 12
    }
}

fn align_all(scanners: &Vec<Scanner>) -> Vec<Scanner> {
    let mut aligned: Vec<Scanner> = vec![scanners[0].clone()];
    let mut unaligned: Vec<Scanner> = scanners.clone();
    let index = unaligned.iter().position(|x| *x == scanners[0]).unwrap();
    unaligned.remove(index);

    loop {
        for aligned_scanner in aligned.clone() {
            for scanner in unaligned.clone() {
                if let Some(matching) = scanner.align(&aligned_scanner) {
                    println!("{:?}", matching.position);
                    aligned.push(matching);
                    let index = unaligned.iter().position(|x| *x == scanner).unwrap();
                    unaligned.remove(index);
                    break;
                }
            }
        }
        println!("{}", aligned.len());
        if aligned.len() == scanners.len() {
            break;
        }
    }

    return aligned;
}

fn main() {
    let args = Cli::from_args();
    let scanners = read_and_parse(&args.path);

    let aligned = align_all(&scanners);

    let mut beacons = vec![];
    for scanner in aligned.clone() {
        beacons.append(&mut scanner.beacons.clone());
    }
    beacons.sort();
    beacons.dedup();

    println!("{}", beacons.len());

    let mut max_distance = 0;
    for scanner_a in aligned.clone() {
        for scanner_b in aligned.clone() {
            let sum = (scanner_a.position.x - scanner_b.position.x).abs()
                + (scanner_a.position.y - scanner_b.position.y).abs()
                + (scanner_a.position.z - scanner_b.position.z).abs();
            if max_distance < sum {
                max_distance = sum;
            }
        }
    }

    println!("{}", max_distance);
}

fn read_and_parse(path: &str) -> Vec<Scanner> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    contents.split("\n\n").map(|s| parse_scanner(s)).collect()
}

fn parse_scanner(lines: &str) -> Scanner {
    let mut parts = lines.split("\n");
    parts.next(); // skip first line
    let mut beacons = vec![];

    for line in parts {
        let mut coords = line.split(",");
        let x = coords.next().unwrap().parse::<i64>().unwrap();
        let y = coords.next().unwrap().parse::<i64>().unwrap();
        let z = coords.next().unwrap().parse::<i64>().unwrap();

        beacons.push(Point { x, y, z });
    }

    return Scanner {
        position: Point { x: 0, y: 0, z: 0 },
        beacons,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_rotation() {
        let scanner = Scanner {
            position: Point { x: 0, y: 0, z: 0 },
            beacons: vec![
                Point { x: -1, y: -1, z: 1 },
                Point { x: -2, y: -2, z: 2 },
                Point { x: -3, y: -3, z: 3 },
                Point { x: -2, y: -3, z: 1 },
                Point { x: 5, y: 6, z: -4 },
                Point { x: 8, y: 0, z: 7 },
            ],
        };

        let scanner2 = Scanner {
            position: Point { x: 0, y: 0, z: 0 },
            beacons: vec![
                Point { x: 1, y: 1, z: -1 },
                Point { x: 2, y: 2, z: -2 },
                Point { x: 3, y: 3, z: -3 },
                Point { x: 1, y: 3, z: -2 },
                Point { x: -4, y: -6, z: 5 },
                Point { x: 7, y: 0, z: 8 },
            ],
        };
        assert_eq!(scanner.rotations().len(), 24);
        assert_eq!(scanner.rotations().contains(&scanner2), true);
    }

    #[test]
    fn test_normalize() {
        let scanner1 = Scanner {
            position: Point { x: 0, y: 0, z: 0 },
            beacons: vec![
                Point { x: 0, y: 2, z: 0 },
                Point { x: 4, y: 1, z: 0 },
                Point { x: 3, y: 3, z: 0 },
            ],
        };
        let normalized_scanner1 = scanner1.normalized();
        let scanner2 = Scanner {
            position: Point { x: 0, y: 0, z: 0 },
            beacons: vec![
                Point { x: -1, y: -1, z: 0 },
                Point { x: -5, y: 0, z: 0 },
                Point { x: -2, y: 1, z: 0 },
            ],
        };
        let normalized_scanner2 = scanner2.normalized();
        assert_eq!(normalized_scanner1, normalized_scanner2);
    }

    #[test]
    fn test_align() {
        let scanners = read_and_parse("data_example2.txt");
        assert_eq!(scanners[1].align(&scanners[0]).is_some(), true);
        assert_eq!(
            scanners[1].align(&scanners[0]).unwrap().position,
            Point {
                x: 68,
                y: -1246,
                z: -43
            }
        );

        let aligned_1 = scanners[1].align(&scanners[0]).unwrap();
        assert_eq!(scanners[4].align(&aligned_1).is_some(), true);
        assert_eq!(
            scanners[4].align(&aligned_1).unwrap().position,
            Point {
                x: -20,
                y: -1133,
                z: 1061
            }
        );

        let aligned_4 = scanners[4].align(&aligned_1).unwrap();
        assert_eq!(scanners[2].align(&aligned_4).is_some(), true);
        assert_eq!(
            scanners[2].align(&aligned_4).unwrap().position,
            Point {
                x: 1105,
                y: -1205,
                z: 1229
            }
        );

        assert_eq!(scanners[3].align(&aligned_1).is_some(), true);
        assert_eq!(
            scanners[3].align(&aligned_1).unwrap().position,
            Point {
                x: -92,
                y: -2380,
                z: -20
            }
        );
    }
}
