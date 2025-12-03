trait Input {
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

#[derive(Debug, PartialEq, Eq)]
struct Batery {
    joltage: usize,
}

impl Input for Batery {
    fn load_batery_info(path: &str) -> Vec<BateryBank> {
        let load_to_str = std::fs::read_to_string(path);

        todo!()
    }
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

