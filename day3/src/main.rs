use std::fs;

fn main() {
    // Read the input from a file
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    // Initialize the state
    let mut enabled = true;
    let mut sum = 0;

    // Scan through the input
    let mut i = 0;
    while i < input.len() {
        if input[i..].starts_with("mul(") {
            if let Some((x, y, len)) = parse_mul(&input[i..]) {
                if enabled {
                    sum += x * y;
                }
                i += len;
            } else {
                i += 1;
            }
        } else if input[i..].starts_with("do()") {
            enabled = true;
            i += 4;
        } else if input[i..].starts_with("don't()") {
            enabled = false;
            i += 7;
        } else {
            i += 1;
        }
    }

    println!(
        "The sum of the results of the enabled multiplications is: {}",
        sum
    );
}

fn parse_mul(input: &str) -> Option<(i32, i32, usize)> {
    let mut chars = input.chars();
    chars.next(); // skip 'm'
    chars.next(); // skip 'u'
    chars.next(); // skip 'l'
    chars.next(); // skip '('

    let mut x = String::new();
    while let Some(c) = chars.next() {
        if c == ',' {
            break;
        }
        if c.is_digit(10) {
            x.push(c);
        } else {
            return None;
        }
    }

    let mut y = String::new();
    while let Some(c) = chars.next() {
        if c == ')' {
            break;
        }
        if c.is_digit(10) {
            y.push(c);
        } else {
            return None;
        }
    }

    let x: i32 = x.parse().ok()?;
    let y: i32 = y.parse().ok()?;
    let len = input.len() - chars.as_str().len();

    Some((x, y, len))
}
