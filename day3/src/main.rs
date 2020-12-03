mod input;

fn count_trees(input: &'static str, right: usize, down: usize) -> u32 {
    let mut tree_count = 0;
    let mut right_offset = right;
    let mut down_offset = down;
    let line_length = input.lines().next().unwrap().chars().count();
    let line_count = input.lines().count();

    while down_offset < line_count {
        let char = input
            .lines()
            .skip(down_offset)
            .next()
            .unwrap()
            .chars()
            .skip(right_offset)
            .next()
            .unwrap();

        if char == '#' {
            tree_count = tree_count + 1;
        }

        if right_offset + right >= line_length {
            right_offset = right_offset + right - line_length;
        } else {
            right_offset = right + right_offset;
        }

        down_offset = down + down_offset;
    }

    tree_count
}

fn main() {
    let input_data = input::get_input();

    let r1d1 = count_trees(input_data, 1, 1);
    let r3d1 = count_trees(input_data, 3, 1);
    let r5d1 = count_trees(input_data, 5, 1);
    let r7d1 = count_trees(input_data, 7, 1);
    let r1d2 = count_trees(input_data, 1, 2);

    println!("Number of Trees with right 1, down 1: {:?}", r1d1);
    println!("Number of Trees with right 3, down 1: {:?}", r3d1);
    println!("Number of Trees with right 5, down 1: {:?}", r5d1);
    println!("Number of Trees with right 7, down 1: {:?}", r7d1);
    println!("Number of Trees with right 1, down 2: {:?}", r1d2);

    println!("Total Count: {:?}", r1d1 * r3d1 * r5d1 * r7d1 * r1d2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
    }

    #[test]
    fn test() {
        assert_eq!(count_trees(test_input(), 3, 1), 7)
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            count_trees(test_input(), 1, 1)
                * count_trees(test_input(), 3, 1)
                * count_trees(test_input(), 5, 1)
                * count_trees(test_input(), 7, 1)
                * count_trees(test_input(), 1, 2),
            336
        )
    }
}
