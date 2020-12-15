mod input;

fn parse(input: &'static str) -> Vec<usize> {
    input.split(',').map(|i| i.parse().unwrap()).collect()
}

fn get(input: &'static str, target: usize) -> usize {
    let data = parse(input);
    let mut spoken = vec![0; target];
    let mut last = 0;

    data.iter().enumerate().for_each(|(index, number)| {
        spoken[*number] = index + 1;
        last = *number;
    });

    for v in data.len()..target {
        let spoken_at = spoken[last];
        spoken[last] = v;

        if spoken_at != 0 {
            last = v - spoken_at;
        } else {
            last = 0
        }
    }

    last
}

fn main() {
    let input_data = input::get_input();

    println!("Part1: {:?}", get(input_data, 2020));
    println!("Part2: {:?}", get(input_data, 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(get("0,3,6", 2020), 436)
    }

    #[test]
    fn test_part2() {
        assert_eq!(get("0,3,6", 30000000), 175594);
        assert_eq!(get("1,3,2", 30000000), 2578);
        assert_eq!(get("2,1,3", 30000000), 3544142);
        assert_eq!(get("1,2,3", 30000000), 261214);
        assert_eq!(get("2,3,1", 30000000), 6895259);
        assert_eq!(get("3,2,1", 30000000), 18);
        assert_eq!(get("3,1,2", 30000000), 362);
    }
}
