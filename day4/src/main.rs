use hex::decode;
use regex::Regex;

mod input;

fn check(input: &'static str, required: Vec<&str>, optional: Vec<&str>) -> usize {
    let mut valid = 0;

    for line in input.split_terminator("\n\n") {
        let stripped_line = line.replace("\n", " ");

        let mut found_keys = vec![];
        for split in stripped_line.split_whitespace() {
            let mut key_value = split.split(":");
            let key = key_value.next().unwrap();

            found_keys.push(key);
        }

        found_keys.sort();

        if found_keys == required || found_keys == optional {
            valid = valid + 1;
        }
    }

    valid
}

fn check_part2(input: &'static str, required: Vec<&str>, optional: Vec<&str>) -> usize {
    let mut valid = 0;
    let nine_digits = Regex::new(r"^\d{9}$").unwrap();

    'line: for line in input.split_terminator("\n\n") {
        let stripped_line = line.replace("\n", " ");

        let mut found_keys = vec![];
        for split in stripped_line.split_whitespace() {
            let mut key_value = split.split(":");
            let key = key_value.next().unwrap();
            let value = key_value.next().unwrap();

            found_keys.push(key);

            let valid = match key {
                "byr" => {
                    let int_value = value.parse::<i32>().unwrap_or(0);

                    int_value >= 1920 && int_value <= 2002
                }
                "iyr" => {
                    let int_value = value.parse::<i32>().unwrap_or(0);

                    int_value >= 2010 && int_value <= 2020
                }
                "eyr" => {
                    let int_value = value.parse::<i32>().unwrap_or(0);

                    int_value >= 2020 && int_value <= 2030
                }
                "hgt" => {
                    let height_type = &value[value.len() - 2..];
                    let int_value = value[..value.len() - 2].parse::<i32>().unwrap_or(0);

                    match height_type {
                        "cm" => int_value >= 150 && int_value <= 193,
                        "in" => int_value >= 59 && int_value <= 76,
                        _ => false,
                    }
                }
                "hcl" => {
                    let prefix = &value[..1];
                    let hex = &value[1..];

                    prefix == "#" && hex.len() == 6 && decode(hex).is_ok()
                }
                "ecl" => vec!["amb", "amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                    .into_iter()
                    .find(|&c| c == value)
                    .is_some(),
                "pid" => nine_digits.is_match(value),
                "cid" => true,
                _ => false,
            };

            if !valid {
                continue 'line;
            }
        }

        found_keys.sort();

        if found_keys == required || found_keys == optional {
            valid = valid + 1;
        }
    }

    valid
}

fn main() {
    let input_data = input::get_input();

    let valid = check(
        input_data,
        vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
        vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
    );

    let valid_part2 = check_part2(
        input_data,
        vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
        vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
    );

    println!("Valid passports: {:?}", valid);
    println!("Valid passports part 2: {:?}", valid_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            check(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in",
                vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
                vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
            ),
            2
        )
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(
            check_part2(
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f",
                vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
                vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
            ),
            1
        )
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(
            check_part2(
                "eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
                vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
                vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
            ),
            1
        )
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(
            check_part2(
                "eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
                vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
                vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
            ),
            1
        )
    }

    #[test]
    fn test_part2_4() {
        assert_eq!(
            check_part2(
                "hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022",
                vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
                vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
            ),
            1
        )
    }

    #[test]
    fn test_part2_5() {
        assert_eq!(
            check_part2(
                "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
                vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
                vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
            ),
            1
        )
    }

    #[test]
    fn test_part2_6() {
        assert_eq!(
            check_part2(
                "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
                vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
                vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
            ),
            0
        )
    }

    #[test]
    fn test_part2_7() {
        assert_eq!(
            check_part2(
                "iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946",
                vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
                vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
            ),
            0
        )
    }

    #[test]
    fn test_part2_8() {
        assert_eq!(
            check_part2(
                "hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
                vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
                vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
            ),
            0
        )
    }
    #[test]
    fn test_part2_9() {
        assert_eq!(
            check_part2(
                "hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007",
                vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
                vec!["byr", "cid", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
            ),
            0
        )
    }
}
