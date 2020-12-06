use std::iter::FromIterator;

mod input;

fn count(input: &'static str) -> usize {
    let mut count = 0;

    for group in input.split_terminator("\n\n") {
        let flat_group = group.replace("\n", "");
        let mut collect_char = flat_group.chars().collect::<Vec<char>>();
        collect_char.sort();
        collect_char.dedup();

        count = count + collect_char.len();
    }

    count
}

fn count_part2(input: &'static str) -> usize {
    let mut count = 0;

    for group in input.split_terminator("\n\n") {
        let group_member_count = group.split_terminator("\n").count();
        let flat_group = group.replace("\n", "");
        let mut collect_char = flat_group.chars().collect::<Vec<char>>();
        collect_char.sort();
        let sorted_flat_string = String::from_iter(collect_char.clone());
        collect_char.dedup();

        for char in collect_char {
            match sorted_flat_string.find(&String::from_iter(vec![char; group_member_count])) {
                Some(_) => count = count + 1,
                None => continue,
            };
        }
    }

    count
}

fn main() {
    let input_data = input::get_input();

    let total = count(input_data);

    println!("Total {:?}", total);

    let part2_total = count_part2(input_data);

    println!("Part2 Total {:?}", part2_total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            count(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            ),
            11
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            count_part2(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            ),
            6
        )
    }
}
