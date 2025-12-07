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

pub fn process_input_part1<T>(lines: &mut T) -> u32
where
    T: Iterator<Item = String>,
{
    let mut sum = 0u32;

    let first_line = lines.next().unwrap();
    let first_line = first_line.chars();
    let mut current_beam_state: Vec<bool> = Vec::new();
    for location_state in first_line {
        if location_state == 'S' {
            current_beam_state.push(true);
        } else {
            current_beam_state.push(false);
        }
    }

    for line in lines {
        let line = line.chars();
        for (index, location_state) in line.enumerate() {
            if location_state == '^' && current_beam_state[index] {
                sum += 1;
                current_beam_state[index] = false;
                if index > 0 {
                    current_beam_state[index - 1] = true;
                }
                if index < current_beam_state.len() - 1 {
                    current_beam_state[index + 1] = true;
                }
            }
        }
    }

    sum
}
