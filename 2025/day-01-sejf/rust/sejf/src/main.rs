struct DialZeroCounter(u32);

struct SejfDialNum(u32);

enum Direction {
    Right,
    Left,
}

struct Rotation {
    direction: Direction,
    distance: SejfDialNum,
}

fn main() {
    println!("Wesołych świąt!");
}

#[cfg(test)]
mod test {
    use crate::SejfDialNum;

    #[test]
    #[should_panic(expected = "Out of scope")]
    fn test_dial_num_scope() {
        let sejf_num = SejfDialNum(100);
    }
}
