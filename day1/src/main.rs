mod input;

const TARGET: isize = 2020;

fn find_and_multiply(input: Vec<isize>) -> isize {
    for i in 0..input.len() {
        let mut into_iter = input.clone().into_iter();
        match into_iter.find(|&x| x == TARGET - input[i]) {
            Some(found) => return found * input[i],
            None => continue,
        }
    }

    return 0;
}

fn find_and_multiply_part2(input: Vec<isize>) -> isize {
    for i in 0..input.len() {
        for k in 0..input.len() {
            if i == k {
                continue;
            }

            let mut into_iter = input.clone().into_iter();
            match into_iter.find(|&x| x == TARGET - input[i] - input[k]) {
                Some(found) => return found * input[i] * input[k],
                None => continue,
            }
        }
    }

    return 0;
}

fn main() {
    let input_data = input::get_input();

    println!("Result: {:?}", find_and_multiply(input_data.clone()));
    println!(
        "Result Part2: {:?}",
        find_and_multiply_part2(input_data.clone())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            find_and_multiply(vec![1721, 979, 366, 299, 675, 1456]),
            514579
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            find_and_multiply_part2(vec![1721, 979, 366, 299, 675, 1456]),
            241861950
        )
    }
}
