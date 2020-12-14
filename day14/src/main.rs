mod input;

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Value {
    mask: &'static str,
    mem: u64,
    value: u64,
}

fn parse(input: &'static str) -> Vec<Value> {
    let mut vec = vec![];
    let mut mask = "";

    for line in input.lines() {
        match line.get(..3).unwrap() {
            "mas" => {
                mask = line.get(7..).unwrap();
            }
            "mem" => {
                let d = line.replace("mem[", "").replace("]", "");
                let s = d.split(" = ").collect::<Vec<&str>>();

                vec.push(Value {
                    mask: mask,
                    mem: s[0].parse().unwrap(),
                    value: s[1].parse().unwrap(),
                });
            }
            _ => (),
        }
    }

    vec
}

fn calc(input: &'static str) -> usize {
    let data = parse(input);
    let mut hash = HashMap::new();
    let mut total = 0;

    for value in data {
        let mut masked = String::from("");
        for (i, m) in value.mask.chars().rev().enumerate() {
            let source = 0b1 << i & value.value;

            match m {
                'X' => {
                    let s = match source > 0 {
                        true => "1",
                        false => "0",
                    };
                    masked = s.to_owned() + &masked;
                }
                '1' => {
                    masked = "1".to_owned() + &masked;
                }
                '0' => {
                    masked = "0".to_owned() + &masked;
                }
                _ => (),
            }
        }
        hash.insert(value.mem, usize::from_str_radix(&masked, 2).unwrap());
    }

    for i in hash {
        total += i.1;
    }

    total
}

fn main() {
    let input_data = input::get_input();

    println!("Part1: {:?}", calc(input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test() {
        assert_eq!(calc(TEST_INPUT), 165)
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(
                "mask = 0010X01001X010000110100000X000010X11
mem[41717] = 288
mem[54146] = 1656
mask = 01X10101X11X01XX01X000011X1000110110
mem[29142] = 13227025
mem[32455] = 1814"
            ),
            vec![
                Value {
                    mask: "0010X01001X010000110100000X000010X11",
                    mem: 41717,
                    value: 288,
                },
                Value {
                    mask: "0010X01001X010000110100000X000010X11",
                    mem: 54146,
                    value: 1656,
                },
                Value {
                    mask: "01X10101X11X01XX01X000011X1000110110",
                    mem: 29142,
                    value: 13227025,
                },
                Value {
                    mask: "01X10101X11X01XX01X000011X1000110110",
                    mem: 32455,
                    value: 1814,
                },
            ]
        )
    }
}
