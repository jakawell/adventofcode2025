use utils::read_lines;

fn main() {
    let joltage_size = 12;
    let mut total_joltage = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            total_joltage += check_max_joltage(joltage_size, &line);
        }
    }
    println!("Answer: {total_joltage}");
}

pub fn check_max_joltage(joltage_size: usize, bank: &str) -> i64 {
    let bank = bank.as_bytes();
    let mut max_joltage = bank[0..joltage_size].to_vec();
    for battery_number in joltage_size..bank.len() {
        let battery = bank[battery_number];
        let mut options = vec![bytes_to_int(&max_joltage)];
        // try dropping the value at each index and putting the new "battery" at the end
        for index in 0..joltage_size {
            let mut option: Vec<u8> = Vec::new();
            option.extend_from_slice(&max_joltage[0..index]);
            if index < joltage_size - 1 {
                option.extend_from_slice(&max_joltage[index + 1..]);
            }
            option.push(battery);

            options.push(bytes_to_int(&option));
        }
        let new_max = options.iter().max().unwrap();
        let new_max = int_to_bytes(&new_max);
        max_joltage = new_max;
    }
    bytes_to_int(&max_joltage)
}

fn bytes_to_int(bytes: &Vec<u8>) -> i64 {
    str::from_utf8(&bytes).unwrap().parse().unwrap()
}

fn int_to_bytes(number: &i64) -> Vec<u8> {
    number.to_string().as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_987654321111111() {
        assert_eq!(check_max_joltage(2, "987654321111111"), 98);
    }

    #[test]
    fn validate_811111111111119() {
        assert_eq!(check_max_joltage(2, "811111111111119"), 89);
    }

    #[test]
    fn validate_234234234234278() {
        assert_eq!(check_max_joltage(2, "234234234234278"), 78);
    }

    #[test]
    fn validate_818181911112111() {
        assert_eq!(check_max_joltage(2, "818181911112111"), 92);
    }

    #[test]
    fn validate_987654321111111_joltage12() {
        assert_eq!(check_max_joltage(12, "987654321111111"), 987654321111);
    }

    #[test]
    fn validate_811111111111119_joltage12() {
        assert_eq!(check_max_joltage(12, "811111111111119"), 811111111119);
    }

    #[test]
    fn validate_234234234234278_joltage12() {
        assert_eq!(check_max_joltage(12, "234234234234278"), 434234234278);
    }

    #[test]
    fn validate_818181911112111_joltage12() {
        assert_eq!(check_max_joltage(12, "818181911112111"), 888911112111);
    }
}
