use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Uruchomienie curl
    let output = Command::new("curl")
        .arg("-s") // -s: tryb cichy (bez informacji o postępie)
        .arg("http://example.com")
        .output()?; // Uruchomienie procesu

    // Sprawdzenie statusu wyjścia
    if output.status.success() {
        // Wypisanie odpowiedzi na stdout
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        // Wypisanie błędu na stderr
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
