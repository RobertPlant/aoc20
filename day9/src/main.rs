mod input;

fn build(input: &'static str) -> Vec<usize> {
    let mut vec = vec![];

    for line in input.split_terminator("\n") {
        vec.push(line.parse::<usize>().expect("should have some data"));
    }

    vec
}

fn get_first(input: &'static str, preamble: usize) -> usize {
    let data = build(input);
    let mut range_start = 0;

    'top: for i in &data[preamble..] {
        let window = range_start + preamble;
        for k in &data[range_start..window] {
            match &data[range_start..window].iter().find(|&x| x + k == *i) {
                Some(_) => {
                    range_start = range_start + 1;
                    continue 'top;
                }
                None => (),
            }
        }

        return *i;
    }

    0
}

fn part2(input: &'static str, target: usize) -> usize {
    let data = build(input);
    let mut range_start = 0;

    for _ in &data[range_start..] {
        let mut current_highest = 0;
        let mut count = 0;

        for x in &data[range_start..] {
            if current_highest < *x {
                current_highest = *x;
            }

            count = count + x;
            if count == target {
                return data[range_start] + current_highest;
            }
        }

        range_start = range_start + 1;
    }

    0
}

fn main() {
    let input_data = input::get_input();

    let part1 = get_first(input_data, 25);
    println!("Part1: {:?}", part1);
    println!("Part2: {:?}", part2(input_data, part1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            get_first(
                "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
                5
            ),
            127
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
                127,
            ),
            62
        )
    }
}
