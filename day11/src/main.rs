mod input;

#[derive(Debug)]
struct Position {
    line: u64,
    seat: u64,
}

#[derive(Debug, PartialEq, Clone)]
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
            Some(lp) => lp,
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

fn count_seatable(input: &'static str) -> u64 {
    let data = parse(input);
    let mut matching = false;
    let mut previous_state = data.clone();

    while !matching {
        let mut new_state = previous_state.clone();
        for (line_index, line) in previous_state.iter().enumerate() {
            for (seat_index, seat) in line.iter().enumerate() {
                let adjacents = count_seated_adjacently(
                    Position {
                        line: line_index as u64,
                        seat: seat_index as u64,
                    },
                    &previous_state,
                );

                if *seat == State::Empty && adjacents == 0 {
                    new_state[line_index][seat_index] = State::Occupied;
                } else if *seat == State::Occupied && adjacents >= 4 {
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

    println!("Part1 {:?}", count_seatable(input_data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            count_seatable(
                "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            ),
            37
        )
    }
}
