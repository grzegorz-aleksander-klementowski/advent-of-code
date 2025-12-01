#[derive(Debug, PartialEq, Eq)]
struct DialZeroCounter(u32);

#[derive(Debug, PartialEq, Eq, Clone)]
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
    use super::*;

    // Safe dial number can't be more than 99
    #[test]
    #[should_panic(expected = "Out of scope")]
    fn test_dial_num_scope() {
        // todo place fn later
        let sejf_num = SejfDialNum(100);
    }

    // Test if the rotation works
    #[test]
    fn test_rotation() {
        let diall_num = SejfDialNum(11);
        let rotation = Rotation {
            direction: Direction::Right,
            distance: SejfDialNum(8),
        };

        // todo place fn later
        let res = diall_num;
        assert_eq!(res, SejfDialNum(19))
    }

    #[test]
    fn test_rotation_circle_corssing() {
        let mut diall_num = SejfDialNum(5);
        let first_rotation = Rotation {
            direction: Direction::Left,
            distance: SejfDialNum(10),
        };

        // todo place fn later
        let res = diall_num.clone();
        assert_eq!(res, SejfDialNum(95));

        let second_rotation = Rotation {
            direction: Direction::Right,
            distance: SejfDialNum(5),
        };

        // todo place fn later
        let res = diall_num;
        assert_eq!(res, SejfDialNum(0));
    }

    #[test]
    fn test_read_file_and_parse() {
        let res = vec!["R0"];

        assert_eq!(
            res,
            vec![
                "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
            ]
        )
    }

    #[test]
    fn test_zeros_counter() {
        // todo add fn later
        let routes = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];

        let mut res = DialZeroCounter(0);
        assert_eq!(res, DialZeroCounter(3));
    }
}
