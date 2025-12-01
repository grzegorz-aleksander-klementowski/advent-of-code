use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Kierunek {
    Gora,
    Prawo,
    Dol,
    Lewo,
}

impl Kierunek {
    fn obrot_w_prawo(&self) -> Kierunek {
        match self {
            Kierunek::Gora => Kierunek::Prawo,
            Kierunek::Prawo => Kierunek::Dol,
            Kierunek::Dol => Kierunek::Lewo,
            Kierunek::Lewo => Kierunek::Gora,
        }
    }

    fn wektor_ruchu(&self) -> (isize, isize) {
        match self {
            Kierunek::Gora => (-1, 0),
            Kierunek::Prawo => (0, 1),
            Kierunek::Dol => (1, 0),
            Kierunek::Lewo => (0, -1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Stan {
    r: isize,
    c: isize,
    kier: Kierunek,
}

fn main() {
    // Wczytanie mapy
    let plik = File::open("./input").expect("Nie udało się otworzyć pliku z mapą.");
    let czytnik = BufReader::new(plik);

    let mut mapa: Vec<Vec<char>> = Vec::new();
    for linia in czytnik.lines() {
        let linia = linia.expect("Błąd odczytu linii z pliku.");
        mapa.push(linia.chars().collect());
    }

    let wysokosc = mapa.len();
    let szerokosc = if wysokosc > 0 { mapa[0].len() } else { 0 };

    // Znajdź strażnika i jego kierunek
    let mut pozycja_straznika: (isize, isize) = (-1, -1);
    let mut kierunek_straznika = Kierunek::Gora; // Domyślnie, zostanie ustawiony właściwie

    'szukaj: for (r, wiersz) in mapa.iter().enumerate() {
        for (c, ch) in wiersz.iter().enumerate() {
            match ch {
                '^' => {
                    pozycja_straznika = (r as isize, c as isize);
                    kierunek_straznika = Kierunek::Gora;
                    break 'szukaj;
                }
                'v' => {
                    pozycja_straznika = (r as isize, c as isize);
                    kierunek_straznika = Kierunek::Dol;
                    break 'szukaj;
                }
                '<' => {
                    pozycja_straznika = (r as isize, c as isize);
                    kierunek_straznika = Kierunek::Lewo;
                    break 'szukaj;
                }
                '>' => {
                    pozycja_straznika = (r as isize, c as isize);
                    kierunek_straznika = Kierunek::Prawo;
                    break 'szukaj;
                }
                _ => {}
            }
        }
    }

    if pozycja_straznika == (-1, -1) {
        eprintln!("Nie znaleziono strażnika na mapie.");
        return;
    }

    // Pozycja startowa strażnika nie może być użyta
    let start_r = pozycja_straznika.0;
    let start_c = pozycja_straznika.1;

    // Funkcja sprawdzająca czy dane pole to przeszkoda
    let czy_przeszkoda = |r: isize, c: isize, mapa: &Vec<Vec<char>>| {
        if r < 0 || c < 0 || r >= wysokosc as isize || c >= szerokosc as isize {
            // Poza mapą
            true
        } else {
            mapa[r as usize][c as usize] == '#'
        }
    };

    // Funkcja sprawdzająca czy symulacja z nową przeszkodą prowadzi do pętli
    let sprawdz_petle = |mapa: &Vec<Vec<char>>| -> bool {
        let mut r = start_r;
        let mut c = start_c;
        let mut kier = kierunek_straznika;

        let mut stany: HashSet<Stan> = HashSet::new();

        // Aby zapobiec nieskończonej symulacji na gigantycznej mapie, można
        // wprowadzić limit iteracji: max liczba stanów to wysokosc * szerokosc * 4 kierunki

        let max_kroki = (wysokosc * szerokosc * 4) + 1;

        // (można zmodyfikować wedle uznania, 1 mln to sporo, ale zabezpiecza przed zawieszeniem)

        for _ in 0..max_kroki {
            let stan = Stan { r, c, kier };
            if stany.contains(&stan) {
                // Wpada w pętlę
                return true;
            }
            stany.insert(stan);

            let (dr, dc) = kier.wektor_ruchu();
            let nr = r + dr;
            let nc = c + dc;

            if nr < 0 || nc < 0 || nr >= wysokosc as isize || nc >= szerokosc as isize {
                // Strażnik wychodzi poza mapę - brak pętli
                return false;
            }

            if czy_przeszkoda(nr, nc, &mapa) {
                // Skręć w prawo
                kier = kier.obrot_w_prawo();
            } else {
                // Idź do przodu
                r = nr;
                c = nc;
                // Sprawdź, czy wyszedł poza mapę po ruchu
                if r < 0 || c < 0 || r >= wysokosc as isize || c >= szerokosc as isize {
                    return false;
                }
            }
        }

        // Jeśli osiągnęliśmy limit bez wyjścia poza mapę i bez powtarzającego się stanu
        // to prawdopodobnie brak pętli (lub za mały limit)
        // Ale zachowajmy bezpiecznie: jeśli limit jest duży, to raczej pętli nie ma.
        false
    };

    let mut licznik_petli = 0;

    // Sprawdź wszystkie możliwe miejsca na umieszczenie przeszkody
    for r in 0..(wysokosc as isize) {
        for c in 0..(szerokosc as isize) {
            if r == start_r && c == start_c {
                continue; // Nie można postawić na pozycji startowej strażnika
            }

            let pole = mapa[r as usize][c as usize];
            // Szukamy pustych pól, np. '.'
            if pole == '.' {
                // Umieść przeszkodę
                mapa[r as usize][c as usize] = '#';
                // Sprawdź czy teraz wystąpi pętla
                let petla = sprawdz_petle(&mapa);
                if petla {
                    licznik_petli += 1;
                }
                // Przywróć stan
                mapa[r as usize][c as usize] = '.';
            }
        }
    }

    println!("{}", licznik_petli);
}
