use crate::days::Day;

pub struct Day03;

impl Day for Day03 {
    fn titre(&self) -> &'static str {
        "Lobby"
    }

    fn partie1(&self, input: &str) -> String {
        let mut total: u64 = 0;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let digits: Vec<u64> = line
                .chars()
                .filter_map(|c| c.to_digit(10).map(u64::from))
                .collect();

            let mut max_joltage: u64 = 0;

            for i in 0..digits.len() {
                for j in (i + 1)..digits.len() {
                    let joltage = digits[i] * 10 + digits[j];
                    if joltage > max_joltage {
                        max_joltage = joltage;
                    }
                }
            }

            total += max_joltage;
        }

        total.to_string()
    }

    fn partie2(&self, input: &str) -> String {
        let mut total: u128 = 0;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let digits: Vec<u128> = line
                .chars()
                .filter_map(|c| c.to_digit(10).map(u128::from))
                .collect();

            let n = digits.len();
            let k = 12;

            total += selectionner_max(&digits, n, k);
        }

        total.to_string()
    }
}

fn selectionner_max(digits: &[u128], n: usize, k: usize) -> u128 {
    let mut resultat: u128 = 0;
    let mut start = 0;

    for i in 0..k {
        let end = n - (k - i - 1);
        let tranche = &digits[start..end];
        let max_val = *tranche.iter().max().unwrap();
        let best_local = tranche.iter().position(|&v| v == max_val).unwrap();
        resultat = resultat * 10 + max_val;
        start += best_local + 1;
    }

    resultat
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXEMPLE: &str = include_str!("../../inputs/day_03_test.txt");

    #[test]
    fn test_partie1_exemple() {
        assert_eq!(Day03.partie1(EXEMPLE), "357");
    }

    #[test]
    fn test_partie2_exemple() {
        assert_eq!(Day03.partie2(EXEMPLE), "3121910778619");
    }
}
