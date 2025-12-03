use std::{ops::RangeInclusive, usize};

trait Input: Sized {
    fn load_into_vec(path_to_file: &str) -> Vec<Self>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct IdRange {
    first_id: ProductId,
    last_id: ProductId,
}

impl Input for IdRange {
    fn load_into_vec(path: &str) -> Vec<Self> {
        let file_cont = std::fs::read_to_string(path).expect("Cannot open the file! ");

        let mut range_vec = Vec::new();
        for range_snip in file_cont.split(',') {
            let range = Self::parse(range_snip);
            range_vec.push(range);
        }
        range_vec
    }
}

impl IdRange {
    // constructor for easier building
    fn new(first_id: ProductId, last_id: ProductId) -> Self {
        Self { first_id, last_id }
    }

    // parse a snippet text to a struct of Range
    fn parse(text: &str) -> Self {
        let ids_vec: Vec<&str> = text.split('-').collect();
        let first_id: usize = ids_vec[0]
            .trim()
            .parse()
            .expect("Can't parse text into a number!");
        let last_id: usize = ids_vec[1]
            .trim()
            .parse()
            .expect("Can't parse text into a number!");

        IdRange::new(ProductId(first_id), ProductId(last_id))
    }

    // transport a range into a usefull std::ops::Range type
    fn transform_to_ops_range(self) -> RangeInclusive<usize> {
        let start = self.first_id;
        let end = self.last_id;
        start.0..=end.0
    }
}

impl Default for IdRange {
    fn default() -> Self {
        Self {
            first_id: ProductId(2),
            last_id: ProductId(999),
        }
    }
}

// Type of product Id
#[derive(Debug, PartialEq, Eq, Clone)]
struct ProductId(usize);
impl ProductId {
    fn identify_id_validity(self) -> IdValidity {
        let id_as_str = self.0.to_string().clone();
        let id_len = id_as_str.len();
        let half = &id_as_str.len() / 2;

        if (id_len % 2) == 0 {
            let fist_half = id_as_str[..half].to_string();
            let second_half = id_as_str[half..].to_string();
            if fist_half == second_half {
                IdValidity::Invalid(self)
            } else {
                IdValidity::Valid(self)
            }
        } else {
            IdValidity::Valid(self)
        }
    }
}

// Enumeration to identify a valid or an invalid id
#[derive(Debug, PartialEq, Eq)]
enum IdValidity {
    Valid(ProductId),
    Invalid(ProductId),
}

impl IdValidity {
    // Adding up the invalid ids (THE GOAL OF 1ST PART OF THE TASK)
    fn add_up_invalid_ids(&self, value: usize) -> usize {
        match self {
            IdValidity::Valid(_) => value,
            IdValidity::Invalid(id) => value + id.0,
        }
    }

    fn add_up_invalid_ids_from_file(path: &str) -> usize {
        let loaded_file_into_vec = IdRange::load_into_vec(path);

        let mut added_inv_ids: usize = 0;

        for range in loaded_file_into_vec {
            let ids_range_as_num = range.transform_to_ops_range();

            for id_num in ids_range_as_num {
                let id = ProductId(id_num);
                match id.identify_id_validity() {
                    IdValidity::Valid(_) => continue,
                    IdValidity::Invalid(id) => {
                        added_inv_ids = IdValidity::Invalid(id).add_up_invalid_ids(added_inv_ids);
                    }
                };
            }
        }

        added_inv_ids
    }
}

fn main() {
    let added_inv_ids = IdValidity::add_up_invalid_ids_from_file("./input");
    println!("Adding up all the invalid IDs produces: {added_inv_ids}");
}

// --- tests --- \\

#[cfg(test)]
mod test {
    use super::*;

    //test if the program load the file into the structues
    #[test]
    fn test_load_file_and_parse() {
        let path = "test";

        let res: Vec<IdRange> = IdRange::load_into_vec(path);
        assert_eq!(
            res,
            vec![
                IdRange {
                    first_id: ProductId(11),
                    last_id: ProductId(22),
                },
                IdRange {
                    first_id: ProductId(95),
                    last_id: ProductId(115),
                },
                IdRange {
                    first_id: ProductId(998),
                    last_id: ProductId(1012),
                },
                IdRange {
                    first_id: ProductId(1188511880),
                    last_id: ProductId(1188511890),
                },
                IdRange {
                    first_id: ProductId(222220),
                    last_id: ProductId(222224),
                },
                IdRange {
                    first_id: ProductId(1698522),
                    last_id: ProductId(1698528),
                },
                IdRange {
                    first_id: ProductId(446443),
                    last_id: ProductId(446449),
                },
                IdRange {
                    first_id: ProductId(38593856),
                    last_id: ProductId(38593862),
                },
                // ----
                IdRange {
                    first_id: ProductId(565653),
                    last_id: ProductId(565659),
                },
                IdRange {
                    first_id: ProductId(824824821),
                    last_id: ProductId(824824827),
                },
                IdRange {
                    first_id: ProductId(2121212118),
                    last_id: ProductId(2121212124),
                },
            ]
        )
    }

