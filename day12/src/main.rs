mod input;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Forward,
    North,
    East,
    South,
    West,
    Left,
    Right,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    amount: i64,
}

fn parse(input: &'static str) -> Vec<Movement> {
    input
        .lines()
        .map(|l| Movement {
            direction: match &l[..1] {
                "F" => Direction::Forward,
                "N" => Direction::North,
                "E" => Direction::East,
                "S" => Direction::South,
                "W" => Direction::West,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => Direction::Unknown,
            },
            amount: l[1..].parse().expect("must be a u64"),
        })
        .collect()
}

fn get_new_direction(current: &Direction, movement: Movement) -> Direction {
    let directions = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    let rotations = movement.amount / 90;

    let current_index = directions
        .iter()
        .position(|d| d == current)
        .expect("it should be one of these");

    let mut new_index = match movement.direction {
        Direction::Right => current_index as i64 + rotations,
        Direction::Left => current_index as i64 - rotations,
        _ => 0,
    };

    if new_index.is_negative() {
        new_index = 4 + new_index;
    }

    directions[new_index.abs() as usize % 4]
}

fn rotate_waypoint(waypoint: Position, movement: Movement) -> Position {
    let mut new_waypoint = waypoint.clone();

    for _ in 0..movement.amount / 90 {
        new_waypoint = match movement.direction {
            Direction::Right => Position {
                x: new_waypoint.y,
                y: new_waypoint.x * -1,
            },
            Direction::Left => Position {
                x: new_waypoint.y * -1,
                y: new_waypoint.x,
            },
            _ => new_waypoint,
        };
    }

    new_waypoint
}

fn follow_navigation(input: &'static str) -> i64 {
    let data = parse(input);
    let mut position = Position { x: 0, y: 0 };
    let mut facing = Direction::East;

    for datum in data {
        position = match datum.direction {
            Direction::Forward => match facing {
                Direction::North => Position {
                    x: position.x,
                    y: position.y + datum.amount,
                },
                Direction::East => Position {
                    x: position.x + datum.amount,
                    y: position.y,
                },
                Direction::South => Position {
                    x: position.x,
                    y: position.y - datum.amount,
                },
                Direction::West => Position {
                    x: position.x - datum.amount,
                    y: position.y,
                },
                _ => position,
            },
            Direction::North => Position {
                x: position.x,
                y: position.y + datum.amount,
            },
            Direction::East => Position {
                x: position.x + datum.amount,
                y: position.y,
            },
            Direction::South => Position {
                x: position.x,
                y: position.y - datum.amount,
            },
            Direction::West => Position {
                x: position.x - datum.amount,
                y: position.y,
            },
            _ => position,
        };

        facing = match datum.direction {
            Direction::Right => get_new_direction(&facing, datum),
            Direction::Left => get_new_direction(&facing, datum),
            _ => facing,
        };
    }

    position.x.abs() + position.y.abs()
}

fn follow_navigation_part2(input: &'static str) -> i64 {
    let data = parse(input);
    let mut position = Position { x: 0, y: 0 };
    let mut waypoint = Position { x: 10, y: 1 };

    for datum in data {
        position = match datum.direction {
            Direction::Forward => Position {
                x: (datum.amount * waypoint.x) + position.x,
                y: (datum.amount * waypoint.y) + position.y,
            },
            _ => position,
        };

        waypoint = match datum.direction {
            Direction::North => Position {
                x: waypoint.x,
                y: waypoint.y + datum.amount,
            },
            Direction::East => Position {
                x: waypoint.x + datum.amount,
                y: waypoint.y,
            },
            Direction::South => Position {
                x: waypoint.x,
                y: waypoint.y - datum.amount,
            },
            Direction::West => Position {
                x: waypoint.x - datum.amount,
                y: waypoint.y,
            },
            Direction::Right => rotate_waypoint(waypoint, datum),
            Direction::Left => rotate_waypoint(waypoint, datum),
            _ => waypoint,
        };
    }

    position.x.abs() + position.y.abs()
}

fn main() {
    let input_data = input::get_input();

    println!("Part1: {:?}", follow_navigation(input_data));
    println!("Part2: {:?}", follow_navigation_part2(input_data));
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test() {
        assert_eq!(follow_navigation(TEST_INPUT), 25)
    }

    #[test]
    fn test_part2() {
        assert_eq!(follow_navigation_part2(TEST_INPUT), 286)
    }

    #[test]
    fn test_direction() {
        assert_eq!(
            get_new_direction(
                &Direction::East,
                Movement {
                    direction: Direction::Right,
                    amount: 90
                }
            ),
            Direction::South
        )
    }

    #[test]
    fn test_direction_2() {
        assert_eq!(
            get_new_direction(
                &Direction::East,
                Movement {
                    direction: Direction::Right,
                    amount: 180
                }
            ),
            Direction::West
        )
    }

    #[test]
    fn test_direction_3() {
        assert_eq!(
            get_new_direction(
                &Direction::East,
                Movement {
                    direction: Direction::Right,
                    amount: 270
                }
            ),
            Direction::North
        )
    }

    #[test]
    fn test_direction_4() {
        assert_eq!(
            get_new_direction(
                &Direction::West,
                Movement {
                    direction: Direction::Right,
                    amount: 270
                }
            ),
            Direction::South
        )
    }

    #[test]
    fn test_direction_l() {
        assert_eq!(
            get_new_direction(
                &Direction::East,
                Movement {
                    direction: Direction::Left,
                    amount: 90
                }
            ),
            Direction::North
        )
    }

    #[test]
    fn test_direction_l_2() {
        assert_eq!(
            get_new_direction(
                &Direction::East,
                Movement {
                    direction: Direction::Left,
                    amount: 180
                }
            ),
            Direction::West
        )
    }

    #[test]
    fn test_direction_l_3() {
        assert_eq!(
            get_new_direction(
                &Direction::East,
                Movement {
                    direction: Direction::Left,
                    amount: 270
                }
            ),
            Direction::South
        )
    }

    #[test]
    fn test_rotate_1() {
        assert_eq!(
            rotate_waypoint(
                Position { x: 10, y: 1 },
                Movement {
                    direction: Direction::Right,
                    amount: 90
                }
            ),
            Position { x: 1, y: -10 },
        )
    }

    #[test]
    fn test_rotate_2() {
        assert_eq!(
            rotate_waypoint(
                Position { x: 10, y: 1 },
                Movement {
                    direction: Direction::Right,
                    amount: 270
                }
            ),
            Position { x: -1, y: 10 },
        )
    }

    #[test]
    fn test_rotate_3() {
        assert_eq!(
            rotate_waypoint(
                Position { x: 10, y: 1 },
                Movement {
                    direction: Direction::Left,
                    amount: 90
                }
            ),
            Position { x: -1, y: 10 },
        )
    }

    #[test]
    fn test_rotate_4() {
        assert_eq!(
            rotate_waypoint(
                Position { x: 10, y: 1 },
                Movement {
                    direction: Direction::Left,
                    amount: 270
                }
            ),
            Position { x: 1, y: -10 },
        )
    }
}
