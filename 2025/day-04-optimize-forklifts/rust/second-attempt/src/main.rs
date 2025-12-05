use std::io::{self, Read};

fn main() {
    // Wczytaj całe wejście jako jeden string
    let input = std::fs::read_to_string("input").expect("Nie mogę odczytać pliku!!");

    // Zamień na siatkę znaków
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        println!("0");
        return;
    }

    let h = grid.len();
    let w = grid[0].len();

    // 8 kierunków sąsiadów
    let directions: &[(isize, isize)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut total_removed = 0usize;

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        // Znajdź wszystkie dostępne rolki w tej rundzie
        for y in 0..h {
            for x in 0..w {
                if grid[y][x] != '@' {
                    continue;
                }

                let mut neighbor_rolls = 0usize;

                for (dy, dx) in directions {
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;

                    if ny < 0 || nx < 0 {
                        continue;
                    }

                    let ny = ny as usize;
                    let nx = nx as usize;

                    if ny >= h || nx >= w {
                        continue;
                    }

                    if grid[ny][nx] == '@' {
                        neighbor_rolls += 1;
                    }
                }

                if neighbor_rolls < 4 {
                    to_remove.push((y, x));
                }
            }
        }

        // Jeśli w tej rundzie nie ma co usuwać, koniec
        if to_remove.is_empty() {
            break;
        }

        // Usuń wszystkie zaznaczone rolki
        for (y, x) in to_remove.into_iter() {
            grid[y][x] = '.'; // albo 'x', nie ma znaczenia: liczą się tylko '@'
            total_removed += 1;
        }
    }

    // Wynik dla Part 2
    println!("{}", total_removed);
}
