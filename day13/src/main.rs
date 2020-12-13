mod input;

type Buses = Vec<u64>;

#[derive(Debug)]
struct NextBus {
    wait: u64,
    id: u64,
}

fn parse(input: &'static str) -> (u64, Buses) {
    let mut lines = input.lines();

    (
        lines.next().unwrap().parse().unwrap(),
        lines
            .next()
            .unwrap()
            .replace("x,", "")
            .split_terminator(",")
            .map(|b| b.parse().unwrap_or(0))
            .collect(),
    )
}

fn find_next_depature(start_time: u64, buses: &Buses) -> NextBus {
    let mut offset = 0;

    loop {
        for bus in buses {
            if (start_time + offset) % bus == 0 {
                return NextBus {
                    wait: offset,
                    id: *bus,
                };
            }
        }

        offset += 1;
    }
}

fn count_wait_distance(input: &'static str) -> u64 {
    let (start_time, buses) = parse(input);
    let next_bus = find_next_depature(start_time, &buses);

    next_bus.id * next_bus.wait
}

fn main() {
    let input_data = input::get_input();

    println!("Part1: {:?}", count_wait_distance(input_data))
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test() {
        assert_eq!(count_wait_distance(TEST_INPUT), 295)
    }
}
