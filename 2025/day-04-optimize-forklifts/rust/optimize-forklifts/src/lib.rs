mod grid {
    use crate::input::{Input, PathToDiagram};

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

    #[derive(Default)]
    struct Field(Option<Forklift>);
    impl Field {
        fn new(forklift: Option<Forklift>) -> Self {
            Field(forklift)
        }
    }

    struct GridArea {
        height: usize,
        weight: usize,
        field: Field,
    }

    struct Grid(Vec<Vec<Field>>);
    impl Input for GridArea {
        fn load(&self) -> String {
            let path = PathToDiagram(String::from("input"));
            todo!()
        }
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
