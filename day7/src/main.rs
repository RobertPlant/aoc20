mod input;
use std::collections::HashMap;

fn delve(
    bags: &HashMap<&str, Vec<&str>>,
    bag_type: &str,
    search: &'static str,
    found_input: bool,
) -> bool {
    let mut found = found_input;

    if bags
        .get(bag_type)
        .unwrap()
        .into_iter()
        .find(|&&t| t == search)
        .is_some()
    {
        found = true;
    } else {
        for bag in bags.get(bag_type).unwrap().into_iter() {
            if delve(bags, bag, search, found) {
                found = true
            }
        }
    }

    found
}

fn delve_part2(
    bags: &HashMap<&str, Vec<(usize, &str)>>,
    search_target: &str,
    found_input: usize,
) -> usize {
    let mut found = found_input;

    for bag in bags.get(search_target).unwrap().into_iter() {
        let loop_count = bag.0 * 1;
        found = found + loop_count + (bag.0 * delve_part2(bags, bag.1, 0));
    }

    found
}

fn count_can_hold(input: &'static str, search_target: &'static str) -> usize {
    let mut count_of = 0;
    let mut bags = HashMap::new();

    for group in input.split_terminator("\n") {
        let mut source_split = group.split_terminator(" contain");
        let source = source_split
            .next()
            .unwrap()
            .split_terminator(" bag")
            .next()
            .unwrap();

        let mut targets = vec![];
        for target in source_split.next().unwrap().split_terminator(",") {
            let count = target.trim().get(..1).unwrap().parse::<i32>().unwrap_or(0);
            let target_name = target
                .trim()
                .get(1..)
                .unwrap()
                .trim()
                .split_terminator(" bag")
                .next()
                .unwrap();

            if count > 0 {
                targets.push(target_name);
            }
        }

        bags.insert(source, targets);
    }

    for group in input.split_terminator("\n") {
        let mut source_split = group.split_terminator(" contain");
        let source = source_split
            .next()
            .unwrap()
            .split_terminator(" bag")
            .next()
            .unwrap();

        if delve(&bags, source, search_target, false) {
            count_of = count_of + 1
        }
    }

    count_of
}

fn count_can_hold_part2(input: &'static str, search_target: &'static str) -> usize {
    let mut count_of = 0;
    let mut bags = HashMap::new();

    for group in input.split_terminator("\n") {
        let mut source_split = group.split_terminator(" contain");
        let source = source_split
            .next()
            .unwrap()
            .split_terminator(" bag")
            .next()
            .unwrap();

        let mut targets = vec![];
        for target in source_split.next().unwrap().split_terminator(",") {
            let count = target
                .trim()
                .get(..1)
                .unwrap()
                .parse::<usize>()
                .unwrap_or(0);
            let target_name = target
                .trim()
                .get(1..)
                .unwrap()
                .trim()
                .split_terminator(" bag")
                .next()
                .unwrap();

            if count > 0 {
                targets.push((count, target_name));
            }
        }

        bags.insert(source, targets);
    }

    for group in input.split_terminator("\n") {
        let mut source_split = group.split_terminator(" contain");
        let source = source_split
            .next()
            .unwrap()
            .split_terminator(" bag")
            .next()
            .unwrap();

        if source == search_target {
            count_of = count_of + delve_part2(&bags, search_target, 0);
        }
    }

    count_of
}

fn main() {
    let input_data = input::get_input();

    let total = count_can_hold(input_data, "shiny gold");

    println!("Found {:?}", total);

    let total_part2 = count_can_hold_part2(input_data, "shiny gold");

    println!("Found part2 {:?}", total_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            count_can_hold(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.",
                "shiny gold"
            ),
            4
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            count_can_hold_part2(
                "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.",
                "shiny gold"
            ),
            126
        )
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(
            count_can_hold_part2(
                "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.",
                "dark blue"
            ),
            2
        )
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(
            count_can_hold_part2(
                "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.",
                "dark green"
            ),
            6
        )
    }

    #[test]
    fn test_part2_() {
        assert_eq!(
            count_can_hold_part2(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.",
                "shiny gold"
            ),
            32
        )
    }

    #[test]
    fn test_part2__() {
        assert_eq!(
            count_can_hold_part2(
                "dark olive bags contain no other bags..
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
vibrant plum bags contain no other bags",
                "shiny gold"
            ),
            3
        )
    }
}
