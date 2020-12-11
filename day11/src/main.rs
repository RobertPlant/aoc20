mod input;

#[derive(Debug, Clone)]
struct Position {
    line: u64,
    seat: u64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Floor,
    Empty,
    Occupied,
    Unknown,
}

type Matrix = Vec<Vec<State>>;

fn parse(input: &'static str) -> Vec<Vec<State>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => State::Floor,
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    _ => State::Unknown,
                })
                .collect()
        })
        .collect()
}

fn add(u: u64, i: i8) -> Option<u64> {
    if i.is_negative() {
        u.checked_sub(i.abs() as u64)
    } else {
        Some(u + i as u64)
    }
}

fn count_seated_adjacently(position: Position, data: &Matrix) -> u64 {
    let mut count = 0;
    let search = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for surrounding_position in search {
        let line_position = match add(position.line, surrounding_position.0) {
            Some(lp) => lp,
            None => continue,
        };
        let line = match data.get(line_position as usize) {
            Some(l) => l,
            None => continue,
        };
        let seat_position = match add(position.seat, surrounding_position.1) {
            Some(sp) => sp,
            None => continue,
        };
        let seat = match line.get(seat_position as usize) {
            Some(s) => s,
            None => continue,
        };

        if *seat == State::Occupied {
            count += 1;
        }
    }

    count
}

fn count_seated_directionally(position: Position, data: &Matrix) -> u64 {
    let mut count = 0;
    let search = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for surrounding_position in search {
        if find_first_non_floor_by_vector(&position, surrounding_position, data) == State::Occupied
        {
            count += 1;
        }
    }

    count
}

fn find_first_non_floor_by_vector(position: &Position, vector: (i8, i8), data: &Matrix) -> State {
    let mut current_position = position.clone();

    loop {
        let line_position = match add(current_position.line, vector.0) {
            Some(lp) => lp,
            None => return State::Floor,
        };
        let line = match data.get(line_position as usize) {
            Some(l) => l,
            None => return State::Floor,
        };
        let seat_position = match add(current_position.seat, vector.1) {
            Some(sp) => sp,
            None => return State::Floor,
        };
        let seat = *match line.get(seat_position as usize) {
            Some(s) => s,
            None => return State::Floor,
        };

        if seat == State::Floor {
            current_position = Position {
                line: line_position,
                seat: seat_position,
            }
        } else {
            return seat;
        }
    }
}

fn count_seated(data: &Matrix) -> u64 {
    let mut count = 0;

    for line in data {
        for seat in line {
            if *seat == State::Occupied {
                count += 1;
            }
        }
    }

    count
}

fn count_seatable(input: &'static str, part2: Option<bool>) -> u64 {
    let is_part2 = part2.unwrap_or(false);
    let data = parse(input);
    let mut matching = false;
    let mut previous_state = data.clone();
    let acceptable_adjacents = match is_part2 {
        true => 5,
        false => 4,
    };

    while !matching {
        let mut new_state = previous_state.clone();
        for (line_index, line) in previous_state.iter().enumerate() {
            for (seat_index, seat) in line.iter().enumerate() {
                let adjacents;
                if is_part2 {
                    adjacents = count_seated_directionally(
                        Position {
                            line: line_index as u64,
                            seat: seat_index as u64,
                        },
                        &previous_state,
                    );
                } else {
                    adjacents = count_seated_adjacently(
                        Position {
                            line: line_index as u64,
                            seat: seat_index as u64,
                        },
                        &previous_state,
                    );
                }

                if *seat == State::Empty && adjacents == 0 {
                    new_state[line_index][seat_index] = State::Occupied;
                } else if *seat == State::Occupied && adjacents >= acceptable_adjacents {
                    new_state[line_index][seat_index] = State::Empty;
                }
            }
        }

        if count_seated(&new_state) == count_seated(&previous_state) {
            matching = true;
        }

        previous_state = new_state.clone();
    }

    count_seated(&previous_state)
}

fn main() {
    let input_data = input::get_input();

    println!("Part1 {:?}", count_seatable(input_data, None));
    println!("Part2 {:?}", count_seatable(input_data, Some(true)));
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test() {
        assert_eq!(count_seatable(TEST_INPUT, None), 37)
    }

    #[test]
    fn test_part2() {
        assert_eq!(count_seatable(TEST_INPUT, Some(true)), 26)
    }
}
