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

pub fn process_input_part2<T>(lines: &mut T) -> i64
where
    T: Iterator<Item = String>,
{
    let mut sum = 0;

    let (rows, operators) = load_data(lines);

    // a single "value set" is a set of numbers that a single operator will be applied to
    let mut value_sets: Vec<Vec<String>> = vec![Vec::new(); operators.len()];
    // we iterate over each row, updating the value set for each column
    for row in rows {
        for (value_set_index, number) in row.iter().enumerate() {
            // we reverse the number and append each numeral to the associated value in the value set (the value at the numeral's index)
            let reversed_number: String = number
                .to_string()
                .chars()
                .rev()
                .map(|i| i.to_string())
                .collect::<String>();
            dbg!(&reversed_number);
            let value_set = &mut value_sets[value_set_index];

            for (col_index, numeral) in reversed_number.chars().enumerate() {
                if let Some(value) = value_set.get(col_index) {
                    let mut value = value.clone();
                    value.push(numeral);
                    value_set[col_index] = value;
                } else {
                    value_set.push(numeral.to_string());
                }
            }
        }
    }

    for (index, operator) in operators.iter().enumerate() {
        let mut total = if *operator == "+" { 0i64 } else { 1i64 };
        let value_set = &value_sets[index];
        for value in value_set {
            print!("{operator} {} ", value);
            if *operator == "+" {
                total += value.parse::<i64>().unwrap();
            } else {
                total *= value.parse::<i64>().unwrap();
            }
        }
        println!("= {total}");
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
