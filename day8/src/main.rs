mod input;

use std::collections::HashMap;

#[derive(Debug)]
struct Operation {
    op: &'static str,
    count: i32,
}

fn exec(input: &'static str) -> i32 {
    let program = build(input);
    let count = 0;
    let visited_keys = vec![];

    run_op(&program, 0, count, visited_keys)
}

fn run_op(
    program: &HashMap<i32, Operation>,
    key: i32,
    mut count: i32,
    mut visited_keys: Vec<i32>,
) -> i32 {
    let operation = program.get(&key).unwrap();
    if visited_keys.iter().find(|&&k| k == key).is_some() {
        return count;
    } else {
        visited_keys.push(key);
    }

    match operation.op {
        "nop" => run_op(&program, key + 1, count, visited_keys),
        "acc" => {
            count = count + operation.count;

            run_op(&program, key + 1, count, visited_keys)
        }
        "jmp" => run_op(&program, key + operation.count, count, visited_keys),
        _ => 0,
    }
}

fn exec_part2(input: &'static str) -> i32 {
    let program = build(input);
    let mut visited_keys = vec![];
    let mut replaced_keys = vec![];
    let mut replaced_this_iter = false;
    let mut found = false;
    let mut found_count = 0;

    while !found {
        match run_op_part2(
            &program,
            0,
            0,
            &mut visited_keys,
            &mut replaced_keys,
            &mut replaced_this_iter,
        ) {
            Some(value) => {
                found = true;
                found_count = value;
            }
            None => (),
        };
        visited_keys = vec![];
        replaced_this_iter = false;
    }

    found_count
}

fn run_op_part2(
    program: &HashMap<i32, Operation>,
    key: i32,
    mut count: i32,
    visited_keys: &mut Vec<i32>,
    replaced_keys: &mut Vec<i32>,
    replaced_this_iter: &mut bool,
) -> Option<i32> {
    let operation;
    let program_key = program.get(&key);

    if program_key.is_some() {
        operation = program_key.unwrap();
    } else {
        return Some(count);
    }

    if visited_keys.iter().find(|&&k| k == key).is_some() {
        return None;
    } else {
        visited_keys.push(key);
    }

    let mut op = operation.op.clone();
    if replaced_keys.iter().find(|&&k| k == key).is_none()
        && (op == "nop" || op == "jmp")
        && !*replaced_this_iter
    {
        if op == "nop" {
            op = "jmp";
        } else {
            op = "nop";
        }
        replaced_keys.push(key);
        *replaced_this_iter = true;
    }

    match op {
        "nop" => run_op_part2(
            &program,
            key + 1,
            count,
            visited_keys,
            replaced_keys,
            replaced_this_iter,
        ),
        "acc" => {
            count = count + operation.count;

            run_op_part2(
                &program,
                key + 1,
                count,
                visited_keys,
                replaced_keys,
                replaced_this_iter,
            )
        }
        "jmp" => run_op_part2(
            &program,
            key + operation.count,
            count,
            visited_keys,
            replaced_keys,
            replaced_this_iter,
        ),
        _ => None,
    }
}

fn build(input: &'static str) -> HashMap<i32, Operation> {
    let mut hash = HashMap::new();
    let mut i = 0;

    for line in input.split_terminator("\n") {
        let operator = line.get(..3).unwrap();
        let count = line.get(4..).unwrap().parse::<i32>().unwrap_or(0);

        hash.insert(
            i,
            Operation {
                op: operator,
                count: count,
            },
        );

        i = i + 1;
    }

    hash
}

fn main() {
    let input_data = input::get_input();

    println!("Total: {:?}", exec(input_data));
    println!("Total Part2: {:?}", exec_part2(input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            exec(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            ),
            5
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            exec_part2(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            ),
            8
        )
    }
}
