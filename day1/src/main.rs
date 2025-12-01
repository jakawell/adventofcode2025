use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut arrow = 50;
    let mut zeroes = 0;
    if let Ok(lines) = read_lines("./day1/input.txt") {
        for line in lines.map_while(Result::ok) {
            arrow = dbg!(advance(arrow, &line));
            if arrow == 0 {
                zeroes += 1;
            }
        }
    }
    println!("Answer: {zeroes}");
}

fn advance(start: i32, code: &str) -> i32 {
    let mut chars = code.chars();
    let direction = chars.next().unwrap();
    let amount: i32 = chars.as_str().parse().unwrap();
    let raw_next = if direction == 'L' {
        (start - amount).rem_euclid(100)
    } else {
        (start + amount).rem_euclid(100)
    };

    if raw_next >= 0 {
        raw_next
    } else {
        100 - raw_next
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
