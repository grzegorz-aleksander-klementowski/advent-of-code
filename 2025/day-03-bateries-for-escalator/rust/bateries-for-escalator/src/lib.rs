/* trait Input {
    fn load_batery_info(path: &str) -> Vec<BateryBank>;
}

trait Validate {
    fn validate_label(self) -> Result<Batery, BatteryErrors>;
}

#[derive(Debug, PartialEq, Eq)]
enum BatteryErrors {
    IncorrectLabel,
}

struct BateryBank(Vec<Batery>);

impl Input for BateryBank {
    fn load_batery_info(path: &str) -> Vec<BateryBank> {
        let load_to_str: Vec<&str> = std::fs::read_to_string(path)
            .expect("Can't read the file!")
            .lines()
            .collect();

        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Batery {
    joltage: usize,
}

impl Batery {
    fn new(digit: usize) {}
}

impl Validate for Batery {
    fn validate_label(self) -> Result<Batery, BatteryErrors> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate_joltage_label() {
        let baterry = Batery { joltage: 11 };
        let res = baterry.validate_label();
        assert_eq!(res, Err(BatteryErrors::IncorrectLabel));
    }
}
 */
