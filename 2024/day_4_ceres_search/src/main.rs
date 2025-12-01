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

    let mut count = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    // Iteracja po każdej możliwej pozycji centrum "X"
    for y in 1..rows - 1 {
        for x in 1..cols - 1 {
            if is_x_mas(&grid, x, y) {
                count += 1;
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

// Sprawdzanie, czy w pozycji (x, y) znajduje się "X-MAS"
fn is_x_mas(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    // Sprawdzanie granic
    if x == 0 || y == 0 || x >= cols - 1 || y >= rows - 1 {
        return false;
    }

    // Pobieranie znaków dla wzoru "X-MAS"
    let top_left = grid[y - 1][x - 1];
    let top_right = grid[y - 1][x + 1];
    let center = grid[y][x];
    let bottom_left = grid[y + 1][x - 1];
    let bottom_right = grid[y + 1][x + 1];

    // Sprawdzanie pierwszego ukośnego "MAS"
    let mas1 = top_left == 'M' && center == 'A' && bottom_right == 'S';
    let mas2 = top_left == 'S' && center == 'A' && bottom_right == 'M';

    // Sprawdzanie drugiego ukośnego "MAS"
    let mas3 = top_right == 'M' && center == 'A' && bottom_left == 'S';
    let mas4 = top_right == 'S' && center == 'A' && bottom_left == 'M';

    // Wzór "X-MAS" spełniony, jeśli obie ukośne linie tworzą "MAS"
    (mas1 || mas2) && (mas3 || mas4)
}

// Funkcja pomocnicza do wczytania linii z pliku
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
