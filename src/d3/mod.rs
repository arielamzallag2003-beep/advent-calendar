use crate::days::Day;

pub mod enzo;
pub mod v1;
pub mod v2;

pub struct Day03;

impl Day for Day03 {
    fn titre(&self) -> &'static str { "Lobby" }

    fn partie1(&self, input: &str) -> String {
        let mut total: u64 = 0;
        for line in input.as_bytes().split(|&b| b == b'\n') {
            let line = if line.last() == Some(&b'\r') { &line[..line.len()-1] } else { line };
            if line.is_empty() { continue; }

            let mut max_left: u8 = 0;
            let mut max_joltage: u64 = 0;
            let mut first = true;
            for &b in line {
                if b < b'0' || b > b'9' { continue; }
                let d = b - b'0';
                if first { first = false; }
                else {
                    let candidate = max_left as u64 * 10 + d as u64;
                    if candidate > max_joltage { max_joltage = candidate; }
                }
                if d > max_left { max_left = d; }
            }
            total += max_joltage;
        }
        total.to_string()
    }

    fn partie2(&self, input: &str) -> String {
        let mut total: u128 = 0;
        for line in input.as_bytes().split(|&b| b == b'\n') {
            let line = if line.last() == Some(&b'\r') { &line[..line.len()-1] } else { line };
            if line.is_empty() { continue; }

            let digits: Vec<u8> = line.iter()
                .filter(|&&b| b >= b'0' && b <= b'9')
                .map(|&b| b - b'0')
                .collect();

            let n = digits.len();
            total += selectionner_max(&digits, n, 12) as u128;
        }
        total.to_string()
    }
}

fn selectionner_max(digits: &[u8], n: usize, k: usize) -> u64 {
    let mut resultat: u64 = 0;
    let mut start = 0;
    for i in 0..k {
        let end = n - (k - i - 1);
        let tranche = &digits[start..end];
        let max_val = *tranche.iter().max().unwrap();
        let best_local = tranche.iter().position(|&v| v == max_val).unwrap();
        resultat = resultat * 10 + max_val as u64;
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
