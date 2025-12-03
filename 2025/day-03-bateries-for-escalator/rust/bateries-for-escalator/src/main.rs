fn main() {
    let loaded_battery_bank: Vec<&str> = std::fs::read_to_string("input")
        .expect("Can't read the file! ")
        .lines()
        .collect();

    for bank in loaded_battery_bank {
        // to do
    }
}
