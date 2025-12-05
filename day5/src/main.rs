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

enum Mode {
    ParseRange,
    ParseItems,
}

pub fn process_input_part1<T>(lines: &mut T) -> i32
where
    T: Iterator<Item = String>,
{
    let mut sum = 0;
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    let mut mode = Mode::ParseRange;
    for line in lines {
        if line.len() == 0 {
            mode = Mode::ParseItems;
            continue;
        }

        match mode {
            Mode::ParseRange => {
                let range: Vec<&str> = line.split('-').collect();
                ranges.push((range[0].parse().unwrap(), range[1].parse().unwrap()));
            }
            Mode::ParseItems => {
                let item: i64 = line.parse().unwrap();

                for range in &ranges {
                    if item >= range.0 && item <= range.1 {
                        println!("Item {item} is fresh!");
                        sum += 1;
                        break;
                    }
                }
            }
        }
    }

    sum
}
