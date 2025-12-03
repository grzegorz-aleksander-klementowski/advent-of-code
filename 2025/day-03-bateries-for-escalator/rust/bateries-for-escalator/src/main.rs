use std::fs;

fn best_joltage(bank: &str, k: usize) -> u64 {
    let mut stack: Vec<char> = Vec::with_capacity(k);
    let mut to_remove = bank.len().saturating_sub(k);

    for c in bank.chars() {
        while to_remove > 0 && !stack.is_empty() && stack.last().unwrap() < &c {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(c);
    }

    stack.truncate(k);

    let s: String = stack.into_iter().collect();
    s.parse::<u64>().expect("Can't parse joltage")
}

fn main() {
    let file_content = fs::read_to_string("input").expect("Can't read the file!");
    let mut total: u64 = 0;

    for bank in file_content.lines() {
        let bank_joltage = best_joltage(bank, 12);
        total += bank_joltage;
    }

    println!("Result (part 2): {total}");
}

/* fn best_bank_joltage(bank: &str) -> u32 {

    let digits: Vec<u32> = bank
        .chars()
        .map(|c| c.to_digit(10).expect("Non-digit in input"))
        .collect();

    let mut best = 0u32;

    for i in 0..digits.len() {
        for j in i + 1..digits.len() {
            let val = digits[i] * 10 + digits[j];
            if val > best {
                best = val;
            }
        }
    }

    best
}

fn main() {
    let file_content = std::fs::read_to_string("input").expect("Can't read the file!");
    let mut counter_battery_joltage: u32 = 0;

    for bank in file_content.lines() {
        counter_battery_joltage += best_bank_joltage(bank);
    }

    println!("Result: {counter_battery_joltage}");
} */

/* fn main() {
    let file_content = std::fs::read_to_string("input").expect("Can't read the file! ");
    let loaded_battery_bank: Vec<&str> = file_content.lines().collect();
    let mut counter_battery_joltage = 0;

    for bank in &loaded_battery_bank {
        // Load the bateries as to the bank as digit
        let mut bank_as_dig: Vec<(usize, usize)> = Vec::new();
        for (index, char) in bank.chars().enumerate() {
            let n = char.to_digit(10).expect("It's not a number! ") as usize;
            bank_as_dig.push((index, n));
        }

        // sort to get the biggest number
        bank_as_dig.sort_by_key(|x| x.1);
        bank_as_dig.reverse();

        let i_first_max = bank_as_dig[0].0;
        bank_as_dig.sort_by_key(|x| x.0);
        let mut first_battery = bank_as_dig[i_first_max];
        let mut second_battery: (usize, usize) = (0, 0);
        // -- fist battery number is OK

        let mut i_second_max = 0;
        bank_as_dig.sort_by_key(|x| x.0);

        let bank_len = bank_as_dig.len() - 1;
        if i_first_max == bank_len {
            // in this case the first label is the last one
            second_battery = first_battery;
            bank_as_dig.sort_by_key(|x| x.1);
            bank_as_dig.reverse();
            first_battery = bank_as_dig[1];
        } else {
            // separating the rest of the bank to find another biggest
            let mut rest = bank_as_dig[i_first_max + 1..].to_vec();
            rest.sort_by_key(|x| x.1);
            rest.reverse();
            i_second_max = rest[0].0;
            second_battery = bank_as_dig[i_second_max];
        }

        let label_1 = first_battery.1.to_string();
        let label_2 = second_battery.1.to_string();
        let battery_to_be_trun_on: usize = format!("{label_1}{label_2}")
            .parse()
            .expect("Can't parse the string to number!");
        counter_battery_joltage += battery_to_be_trun_on;
    }

    println!("Result: {counter_battery_joltage}");
} */
