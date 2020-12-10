mod input;

fn build(input: &'static str) -> Vec<u64> {
    let mut vec = vec![];

    vec.push(0);
    for line in input.split_terminator("\n") {
        vec.push(line.parse::<u64>().expect("should have some data"));
    }

    vec.sort();
    vec.push(vec.last().unwrap() + 3);

    vec
}

fn calc(input: &'static str) -> u64 {
    let data = build(input);
    let mut previous_value = 0;
    let mut one_gap = 0;
    let mut three_gap = 0;

    for i in data {
        match i - previous_value {
            1 => {
                one_gap += 1;
            }
            3 => {
                three_gap += 1;
            }
            _ => (),
        }
        previous_value = i;
    }

    one_gap * three_gap
}

fn multiply_sub_arrangements(data: &Vec<u64>) -> u64 {
    if data.len() <= 1 {
        return 1;
    }

    let start = data[0];
    let target = data[data.len() - 1];
    let mut count = 0;

    for i in 1usize..=3 {
        let next = data[i];

        if next - start > 3 {
            break;
        } else if next == target {
            count += 1;
            break;
        } else {
            count += multiply_sub_arrangements(&data[i..].to_vec());
        }
    }

    count
}

fn calc_p2(input: &'static str) -> u64 {
    let data = build(input);
    let mut start = 0;
    let mut end = 1;
    let mut count = 1;

    loop {
        while data[end] - data[end - 1] < 3 && end + 1 < data.len() {
            end += 1;
        }

        count *= multiply_sub_arrangements(&data[start..end].to_vec());
        start = end;

        while data[start] - data[start - 1] >= 3 {
            start += 1;

            if start >= data.len() {
                return count;
            }
        }

        end = start;
        start -= 1;
    }
}

fn main() {
    let input_data = input::get_input();

    println!("Part1: {:?}", calc(input_data));
    println!("Part2: {:?}", calc_p2(input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_short() {
        assert_eq!(
            calc(
                "16
10
15
5
1
11
7
19
6
12
4"
            ),
            7 * 5
        )
    }

    #[test]
    fn test_p1_longer() {
        assert_eq!(
            calc(
                "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
            ),
            22 * 10
        )
    }

    #[test]
    fn test_p2_short() {
        assert_eq!(
            calc_p2(
                "16
10
15
5
1
11
7
19
6
12
4"
            ),
            8
        )
    }

    #[test]
    fn test_p2_longer() {
        assert_eq!(
            calc_p2(
                "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
            ),
            19208
        )
    }
}
