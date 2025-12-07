use utils::read_lines;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut lines = lines.map_while(Result::ok);
        let sum = process_input_part2(&mut lines);
        println!("Answer: {sum}");
    } else {
        println!("Maybe missing the file.");
    }
}

pub fn process_input_part1<T>(lines: &mut T) -> i64
where
    T: Iterator<Item = String>,
{
    let mut sum = 0;

    let (rows, operators) = load_data(lines);

    for (index, operator) in operators.iter().enumerate() {
        let mut total = if *operator == "+" { 0i64 } else { 1i64 };
        for row in rows.iter() {
            // print!("{operator} {} ", row.get(index).unwrap());
            if *operator == "+" {
                total += row.get(index).unwrap();
            } else {
                total *= row.get(index).unwrap();
            }
        }
        println!("= {total}");
        sum += total;
    }

    sum
}

#[derive(Debug)]
struct ColumnInfo {
    operator: char,
    size: usize,
    start_index: usize,
    value_set: Vec<String>,
}

pub fn process_input_part2<T>(lines: &mut T) -> u64
where
    T: Iterator<Item = String>,
{
    let mut sum = 0;

    // this will store all the number records for later processing, which yes is putting the whole input in memory, but the whole file is 19kb so bite me
    let mut data_rows: Vec<String> = Vec::new();
    let mut columns: Vec<ColumnInfo> = Vec::new();

    // first we find the last row to get information about the column sizes
    for line in lines {
        let mut line_chars = line.chars().enumerate();
        let mut first_char = line_chars.next().unwrap().1;
        let mut column_start_index = 0;
        if first_char == '+' || first_char == '*' {
            let mut column_size: usize = 0; // we start at zero because there will always be an extra whitespace character separating the columns
            for (index, element) in line_chars {
                if element.is_whitespace() {
                    column_size += 1;
                } else {
                    columns.push(ColumnInfo {
                        operator: first_char,
                        size: column_size,
                        start_index: column_start_index,
                        value_set: vec![String::new(); column_size],
                    });
                    first_char = element;
                    column_size = 0;
                    column_start_index = index;
                }
            }
            // add the last column
            columns.push(ColumnInfo {
                operator: first_char,
                size: column_size + 1, // add one to correct for logic assuming an extra whitespace after each column
                start_index: column_start_index,
                value_set: vec![String::new(); column_size + 1],
            });
        } else {
            data_rows.push(line.clone());
        }
    }

    // next we iterate over each row to get the value sets to operate on
    for row in data_rows.iter() {
        let mut current_column_index: usize = 0;
        let mut current_column = &mut columns[current_column_index];
        for (index, element) in row.chars().enumerate() {
            if index > current_column.start_index + current_column.size {
                current_column_index += 1;
                current_column = &mut columns[current_column_index];
            }

            if element.is_numeric() {
                current_column.value_set[index - current_column.start_index].push(element);
            }
        }
    }

    // finally we complete the operations
    for column in columns {
        let mut total = if column.operator == '+' { 0 } else { 1 };
        for value in column.value_set {
            let value: u64 = value.parse().unwrap();
            if column.operator == '+' {
                total += value;
            } else {
                total *= value;
            }
        }
        sum += total;
    }

    sum
}

fn load_data<T>(lines: &mut T) -> (Vec<Vec<i64>>, Vec<String>)
where
    T: Iterator<Item = String>,
{
    let mut rows: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<String> = Vec::new();
    for line in lines {
        let elements: Vec<String> = line.split_whitespace().map(String::from).collect();
        if elements.get(0).unwrap().parse::<i64>().is_ok() {
            rows.push(elements.iter().map(|e| e.parse().unwrap()).collect());
        } else {
            operators = elements;
        }
    }

    (rows, operators)
}
