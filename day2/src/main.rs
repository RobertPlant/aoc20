use std::ops::BitXor;

mod input;

fn is_valid(pass: &input::PasswordInput) -> bool {
    let mut count = 0;
    let chars: Vec<char> = pass.password.chars().collect();

    for c in chars {
        if c == pass.char {
            count = count + 1;
        }
    }

    count <= pass.max && count >= pass.min
}

#[derive(Debug, PartialEq)]
struct Scalar(bool);

impl BitXor for Scalar {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

fn is_valid_part2(pass: &input::PasswordInput) -> Scalar {
    let chars: Vec<char> = pass.password.chars().collect();
    let mut key = 0;
    let mut position_one_match = false;
    let mut position_two_match = false;

    for c in chars {
        key = key + 1;

        if pass.min == key && pass.char == c {
            position_one_match = true;
        }

        if pass.max == key && pass.char == c {
            position_two_match = true;
        }
    }

    Scalar(position_one_match) ^ Scalar(position_two_match)
}

fn main() {
    let input_data = input::get_input();
    let length = input_data.len();
    let mut part_one_count = 0;
    let mut part_two_count = 0;

    for i in input_data {
        if is_valid(&i) {
            part_one_count = part_one_count + 1;
        }
        if is_valid_part2(&i) == Scalar(true) {
            part_two_count = part_two_count + 1;
        }
    }

    println!("Part1 Valid: {:?}", part_one_count);
    println!("Part1 Invalid: {:?}", length - part_one_count);

    println!("Part2 Valid: {:?}", part_two_count);
    println!("Part2 Invalid: {:?}", length - part_two_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(
            is_valid(&input::PasswordInput {
                min: 1,
                max: 3,
                char: 'a',
                password: "abcde"
            }),
            true
        )
    }

    #[test]
    fn test_cdefg() {
        assert_eq!(
            is_valid(&input::PasswordInput {
                min: 1,
                max: 3,
                char: 'b',
                password: "cdefg"
            }),
            false
        )
    }

    #[test]
    fn test_ccccccccc() {
        assert_eq!(
            is_valid(&input::PasswordInput {
                min: 2,
                max: 9,
                char: 'c',
                password: "ccccccccc"
            }),
            true
        )
    }
    #[test]
    fn test_abcde_part2() {
        assert_eq!(
            is_valid_part2(&input::PasswordInput {
                min: 1,
                max: 3,
                char: 'a',
                password: "abcde"
            }),
            Scalar(true)
        )
    }

    #[test]
    fn test_cdefg_part2() {
        assert_eq!(
            is_valid_part2(&input::PasswordInput {
                min: 1,
                max: 3,
                char: 'b',
                password: "cdefg"
            }),
            Scalar(false)
        )
    }

    #[test]
    fn test_ccccccccc_part2() {
        assert_eq!(
            is_valid_part2(&input::PasswordInput {
                min: 2,
                max: 9,
                char: 'c',
                password: "ccccccccc"
            }),
            Scalar(false)
        )
    }
}
