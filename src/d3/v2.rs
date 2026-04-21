pub fn partie1(input: &str) -> String {
    let mut total: u64 = 0;

    for line in input.lines() {
        let line = line.trim();

        let digits: Vec<u64> = line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
        let mut best: u64 = 0;

        let mut max: u64 = 0;
        for j in 0..digits.len() {
            let joltage = max * 10 + digits[j];
            if joltage > best {
                best = joltage;
            }
            if digits[j] > max {
                max = digits[j];
            }
        }

        total += best;
    }

    return total.to_string();
}

pub fn partie2(input: &str) -> String {
    let mut total: u64 = 0;

    for line in input.lines() {
        let line = line.trim();

        let digits: Vec<u64> = line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();

        let mut joltage: u64 = 0;
        let mut start: usize = 0;
        for pick in 0..12 {
            let end = digits.len() - (11 - pick);
            let mut best_digit = 0;
            let mut pos = 0;
            for i in start..end {
                if digits[i] > best_digit {
                    best_digit = digits[i];
                    pos = i - start;
                }
            }
            joltage = joltage * 10 + best_digit;
            start = start + pos + 1;
        }

        total += joltage;
    }

    return total.to_string();
}