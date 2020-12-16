mod input;

use regex::Regex;

#[derive(Debug, Clone)]
struct Ticket {
    fields: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Rule {
    min: usize,
    max: usize,
    pick: bool,
}

type Data = (Vec<Rule>, Ticket, Vec<Ticket>);
type MatchedData = (Vec<(Rule, Rule)>, Ticket, Vec<Ticket>);

fn parse(input: &'static str, multiply: Option<&'static str>) -> Data {
    let mut rules = vec![];
    let mut ticket = Ticket { fields: vec![] };
    let mut nearby = vec![];
    let mut input_type = 0;
    let select_only = multiply.unwrap_or("*");

    let rule_regex = Regex::new(r"(\d*)-(\d*)").unwrap();

    for line in input.split("\n\n") {
        match input_type {
            0 => {
                for item in line.lines() {
                    for rule in rule_regex.captures_iter(item) {
                        rules.push(Rule {
                            min: rule[1].parse().unwrap(),
                            max: rule[2].parse().unwrap(),
                            pick: select_only == "*" || item.contains(select_only),
                        })
                    }
                }
            }
            1 => {
                ticket = Ticket {
                    fields: line
                        .replace("your ticket:\n", "")
                        .split(",")
                        .map(|i| i.parse().unwrap())
                        .collect(),
                }
            }
            2 => {
                for line in line.lines().skip(1) {
                    nearby.push(Ticket {
                        fields: line.split(",").map(|i| i.parse().unwrap()).collect(),
                    })
                }
            }
            _ => (),
        }

        input_type += 1;
    }

    (rules, ticket, nearby)
}

fn count_invalid_fields(input: &'static str) -> usize {
    let (rules, _, others) = parse(input, None);
    let mut count = 0;

    for other in others {
        for field in other.fields {
            let mut valid = false;

            for rule in &rules {
                if field >= rule.min && field <= rule.max {
                    valid = true;
                }
            }

            if !valid {
                count += field;
            }
        }
    }

    count
}

fn remove_invalid_tickets(input: &'static str, multiply: &'static str) -> MatchedData {
    let (rules, ticket, others) = parse(input, Some(multiply));
    let mut valid_others = vec![];
    let mut matched_rules = vec![];
    let field_length = ticket.fields.len();
    for (i, rule) in rules.iter().enumerate() {
        if i % 2 == 0 {
            matched_rules.push((rule.clone(), rules[i + 1].clone()));
        }
    }

    for other in &others {
        let mut valid = 0;

        for field in &other.fields {
            let mut field_matched = false;
            for (rule1, rule2) in &matched_rules {
                if (field >= &rule1.min && field <= &rule1.max)
                    || (field >= &rule2.min && field <= &rule2.max)
                {
                    if !field_matched {
                        valid += 1;
                        field_matched = true;
                    }
                }
            }
        }

        if valid >= field_length {
            valid_others.push(other.clone());
        }
    }

    (matched_rules, ticket, valid_others)
}

fn part2(input: &'static str, multiply: &'static str) -> usize {
    let (rules, ticket, others) = remove_invalid_tickets(input, multiply);
    let count_of_fields = ticket.fields.len();
    let count_of_others = others.len();

    if count_of_others == 0 {
        return 0;
    }

    let mut output = vec![];
    let mut valid: Vec<Vec<usize>> = vec![vec![]; count_of_fields];

    for other in others {
        for (field_index, field) in other.fields.iter().enumerate() {
            for (rule_index, (rule1, rule2)) in rules.iter().enumerate() {
                if (field >= &rule1.min && field <= &rule1.max)
                    || (field >= &rule2.min && field <= &rule2.max)
                {
                    valid[rule_index].push(field_index);
                }
            }
        }
    }

    for v in valid {
        for (rule_index, rule) in rules.iter().enumerate() {
            if rule.0.pick && v.iter().filter(|&&i| i == rule_index).count() >= count_of_others {
                output.push(ticket.fields[rule_index]);
            }
        }
    }

    output.sort();
    output.dedup();

    output.iter().product::<usize>()
}

fn main() {
    let input_data = input::get_input();

    println!("Part1: {:?}", count_invalid_fields(input_data));
    println!("Part2: {:?}", part2(input_data, "departure"));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test() {
        assert_eq!(count_invalid_fields(TEST_INPUT), 71)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT, "class"), 7)
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(TEST_INPUT, "row"), 1)
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(
            part2(
                "class: 1-3 or 5-7
row: 6-11 or 33-44
class: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12",
                "class"
            ),
            7 * 14
        )
    }

    #[test]
    fn test_part2_4() {
        assert_eq!(
            part2(
                "class: 1-3 or 5-7
row: 6-11 or 33-44
class: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
500,6,12",
                "class"
            ),
            0
        )
    }

    #[test]
    fn test_part2_5() {
        assert_eq!(
            part2(
                "departure location: 34-269 or 286-964
departure station: 27-584 or 609-973
departure platform: 49-135 or 155-974
departure track: 36-248 or 255-954
departure date: 50-373 or 381-974
departure time: 49-454 or 472-967
arrival location: 33-900 or 925-968
arrival station: 46-699 or 706-965
arrival platform: 42-656 or 666-967
arrival track: 49-408 or 425-950
class: 30-626 or 651-957
duration: 43-109 or 127-964
price: 33-778 or 795-952
route: 37-296 or 315-966
row: 28-318 or 342-965
seat: 33-189 or 208-959
train: 49-536 or 552-968
type: 46-749 or 772-949
wagon: 29-386 or 401-954
zone: 34-344 or 368-954

your ticket:
109,101,79,127,71,59,67,61,173,157,163,103,83,97,73,167,53,107,89,131

nearby tickets:
994,873,258,172,884,554,258,712,264,183,728,255,824,54,740,838,614,850,881,873",
                "class"
            ),
            0
        )
    }
}
