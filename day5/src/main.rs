mod input;

#[derive(Debug)]
struct Ticket {
    id: usize,
    row: usize,
    column: usize,
}

#[derive(Debug)]
struct Row {
    column: Vec<Ticket>,
}

fn calc_id(row: usize, column:usize) -> usize {
    row * 8 + column
}

fn get_highest(input: &Vec<&'static str>) -> usize {
    let mut highest = 0;

    for line in input {
        let id = get_id(line);
        if id > highest {
            highest = id;
        }
    }

    highest
}

fn get_id(input: &'static str) -> usize {
    let row_string = &input[..input.len() - 3].replace("B", "1").replace("F", "0");
    let column_string = &input[input.len() - 3..].replace("R", "1").replace("L", "0");

    let row = usize::from_str_radix(row_string, 2).unwrap();
    let column = usize::from_str_radix(column_string, 2).unwrap();

    calc_id(row, column)
}

fn get_ticket(input: &'static str) -> Ticket {
    let row_string = &input[..input.len() - 3].replace("B", "1").replace("F", "0");
    let column_string = &input[input.len() - 3..].replace("R", "1").replace("L", "0");

    let row = usize::from_str_radix(row_string, 2).unwrap();
    let column = usize::from_str_radix(column_string, 2).unwrap();

    Ticket {
        id: calc_id(row, column),
        row,
        column,
    }
}

fn get_missing(input: &Vec<&'static str>) -> Option<Ticket> {
    let mut missing: Option<Ticket> = None;
    let mut rows: Vec<Row> = vec![];
    for _ in 0..126 {
        rows.push(Row { column: vec![] })
    }

    for line in input {
        let ticket = get_ticket(line);
        rows[ticket.row].column.push(ticket)
    }

    for line in 0..126 {
        if line == 0 || line == 127 {
            continue;
        }
        if rows[line].column.len() == 7 {
            for column in 0..7 {
                match rows[line].column.iter().find(|&c| c.column == column) {
                    Some(_) => {}
                    None => {
                        missing = Some(Ticket {
                            id: calc_id(line, column),
                            row: line,
                            column,
                        })
                    }
                }
            }
        }
    }

    missing
}

fn main() {
    let input_data = input::get_input();

    let highest = get_highest(&input_data);

    println!("Highest: {:?}", highest);

    let missing = get_missing(&input_data);

    println!("My Ticket: {:?}", missing.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(get_id("BFFFBBFRRR"), 567);
        assert_eq!(get_id("FFFBBBFRRR"), 119);
        assert_eq!(get_id("BBFFBBFRLL"), 820);
    }
}
