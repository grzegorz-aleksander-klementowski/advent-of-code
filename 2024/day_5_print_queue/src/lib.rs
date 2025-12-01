use std::fs::File;
use std::io::{self, BufRead};

pub struct PoleceniaDoDrukarki {
    ścieżka: &'static str,
}

impl PoleceniaDoDrukarki {
    pub fn nowy(ścieżka: &'static str) -> Self {
        Self { ścieżka }
    }

    fn czytaj(&self) -> io::BufReader<File> {
        let polecenia: File = File::open(self.ścieżka)
            .expect("Nie można odnaleźć poleceń. Nazwa zapisku winna nosić nazwę „input”");
        let czytnik_poleceń: io::BufReader<File> = io::BufReader::new(polecenia);
        return czytnik_poleceń;
    }

    pub fn ilość_zestawień_stron(&self) -> usize {
        let ilość_linii_w_zapisku: usize = self.czytaj().lines().count();
        return ilość_linii_w_zapisku;
    }
}
