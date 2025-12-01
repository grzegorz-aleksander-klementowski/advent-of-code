use std::fs;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Ścieżka do pliku z danymi
    let ścieżka_do_pliku = "./zapiski/sprawozdania_o_poziomach";

    // Otwieramy plik i buforujemy go
    let plik = fs::File::open(ścieżka_do_pliku)?;
    let czytnik = io::BufReader::new(plik);

    // Inicjalizujemy licznik bezpiecznych raportów
    let mut liczba_bezpiecznych = 0;

    // Przetwarzamy każdą linię
    for linia in czytnik.lines() {
        let linia = linia?;
        // Zamiana linii na wektor liczb
        let poziomy: Vec<i32> = linia
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        // Sprawdzamy, czy raport jest bezpieczny lub czy może być bezpieczny po usunięciu jednego poziomu
        if czy_bezpieczny(&poziomy) || czy_bezpieczny_po_usunięciu(&poziomy) {
            liczba_bezpiecznych += 1;
        }
    }

    println!(
        "Liczba bezpiecznych raportów (z tłumikiem): {}",
        liczba_bezpiecznych
    );

    Ok(())
}

// Funkcja sprawdzająca, czy raport jest bezpieczny
fn czy_bezpieczny(poziomy: &[i32]) -> bool {
    if poziomy.len() < 2 {
        return false; // Raport musi mieć co najmniej 2 poziomy
    }

    // Sprawdź, czy różnice są w zakresie 1..=3
    let różnice: Vec<i32> = poziomy.windows(2).map(|para| para[1] - para[0]).collect();

    let wszystkie_poprawne_różnice = różnice.iter().all(|&r| r.abs() >= 1 && r.abs() <= 3);

    // Sprawdź, czy poziomy są monotoniczne (rosnące lub malejące)
    let monotoniczny = różnice.iter().all(|&r| r > 0) || różnice.iter().all(|&r| r < 0);

    wszystkie_poprawne_różnice && monotoniczny
}

// Funkcja sprawdzająca, czy raport jest bezpieczny po usunięciu jednego poziomu
fn czy_bezpieczny_po_usunięciu(poziomy: &[i32]) -> bool {
    for i in 0..poziomy.len() {
        // Tworzymy nowy raport bez poziomu na indeksie `i`
        let mut nowy_raport = poziomy.to_vec();
        nowy_raport.remove(i);

        // Sprawdzamy, czy nowy raport jest bezpieczny
        if czy_bezpieczny(&nowy_raport) {
            return true;
        }
    }
    false
}
