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

pub fn process_input_part2<T>(lines: &mut T) -> i64
where
    T: Iterator<Item = String>,
{
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for line in lines {
        // we skip the item list now
        if line.len() == 0 {
            break;
        }

        let range: Vec<&str> = line.split('-').collect();
        ranges.push((range[0].parse().unwrap(), range[1].parse().unwrap()));
    }

    // sort by first number (yes, we're gonna be iterating like 3 or 4 times for this solution, but I can't be bothered sorry not sorry lol)
    ranges.sort_by_key(|r| r.0); // but look at it, it's such a widdle wine of code 🥰 it couldn't possibly be a big O problem, right??! 🥹

    let mut sum = 0i64;
    // these starting values ensure that the first range processed will add 0 to the sum
    let mut current_min: i64 = -1;
    let mut current_max: i64 = -2;
    for range in ranges {
        if range.0 > current_max {
            sum += current_max - current_min + 1; // add one because the ranges are inclusive
            current_min = range.0;
            current_max = range.1;
        } else if range.1 > current_max {
            current_max = range.1;
        }
    }
    sum += current_max - current_min + 1; // sum the final range

    sum
}
