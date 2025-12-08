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

pub fn process_input_part2<T>(lines: &mut T) -> usize
where
    T: Iterator<Item = String>,
{
    let first_line = lines.next().unwrap();
    let first_line = first_line.chars();
    let mut current_beam_state: u32 = 0;
    for (i, location_state) in first_line.enumerate() {
        if location_state == 'S' {
            current_beam_state |= 1 << i;
        }
    }

    let mut current_timelines = vec![current_beam_state];
    let mut new_timelines: Vec<u32> = Vec::new();
    for (row_index, line) in lines.enumerate() {
        // every other row seems to be all blank
        // if row_index % 2 == 0 {
        //     continue;
        // }
        println!(
            "Starting row {row_index} ({} timelines)",
            current_timelines.len()
        );
        let line = splitter_row_to_bits(&line);
        for timeline in &current_timelines {
            let splits = line & timeline;
            if splits > 0 {
                // split the timelines
            } else {
                new_timelines.push(*timeline);
            }
            // let line = line.chars();
            // let mut did_split = false;
            // for (index, location_state) in line.enumerate() {
            //     if location_state == '^' && timeline[index] {
            //         did_split = true;
            //         if index > 0 {
            //             let mut new_timeline = timeline.clone();
            //             new_timeline[index] = false;
            //             new_timeline[index - 1] = true;
            //             new_timelines.push(new_timeline);
            //         }
            //         if index < timeline.len() - 1 {
            //             let mut new_timeline = timeline.clone();
            //             new_timeline[index] = false;
            //             new_timeline[index + 1] = true;
            //             new_timelines.push(new_timeline);
            //         }
            //     }
            // }
            // if this iteration didn't cause this timeline to split, we just want to copy it to the next iteration's timelines
            // if !did_split {
            //     new_timelines.push(timeline.clone());
            // }
        }

        // prep for the next iteration by moving the "next" timelines to the "current" timelines and resetting "next"
        current_timelines = new_timelines;
        new_timelines = Vec::new();
    }

    current_timelines.len()
}

fn splitter_row_to_bits(row: &String) -> u32 {
    let mut result = 0;
    for (i, location_state) in row.chars().enumerate() {
        if location_state == 'S' {
            result |= 1 << i;
        }
    }

    result
}
