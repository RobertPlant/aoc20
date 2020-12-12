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

#[derive(Debug)]
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

fn main() {
    let input_data = input::get_input();

    println!("Part1: {:?}", follow_navigation(input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            follow_navigation(
                "F10
N3
F7
R90
F11"
            ),
            25
        )
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
}
