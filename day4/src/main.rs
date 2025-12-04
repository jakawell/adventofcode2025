use utils::read_lines;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut lines = lines.map_while(Result::ok);
        println!("Answer: {}", process_input(&mut lines));
    } else {
        println!("Maybe missing the file.");
    }
}

pub fn process_input<T>(lines: &mut T) -> i32
where
    T: Iterator<Item = String>,
{
    let mut total = 0;

    let first_line = &lines.next().unwrap();
    let space_length = first_line.len();

    // create a fictional empty row above the first row
    let mut row_above = '.'.to_string().repeat(space_length);
    let mut row = first_line.to_string();
    let mut row_below: String;

    for next_row in lines {
        row_below = next_row;
        total += check_row(row_above.as_bytes(), row.as_bytes(), row_below.as_bytes());

        // move up
        row_above = row;
        row = row_below;
    }

    // check last row with another fictional empty row below it
    row_below = '.'.to_string().repeat(space_length);
    total += check_row(row_above.as_bytes(), row.as_bytes(), row_below.as_bytes());

    total
}

pub fn check_row(row_above: &[u8], row: &[u8], row_below: &[u8]) -> i32 {
    let mut debug_printable: String = String::from("");

    let mut sum = 0;
    for (index, location) in row.iter().enumerate() {
        if *location != b'@' {
            debug_printable.push(*location as char);
            continue;
        }

        let mut local_sum = 0;
        if index > 0 {
            local_sum += count_location(&row_above[index - 1]);
            local_sum += count_location(&row[index - 1]);
            local_sum += count_location(&row_below[index - 1]);
        }
        local_sum += count_location(&row_above[index]);
        local_sum += count_location(&row_below[index]);
        if index < row.len() - 1 {
            local_sum += count_location(&row_above[index + 1]);
            local_sum += count_location(&row[index + 1]);
            local_sum += count_location(&row_below[index + 1]);
        }

        if local_sum < 4 {
            sum += 1;
            debug_printable.push('x');
        } else {
            debug_printable.push(*location as char);
        }
    }

    println!("{debug_printable}");

    sum
}

fn count_location(location: &u8) -> i32 {
    let should_count = *location == b'@';
    if should_count { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_line() {
        let row_above = "..@@.@@@@.".as_bytes();
        let row = "@@@.@.@.@@".as_bytes();
        let row_below = "@@@@@.@.@@".as_bytes();

        assert_eq!(check_row(row_above, row, row_below), 1);
    }
}
