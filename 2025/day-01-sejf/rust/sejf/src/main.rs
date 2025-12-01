use std::fs;

trait Input {
    fn read_a_document(path: &str) -> Self;
}

struct CrackedCode(u32);

struct TheAttachedDocument(String);

impl TheAttachedDocument {
    fn crack_the_code(doc: String) -> CrackedCode {
        let mut num_of_zeros = SejfDialZeroCounter::default();

        for line in doc.lines() {
            let rote = Rotation::parse(line);
            let mut sejf_dial_num = SejfDialNum::default();

            sejf_dial_num.rotate_sejf_dial(&rote);

            num_of_zeros.count_zeros(&sejf_dial_num);
        }

        CrackedCode(num_of_zeros.0)
    }
}

impl Input for TheAttachedDocument {
    fn read_a_document(path: &str) -> Self {
        let file_content = fs::read_to_string(path).expect("Can't read file.");
        Self(file_content)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SejfDialNum(u32);

impl SejfDialNum {
    fn rotate_sejf_dial(&mut self, rote: &Rotation) {
        let sejf_dial_range = 100;
        let distance = rote.distance.0 % sejf_dial_range;

        match rote.direction {
            Direction::Right => self.0 = (self.0 + distance) % sejf_dial_range,
            Direction::Left => self.0 = (self.0 + sejf_dial_range - distance) % sejf_dial_range,
        }
    }
}

impl Default for SejfDialNum {
    fn default() -> Self {
        SejfDialNum(50)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq)]
struct Rotation {
    direction: Direction,
    distance: SejfDialNum,
}

impl Rotation {
    fn new(direction: Direction, distance: SejfDialNum) -> Self {
        Rotation {
            direction,
            distance,
        }
    }

    fn parse(doc_snippet: &str) -> Rotation {
        doc_snippet.to_string();
        let mut snippet_chars = doc_snippet.chars();

        let direction = match snippet_chars.next().expect("Here should be a char!") {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!(
                "A wrong letter from the doc snippet. Should be a char `R` or `L`. The snippet: {doc_snippet:?}"
            ),
        };

        let distance: u32 = snippet_chars
            .as_str()
            .parse()
            .expect("Here should be a number");

        Rotation::new(direction, SejfDialNum(distance))
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Rotation {
            direction: Direction::Right,
            distance: SejfDialNum(1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
struct SejfDialZeroCounter(u32);

impl SejfDialZeroCounter {
    fn count_zeros(&mut self, sejf_dial_num: &SejfDialNum) {
        if sejf_dial_num.0 == 0 {
            self.0 += 1;
        }
    }
}

fn main() {
    println!("Wesołych świąt!");
}

#[cfg(test)]
mod test {
    use super::*;

    // Test if the rotation works
    #[test]
    fn test_rotation() {
        let mut diall_num = SejfDialNum(11);
        let rotation = Rotation {
            direction: Direction::Right,
            distance: SejfDialNum(8),
        };

        diall_num.rotate_sejf_dial(&rotation);

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

        // rotation that cross zero from a „possitive” num side
        diall_num.rotate_sejf_dial(&first_rotation);

        let res = diall_num.clone();
        assert_eq!(res, SejfDialNum(95));

        // rotation that move to 0 from a „negative” num side
        let second_rotation = Rotation {
            direction: Direction::Right,
            distance: SejfDialNum(5),
        };

        diall_num.rotate_sejf_dial(&second_rotation);

        let res = diall_num;
        assert_eq!(res, SejfDialNum(0));
    }

    #[test]
    fn test_rotation_parse() {
        let doc_snip_zero = "R0";

        let res = Rotation::parse(doc_snip_zero);

        assert_eq!(
            res,
            Rotation {
                direction: Direction::Right,
                distance: SejfDialNum(0),
            }
        );

        let doc_snip_left = "L40";
        let res = Rotation::parse(doc_snip_left);

        assert_eq!(
            res,
            Rotation {
                direction: Direction::Left,
                distance: SejfDialNum(40),
            }
        );
    }

    #[test]
    fn test_zeros_counter() {
        // todo add fn later

        let mut res = SejfDialZeroCounter::default();
        res.count_zeros(&SejfDialNum(40));
        res.count_zeros(&SejfDialNum(25));
        res.count_zeros(&SejfDialNum(0));
        res.count_zeros(&SejfDialNum(0));
        res.count_zeros(&SejfDialNum(65));
        res.count_zeros(&SejfDialNum(0));

        assert_eq!(res, SejfDialZeroCounter(3));
    }

    #[test]
    fn test() {
        let routes = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
    }
}
