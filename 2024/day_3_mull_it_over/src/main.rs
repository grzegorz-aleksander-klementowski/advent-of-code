use std::fs;

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("./zapiski/pomieszana_pamięć")?;
    let input = content.trim();

    let mut idx = 0;
    let mut sum = 0usize;
    let mut enabled = true;
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();

    while idx < len {
        if let Some(new_idx) = parse_do(&chars, idx) {
            enabled = true;
            idx = new_idx;
        } else if let Some(new_idx) = parse_dont(&chars, idx) {
            enabled = false;
            idx = new_idx;
        } else if let Some((x, y, new_idx)) = parse_mul(&chars, idx) {
            if enabled {
                sum += x * y;
            }
            idx = new_idx;
        } else {
            idx += 1;
        }
    }

    println!("{}", sum);
    Ok(())
}

fn parse_do(chars: &[char], idx: usize) -> Option<usize> {
    if chars.len() >= idx + 4 && &chars[idx..idx + 4] == ['d', 'o', '(', ')'] {
        Some(idx + 4)
    } else {
        None
    }
}

fn parse_dont(chars: &[char], idx: usize) -> Option<usize> {
    if chars.len() >= idx + 7 && &chars[idx..idx + 7] == ['d', 'o', 'n', '\'', 't', '(', ')'] {
        Some(idx + 7)
    } else {
        None
    }
}

fn parse_mul(chars: &[char], idx: usize) -> Option<(usize, usize, usize)> {
    if chars.len() < idx + 4 || &chars[idx..idx + 4] != ['m', 'u', 'l', '('] {
        return None;
    }
    let mut i = idx + 4;

    // Parse X: 1-3 digits
    let start_x = i;
    while i < chars.len() && chars[i].is_ascii_digit() && (i - start_x) < 3 {
        i += 1;
    }
    if start_x == i || (i - start_x) > 3 {
        return None; // No digits found or more than 3 digits
    }
    let x_str: String = chars[start_x..i].iter().collect();
    let x = x_str.parse::<usize>().ok()?;

    // Next character must be ','
    if i >= chars.len() || chars[i] != ',' {
        return None;
    }
    i += 1;

    // Parse Y: 1-3 digits
    let start_y = i;
    while i < chars.len() && chars[i].is_ascii_digit() && (i - start_y) < 3 {
        i += 1;
    }
    if start_y == i || (i - start_y) > 3 {
        return None; // No digits found or more than 3 digits
    }
    let y_str: String = chars[start_y..i].iter().collect();
    let y = y_str.parse::<usize>().ok()?;

    // Next character must be ')'
    if i >= chars.len() || chars[i] != ')' {
        return None;
    }
    i += 1;

    Some((x, y, i))
}
