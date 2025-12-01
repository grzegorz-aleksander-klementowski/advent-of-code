use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let sciezka = "./input";
    let plik = File::open(&sciezka)?;
    let czytnik = BufReader::new(plik);

    let mut linie = czytnik.lines().map(|l| l.unwrap());

    let mut reguly: Vec<(i32, i32)> = Vec::new();
    let mut aktualizacje: Vec<Vec<i32>> = Vec::new();

    // Wczytujemy reguły do momentu pustej linii
    loop {
        let linia = match linie.next() {
            Some(l) => l,
            None => break, // brak danych
        };
        let linia = linia.trim();
        if linia.is_empty() {
            break;
        }
        if let Some(idx) = linia.find('|') {
            let lewa = &linia[..idx];
            let prawa = &linia[idx + 1..];
            let x: i32 = lewa.trim().parse().expect("Błąd parsowania liczby");
            let y: i32 = prawa.trim().parse().expect("Błąd parsowania liczby");
            reguly.push((x, y));
        }
    }

    // Wczytujemy aktualizacje
    for linia in linie {
        let linia = linia.trim();
        if linia.is_empty() {
            continue;
        }
        let strony: Vec<i32> = linia
            .split(',')
            .map(|elem| elem.trim().parse().expect("Błąd parsowania strony"))
            .collect();
        aktualizacje.push(strony);
    }

    let mut suma_srodkowych_poprawnych = 0;
    let mut niepoprawne_aktualizacje: Vec<Vec<i32>> = Vec::new();

    'outer: for aktualizacja in &aktualizacje {
        let mut indeksy = HashMap::new();
        for (i, &strona) in aktualizacja.iter().enumerate() {
            indeksy.insert(strona, i);
        }

        // Sprawdzamy reguły dla aktualizacji
        for &(x, y) in &reguly {
            if let (Some(ix), Some(iy)) = (indeksy.get(&x), indeksy.get(&y)) {
                // Musi być x przed y
                if ix >= iy {
                    // Niepoprawna aktualizacja
                    niepoprawne_aktualizacje.push(aktualizacja.clone());
                    continue 'outer;
                }
            }
        }

        // Jeśli doszliśmy tu, aktualizacja jest poprawna
        let n = aktualizacja.len();
        let srodkowy_indeks = n / 2;
        suma_srodkowych_poprawnych += aktualizacja[srodkowy_indeks];
    }

    println!(
        "Suma środkowych stron poprawnych aktualizacji: {}",
        suma_srodkowych_poprawnych
    );

    // TERAZ DRUGA CZĘŚĆ:
    // Dla każdej niepoprawnej aktualizacji posortujemy strony wg reguł.
    let mut suma_srodkowych_niepoprawnych = 0;

    for aktualizacja in niepoprawne_aktualizacje {
        // Zbieramy strony tej aktualizacji:
        let zbior_stron: HashSet<i32> = aktualizacja.iter().cloned().collect();

        // Budujemy graf zależności istotny tylko dla tej aktualizacji
        // graf: strona -> strony, które muszą być po niej (są zależne)
        let mut graf: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut stopnie_wejscia: HashMap<i32, usize> = HashMap::new();

        for &s in &zbior_stron {
            graf.insert(s, Vec::new());
            stopnie_wejscia.insert(s, 0);
        }

        for &(x, y) in &reguly {
            if zbior_stron.contains(&x) && zbior_stron.contains(&y) {
                // Dodajemy krawędź x -> y
                graf.get_mut(&x).unwrap().push(y);
                *stopnie_wejscia.get_mut(&y).unwrap() += 1;
            }
        }

        // Sortowanie topologiczne (Kahn)
        let mut kolejka = VecDeque::new();
        for (&strona, &stopien) in &stopnie_wejscia {
            if stopien == 0 {
                kolejka.push_back(strona);
            }
        }

        let mut wynik_sortowania = Vec::new();
        while let Some(u) = kolejka.pop_front() {
            wynik_sortowania.push(u);
            if let Some(nastepniki) = graf.get(&u) {
                for &v in nastepniki {
                    let w = stopnie_wejscia.get_mut(&v).unwrap();
                    *w -= 1;
                    if *w == 0 {
                        kolejka.push_back(v);
                    }
                }
            }
        }

        // Załóżmy, że zawsze uda się posortować wszystkie strony
        let n = wynik_sortowania.len();
        let srodkowy_indeks = n / 2;
        suma_srodkowych_niepoprawnych += wynik_sortowania[srodkowy_indeks];
    }

    println!(
        "Suma środkowych stron niepoprawnych aktualizacji po poprawieniu: {}",
        suma_srodkowych_niepoprawnych
    );

    Ok(())
}
