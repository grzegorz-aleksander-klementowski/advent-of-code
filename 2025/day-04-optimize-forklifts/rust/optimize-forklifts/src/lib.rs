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
            let mut grid_row: Vec<Field> = Vec::new();
            let mut grid: Vec<Vec<Field>> = Vec::new();
            /* for row in content.lines() {

            } */
            todo!()
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

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

    struct GridArea {
        height: usize,
        weight: usize,
        field: Field,
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
