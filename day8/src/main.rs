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
}
