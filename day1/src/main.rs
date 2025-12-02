use utils::read_lines;

fn main() {
    let mut arrow = 50;
    let mut zeroes = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            let (new_arrow, zero_crosses) = advance(arrow, &line);
            arrow = new_arrow;
            zeroes += zero_crosses;
        }
    }
    println!("Answer: {zeroes}");
}

fn advance(start: i32, code: &str) -> (i32, i32) {
    println!("Input:");
    println!("  from {start}");
    println!("  move {code}");

    let mut chars = code.chars();
    let direction = chars.next().unwrap();
    let amount: i32 = chars.as_str().parse().unwrap();
    let raw_next = if direction == 'L' {
        start - amount
    } else {
        start + amount
    };

    let mut zero_crosses = 0;

    // If the result is negative, we crossed 0 at least once to get there (as long as we didn't start on 0)
    if start != 0 && raw_next < 0 {
        zero_crosses += 1;
    }

    // Integer division tells us how many complete cycles we made
    zero_crosses += (raw_next / 100).abs();

    // Mathmatical modulus operation gives us the actual value it ends on
    let modulo = raw_next.rem_euclid(100);

    // If we turn right directly to zero without making a complete cycle, that still counts as using zero
    if raw_next == 0 {
        zero_crosses += 1;
    }

    println!("Raw move: {raw_next}");
    println!("Output:");
    println!("  needle now at {modulo}");
    println!("  with {zero_crosses} crosses over zero");
    println!("");

    (modulo, zero_crosses)
}
