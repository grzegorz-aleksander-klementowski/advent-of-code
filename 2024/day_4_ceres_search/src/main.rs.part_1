use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "./input";

    // Próbujemy wczytać siatkę z pliku
    let grid = match read_grid(filename) {
        Ok(grid) => grid,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            return;
        }
    };

    if grid.is_empty() || grid[0].is_empty() {
        eprintln!("Error: Grid is empty or invalid.");
        return;
    }

    let word = "XMAS";

    // Definicja ośmiu kierunków
    let directions = [
        (0, 1),   // W prawo
        (1, 1),   // W dół i prawo (ukośnie)
        (1, 0),   // W dół
        (1, -1),  // W dół i lewo (ukośnie)
        (0, -1),  // W lewo
        (-1, -1), // W górę i lewo (ukośnie)
        (-1, 0),  // W górę
        (-1, 1),  // W górę i prawo (ukośnie)
    ];

    let mut count = 0;

    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    // Iteracja po każdej komórce siatki
    for y in 0..rows {
        for x in 0..cols {
            for &(dx, dy) in &directions {
                if check_word(&grid, x, y, dx, dy, word) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

// Funkcja do wczytania siatki z pliku
fn read_grid(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let mut grid = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(content) = line {
                grid.push(content.chars().collect());
            }
        }
    }

    Ok(grid)
}

// Funkcja sprawdzająca, czy słowo istnieje w danym kierunku
fn check_word(
    grid: &Vec<Vec<char>>,
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    word: &str,
) -> bool {
    let chars: Vec<char> = word.chars().collect();
    let word_len = chars.len() as isize;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for i in 0..word_len {
        let nx = x + dx * i;
        let ny = y + dy * i;

        if nx < 0 || ny < 0 || nx >= cols || ny >= rows {
            return false;
        }

        if grid[ny as usize][nx as usize] != chars[i as usize] {
            return false;
        }
    }

    true
}

// Funkcja pomocnicza do wczytania linii z pliku
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

