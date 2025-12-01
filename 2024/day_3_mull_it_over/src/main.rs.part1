use std::fs;

fn main() {
    // Wczytaj zawartość pliku z pomieszaną pamięcią
    let input = fs::read_to_string("./zapiski/pomieszana_pamięć")
        .expect("Nie udało się odczytać pliku pomieszana_pamięć");

    let mut total = 0;
    let input_bytes = input.as_bytes();
    let input_len = input_bytes.len();
    let mut i = 0;

    while i < input_len {
        // Szukamy 'm' jako potencjalny początek 'mul('
        if input_bytes[i] == b'm' {
            if i + 3 < input_len && &input_bytes[i..i+4] == b"mul(" {
                let mut j = i + 4;
                let mut num1 = String::new();
                // Parsujemy pierwszą liczbę
                while j < input_len && input_bytes[j].is_ascii_digit() {
                    num1.push(input_bytes[j] as char);
                    j += 1;
                }
                // Sprawdzamy czy po pierwszej liczbie jest przecinek
                if !num1.is_empty() && j < input_len && input_bytes[j] == b',' {
                    j += 1;
                    let mut num2 = String::new();
                    // Parsujemy drugą liczbę
                    while j < input_len && input_bytes[j].is_ascii_digit() {
                        num2.push(input_bytes[j] as char);
                        j += 1;
                    }
                    // Sprawdzamy czy po drugiej liczbie jest zamykający nawias
                    if !num2.is_empty() && j < input_len && input_bytes[j] == b')' {
                        // Upewniamy się, że liczby mają 1-3 cyfry
                        if num1.len() <= 3 && num2.len() <= 3 {
                            let x: i32 = num1.parse().unwrap();
                            let y: i32 = num2.parse().unwrap();
                            total += x * y;
                        }
                        j += 1;
                        i = j;
                        continue;
                    }
                }
            }
        }
        i += 1;
    }

    println!("Suma wyników mnożeń: {}", total);
}