    // test parsing of text to range struct
    #[test]
    fn test_parsing() {
        let example = "11-22";
        let res = IdRange::parse(example);

        assert_eq!(
            res,
            IdRange {
                first_id: ProductId(11),
                last_id: ProductId(22),
            }
        )
    }

    #[test]
    fn test_id_validity() {
        let inv_ids = [ProductId(55), ProductId(7474), ProductId(123123)];
        let valid_id = ProductId(101);

        // test if detect an invalid ids
        for id in inv_ids {
            let res = id.clone().identify_id_validity(); // fn'll place here
            assert_eq!(res, IdValidity::Invalid(id.clone()));
        }

        // test if detect a valid id
        let res = valid_id.clone().identify_id_validity(); // fn'll place here
        assert_eq!(res, IdValidity::Valid(valid_id));
    }

    #[test]
    // test finding invalid ids forom a range
    fn test_invalid_ids_range() {
        let id_ranges = [
            IdRange {
                first_id: ProductId(11),
                last_id: ProductId(22),
            },
            IdRange {
                first_id: ProductId(99),
                last_id: ProductId(115),
            },
            IdRange {
                first_id: ProductId(998),
                last_id: ProductId(1012),
            },
            IdRange {
                first_id: ProductId(1188511880),
                last_id: ProductId(1188511890),
            },
            IdRange {
                first_id: ProductId(222220),
                last_id: ProductId(222224),
            },
            IdRange {
                first_id: ProductId(1698522),
                last_id: ProductId(1698528),
            },
            IdRange {
                first_id: ProductId(446443),
                last_id: ProductId(446449),
            },
            IdRange {
                first_id: ProductId(38593856),
                last_id: ProductId(38593862),
            },
        ];

        // checking exceptions – if 2
        let range = id_ranges[0].clone().transform_to_ops_range();
        let mut res = 0;
        for id in range {
            let id = ProductId(id);
            match id.identify_id_validity() {
                IdValidity::Valid(_) => continue,
                IdValidity::Invalid(_) => res += 1,
            }
        }
        assert_eq!(res, 2);

        // checking usual in the case
        let range = id_ranges[1].clone().transform_to_ops_range();
        let mut res = 0;
        for id in range {
            let id = ProductId(id);
            match id.identify_id_validity() {
                IdValidity::Valid(_) => continue,
                IdValidity::Invalid(_) => res += 1,
            }
        }
        assert_eq!(res, 1);

        // checking exceptions – if 0
        let range = id_ranges[5].clone().transform_to_ops_range();
        let mut res = 0;
        for id in range {
            let id = ProductId(id);
            match id.identify_id_validity() {
                IdValidity::Valid(_) => continue,
                IdValidity::Invalid(_) => res += 1,
            }
        }
        assert_eq!(res, 0);

        let mut count: usize = 0;
        for range in &id_ranges {
            let range = range.clone().transform_to_ops_range();
            for id in range {
                let id = ProductId(id);
                match id.identify_id_validity() {
                    IdValidity::Valid(_) => continue,
                    IdValidity::Invalid(_) => count += 1,
                }
            }
        }

        assert_eq!(count, 8);
    }

    #[test]
    fn test_adding_up_invalids_ids() {
        let inv_ids = [
            IdValidity::Invalid(ProductId(11)),
            IdValidity::Invalid(ProductId(22)),
            IdValidity::Invalid(ProductId(99)),
            IdValidity::Invalid(ProductId(1010)),
            IdValidity::Invalid(ProductId(1188511885)),
            IdValidity::Invalid(ProductId(222222)),
            IdValidity::Invalid(ProductId(446446)),
            IdValidity::Invalid(ProductId(38593859)),
        ];

        let mut res: usize = 0;
        for id in inv_ids {
            res = id.add_up_invalid_ids(res);
        }

        assert_eq!(res, 1227775554);
    }

    #[test]
    fn test_add_up_invalid_ids_from_file() {
        let file_path = "./test";
        let res = IdValidity::add_up_invalid_ids_from_file(file_path);
        assert_eq!(res, 1227775554);
    }
}
