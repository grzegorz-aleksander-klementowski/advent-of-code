use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Ścieżka do pliku wejściowego
    let sciezka_pliku = "input";

    // Otwieranie pliku
    let plik = File::open(sciezka_pliku)?;
    let czytnik = io::BufReader::new(plik);

    // Inicjalizacja wektorów
    let mut lista_lewa = Vec::new();
    let mut lista_prawa = Vec::new();

    // Odczyt pliku linia po linii
    for linia in czytnik.lines() {
        let linia = linia?;
        if let Some((lewa, prawa)) = linia.split_once(' ') {
            lista_lewa.push(lewa.trim().parse::<i32>().unwrap());
            lista_prawa.push(prawa.trim().parse::<i32>().unwrap());
        }
    }

    // Sortowanie obu list
    lista_lewa.sort();
    lista_prawa.sort();

    // ** Część pierwsza: Obliczanie sumy różnic **
    let suma_roznic: i32 = lista_lewa
        .iter()
        .zip(lista_prawa.iter())
        .map(|(lewa, prawa)| (lewa - prawa).abs())
        .sum();

    println!("Suma różnic: {}", suma_roznic);

    // ** Część druga: Obliczanie wskaźnika podobieństwa **
    // Zliczanie wystąpień liczb w prawej liście
    let mut mapa_wystapien: HashMap<i32, i32> = HashMap::new();
    for &prawa in &lista_prawa {
        *mapa_wystapien.entry(prawa).or_insert(0) += 1;
    }

    // Obliczanie wskaźnika podobieństwa
    let wskaznik_podobienstwa: i32 = lista_lewa
        .iter()
        .map(|&lewa| lewa * mapa_wystapien.get(&lewa).unwrap_or(&0))
        .sum();

    println!("Wskaźnik podobieństwa: {}", wskaznik_podobienstwa);

    Ok(())
}
