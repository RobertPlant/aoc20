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
            .split_terminator(",")
            .map(|b| b.parse().unwrap_or(0))
            .collect(),
    )
}

fn parse_p2(input: &'static str) -> Vec<(i64, i64)> {
    input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, l)| l.parse().ok().map(|l| (l - i as i64, l)))
        .collect()
}

fn find_next_depature(start_time: u64, buses: &Buses) -> NextBus {
    let mut offset = 0;

    loop {
        for bus in buses {
            if *bus == 0 {
                continue;
            }

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

fn find_departure_alignment(input: &'static str) -> i64 {
    let data = parse_p2(input);

    let prod = data.iter().map(|n| n.1).product::<i64>();

    println!("prod {:?}", prod);
    data.iter()
        .map(|(interval, offset)| {
            let p = prod / offset;
            interval * ((egcd(p, *offset).1 % offset + offset) % offset) * p
        })
        .sum::<i64>()
        % prod
}

#[inline(always)]
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn main() {
    let input_data = input::get_input();

    println!("Part1: {:?}", count_wait_distance(input_data));
    println!("Part2: {:?}", find_departure_alignment(input_data));
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

    #[test]
    fn test_part2() {
        assert_eq!(find_departure_alignment(TEST_INPUT), 1068781)
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(find_departure_alignment("1\n67,7,59,61"), 754018)
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(find_departure_alignment("1\n67,x,7,59,61"), 779210)
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(find_departure_alignment("1\n67,7,x,59,61"), 1261476)
    }

    #[test]
    fn test_part2_4() {
        assert_eq!(find_departure_alignment("1\n1789,37,47,1889"), 1202161486)
    }
}
