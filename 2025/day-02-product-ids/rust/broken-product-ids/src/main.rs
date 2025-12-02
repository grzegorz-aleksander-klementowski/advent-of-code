use std::ops::Range;

trait Input {
    fn load_ids_range() -> Vec<IdRange>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct IdRange {
    first_id: ProductId,
    last_id: ProductId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ProductId(usize);

#[derive(Debug, PartialEq, Eq)]
enum IdValidity {
    Valid(ProductId),
    Invalid(ProductId),
}

fn main() {
    println!("Hello, world!");
}

// --- tests --- \\

#[cfg(test)]
mod test {
    use super::*;
    // test parsing of text to range struct
    #[test]
    fn test_parsing() {
        let example = "11-22";
        let res = IdRange {
            first_id: ProductId(0),
            last_id: ProductId(0),
        };

        // fn'll place here

        assert_eq!(
            res,
            IdRange {
                first_id: ProductId(11),
                last_id: ProductId(22),
            }
        )
    }

    //test if the program load the file into the structues
    fn test_load_file_and_parse() {
        let path = "test";

        // fn'll place here

        let res: Vec<IdRange> = Vec::new();
        assert_eq!(
            res,
            vec![
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
            ]
        )
    }
    #[test]
    fn test_id_validity() {
        let inv_ids = [ProductId(55), ProductId(7474), ProductId(123123)];
        let valid_id = ProductId(101);

        // test if detect invalid ids
        for id in inv_ids {
            let res = IdValidity::Valid(id.clone()); // fn'll place here
            assert_eq!(res, IdValidity::Invalid(id));
        }

        // test if detect a valid id
        let res = IdValidity::Invalid(ProductId(101)); // fn'll place here
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
        let range = &id_ranges[0];
        // fn'll place here
        let res = 0;
        assert_eq!(res, 2);

        // checking usual in the case
        let range = &id_ranges[1];
        // fn'll place here
        let res = 0;
        assert_eq!(res, 1);

        // checking exceptions – if 0
        let range = &id_ranges[5];
        // fn'll place here
        let res = 0;
        assert_eq!(res, 0);

        let count: usize = 0;
        for range in &id_ranges {
            // fn'll place here
        }

        assert_eq!(count, 8);
    }

    #[test]
    fn test_adding_up_invalids_ids() {
        let inv_ids = [11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859];

        // fn'll place here

        let res = 0;

        assert_eq!(res, 1227775554);
    }
}
