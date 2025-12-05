mod grid {
    use crate::input::{Input, PathToDiagram};

    #[derive(Debug, PartialEq, Eq)]
    struct Forklift;
    impl Forklift {
        fn parse(diagram_char: char) -> Option<Self> {
            match diagram_char {
                '@' => Some(Forklift),
                '.' => None,
                _ => panic!("Found incorrect diagram char: {diagram_char}"),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    struct Field(Option<Forklift>);
    impl Field {
        fn new(forklift: Option<Forklift>) -> Self {
            Field(forklift)
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Grid(Vec<Vec<Field>>);
    impl Grid {
        fn parse(path: PathToDiagram) -> Self {
            let content = Input::load(&path);
            let mut grid: Vec<Vec<Field>> = Vec::new();
            for row in content.lines() {
                let mut grid_row: Vec<Field> = Vec::new();
                for char in row.chars() {
                    let forklift = Forklift::parse(char);
                    let field = Field::new(forklift);
                    grid_row.push(field);
                }
                grid.push(grid_row);
            }
            Grid::new(grid)
        }
        fn new(fields_vec: Vec<Vec<Field>>) -> Self {
            Grid(fields_vec)
        }
    }

    /* struct GridArea {
        height: usize,
        weight: usize,
        field: Field,
    } */

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_new_field() {
            let field_with_forklift = Field::new(Some(Forklift));
            let field_with_no_forklift = Field::new(None);

            let res = field_with_forklift;
            assert_eq!(res, Field(Some(Forklift)));

            let res = field_with_no_forklift;
            assert_eq!(res, Field(None));
        }
    }

    #[test]
    fn test_parse_forklift() {
        let forklift_char = '@';
        let no_forklift_char = '.';

        let res = Forklift::parse(forklift_char);
        assert_eq!(res, Some(Forklift));

        let res = Forklift::parse(no_forklift_char);
        assert_eq!(res, None);
    }

    #[test]
    fn test_parse_grid() {
        let path = PathToDiagram("test".to_string());
        let res = Grid::parse(path);
        assert_eq!(
            res,
            Grid(vec![
                vec![
                    Field(None),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(None)
                ],
                vec![
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(Some(Forklift))
                ],
                vec![
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(Some(Forklift))
                ],
                vec![
                    Field(None),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift))
                ],
                vec![
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(None)
                ],
                vec![
                    Field(None),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift))
                ],
                vec![
                    Field(None),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift))
                ],
                vec![
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift))
                ],
                vec![
                    Field(None),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift))
                ],
                vec![
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(Some(Forklift)),
                    Field(None),
                    Field(Some(Forklift)),
                    Field(None)
                ],
            ])
        );
    }
}

mod input {
    pub struct PathToDiagram(pub String);

    pub trait Input {
        fn load(&self) -> String;
    }

    impl Input for PathToDiagram {
        fn load(&self) -> String {
            std::fs::read_to_string(&self.0).unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
