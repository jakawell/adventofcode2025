use utils::read_lines;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut lines = lines.map_while(Result::ok);
        let sum = process_input_part1(&mut lines);
        println!("Answer: {sum}");
    } else {
        println!("Maybe missing the file.");
    }
}

fn process_input_part1<T>(lines: &mut T) -> i64
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
