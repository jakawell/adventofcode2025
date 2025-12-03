use utils::read_lines;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        if let Some(input) = lines.map_while(Result::ok).next() {
            let ranges = input.split(',');
            let mut sum = 0;
            for range in ranges {
                let split_range: Vec<&str> = range.split('-').collect();
                sum += iterate_range(&split_range[0], &split_range[1]);
            }
            println!("Answer: {}", sum);
        }
    }
}

fn iterate_range(start: &str, end: &str) -> i64 {
    let start: i64 = start.parse().unwrap();
    let end = end.parse::<i64>().unwrap() + 1;
    let mut sum = 0i64;
    for value in start..end {
        if !check_id_pattern(&value.to_string()) {
            println!("{value}");
            sum += value as i64;
        }
    }
    sum
}

pub fn check_id_halfsies(id: &str) -> bool {
    // odd lengths are automatically valid since they can't be split in two
    if id.len() % 2 != 0 {
        return true;
    }

    let half_length = id.len() / 2;
    let half1 = &id[0..half_length];
    let half2 = &id[half_length..];

    half1 != half2
}

pub fn check_id_pattern(id: &str) -> bool {
    // we need at least one byte to start pattern checking
    if id.len() == 0 {
        return true;
    }
    // create a pattern String that only contains the first byte
    // check if the next byte matches it
    // if it does, we're still okay
    // if it doesn't, add the new byte to the pattern
    // if there are no bytes remaining, we're done
    // if there are n bytes (where n is the length of the pattern) remaining, go back to the start, checking if the next bytes match the pattern
    // if there are less than n bytes, we're done and there is no repeating pattern
    let bytes = id.as_bytes();
    let mut pattern = &bytes[0..1];
    let mut remaining_bytes = &bytes[1..];
    let mut index = 1;
    loop {
        let pattern_length = pattern.len();
        // check if there are enough bytes left to match the pattern
        if remaining_bytes.len() >= pattern_length {
            let to_check = &remaining_bytes[0..pattern_length];
            // if the next bytes DO NOT match the pattern, we update the pattern to be all the processed bytes so far plus this new one
            if to_check != pattern {
                index += 1;
                let processed_bytes = &bytes[0..index];
                remaining_bytes = &bytes[index..];
                pattern = processed_bytes;
            }
            // if the next bytes DO match the pattern, we move those bytes to the processed list and advance past them
            else {
                index += pattern_length;
                remaining_bytes = &bytes[index..];
            }
        }
        // there are no longer enough bytes to match the pattern, so this is okay
        else {
            return true;
        }

        // we have reached the end
        if index >= bytes.len() {
            // if the pattern is just the entire string, it does not repeat and it's okay
            return pattern.len() == bytes.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_121() {
        assert!(check_id_halfsies("121") == true);
    }

    #[test]
    fn valid_1221() {
        assert!(check_id_halfsies("1221") == true);
    }

    #[test]
    fn invalid_55() {
        assert!(check_id_halfsies("55") == false);
    }

    #[test]
    fn invalid_6464() {
        assert!(check_id_halfsies("6464") == false);
    }

    #[test]
    fn invalid_123123() {
        assert!(check_id_halfsies("123123") == false);
    }

    #[test]
    fn pattern_valid_121() {
        assert!(check_id_pattern("121") == true);
    }

    #[test]
    fn pattern_valid_1221() {
        assert!(check_id_pattern("1221") == true);
    }

    #[test]
    fn pattern_invalid_55() {
        assert!(check_id_pattern("55") == false);
    }

    #[test]
    fn pattern_invalid_6464() {
        assert!(check_id_pattern("6464") == false);
    }

    #[test]
    fn pattern_invalid_123123() {
        assert!(check_id_pattern("123123") == false);
    }
}
