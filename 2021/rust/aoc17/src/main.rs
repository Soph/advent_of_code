#[derive(Clone, Hash, Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    //target area: x=124..174, y=-123..-86
    // target area: x=20..30, y=-10..-5
    find(Rectangle {
        top_left: Point { x: 20, y: -5 },
        bottom_right: Point { x: 30, y: -10 },
    });
    find(Rectangle {
        top_left: Point { x: 124, y: -86 },
        bottom_right: Point { x: 174, y: -123 },
    });
}

fn find(target_area: Rectangle) {
    let mut found: Vec<i64> = vec![];
    for x in 0..=target_area.bottom_right.x {
        for y in -4000..4000 {
            match fire((x, y), target_area.clone()) {
                Some(height) => found.push(height),
                None => (),
            }
        }
    }
    found.sort();
    println!("{}", found[found.len() - 1]);
    println!("{}", found.len());
}

fn fire(velocity: (i64, i64), target_area: Rectangle) -> Option<i64> {
    let mut current_x: i64 = 0;
    let mut current_y: i64 = 0;
    let mut velocity = velocity;
    let mut max_y = target_area.bottom_right.y;
    loop {
        current_x += velocity.0;
        current_y += velocity.1;
        if velocity.0 > 0 {
            velocity.0 -= 1;
        }
        if current_y > max_y {
            max_y = current_y;
        }
        velocity.1 -= 1;
        match is_hit(
            Point {
                x: current_x,
                y: current_y,
            },
            &target_area,
        ) {
            Some(true) => return Some(max_y),
            Some(false) => return None,
            _ => continue,
        }
    }
}

fn is_hit(point: Point, target_area: &Rectangle) -> Option<bool> {
    if point.x >= target_area.top_left.x
        && point.x <= target_area.bottom_right.x
        && point.y <= target_area.top_left.y
        && point.y >= target_area.bottom_right.y
    {
        return Some(true);
    } else if point.y >= target_area.bottom_right.y && point.x <= target_area.bottom_right.x {
        return None;
    } else {
        return Some(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hit() {
        assert_eq!(
            is_hit(
                Point { x: 0, y: 0 },
                &Rectangle {
                    top_left: Point { x: 0, y: 0 },
                    bottom_right: Point { x: 1, y: -1 }
                }
            )
            .unwrap(),
            true
        );
        assert_eq!(
            is_hit(
                Point { x: 0, y: 0 },
                &Rectangle {
                    top_left: Point { x: 10, y: -10 },
                    bottom_right: Point { x: 10, y: -12 }
                }
            ),
            None
        );
        assert_eq!(
            is_hit(
                Point { x: 0, y: -20 },
                &Rectangle {
                    top_left: Point { x: 10, y: -10 },
                    bottom_right: Point { x: 10, y: -12 }
                }
            )
            .unwrap(),
            false
        );
        assert_eq!(
            is_hit(
                Point { x: 28, y: -7 },
                &Rectangle {
                    top_left: Point { x: 20, y: -5 },
                    bottom_right: Point { x: 30, y: -10 }
                }
            )
            .unwrap(),
            true
        );
    }

    #[test]
    fn test_fire() {
        assert_eq!(
            fire(
                (7, 2),
                Rectangle {
                    top_left: Point { x: 20, y: -5 },
                    bottom_right: Point { x: 30, y: -10 }
                }
            )
            .unwrap(),
            3
        );
        assert_eq!(
            fire(
                (6, 9),
                Rectangle {
                    top_left: Point { x: 20, y: -5 },
                    bottom_right: Point { x: 30, y: -10 }
                }
            )
            .unwrap(),
            45
        );
    }
}
