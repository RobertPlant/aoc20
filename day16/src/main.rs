mod input;

use regex::Regex;

#[derive(Debug)]
struct Ticket {
    fields: Vec<usize>,
}

#[derive(Debug)]
struct Rule {
    min: usize,
    max: usize,
}

fn parse(input: &'static str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut rules = vec![];
    let mut ticket = Ticket { fields: vec![] };
    let mut nearby = vec![];
    let mut input_type = 0;

    let rule_regex = Regex::new(r"(\d*)-(\d*)").unwrap();

    for line in input.split("\n\n") {
        match input_type {
            0 => {
                for rule in rule_regex.captures_iter(line) {
                    rules.push(Rule {
                        min: rule[1].parse().unwrap(),
                        max: rule[2].parse().unwrap(),
                    })
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
    let (rules, _, others) = parse(input);
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

fn main() {
    let input_data = input::get_input();

    println!("Part1: {:?}", count_invalid_fields(input_data));
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
}
