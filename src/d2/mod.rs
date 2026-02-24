use crate::days::Day;

pub struct Day02;

impl Day for Day02 {
    fn titre(&self) -> &'static str {
        "Gift Shop"
    }

    fn partie1(&self, input: &str) -> String {
        let mut total: u64 = 0;

        for range_str in input.split(',') {
            let range_str = range_str.trim();
            if range_str.is_empty() {
                continue;
            }
            let Some((start_str, end_str)) = range_str.split_once('-') else {
                continue;
            };
            let (Ok(start), Ok(end)) = (start_str.parse::<u64>(), end_str.parse::<u64>()) else {
                continue;
            };

            for n in start..=end {
                if est_invalide(n) {
                    total += n;
                }
            }
        }

        total.to_string()
    }

    fn partie2(&self, input: &str) -> String {
        let mut total: u64 = 0;

        for range_str in input.split(',') {
            let range_str = range_str.trim();
            if range_str.is_empty() {
                continue;
            }
            let Some((start_str, end_str)) = range_str.split_once('-') else {
                continue;
            };
            let (Ok(start), Ok(end)) = (start_str.parse::<u64>(), end_str.parse::<u64>()) else {
                continue;
            };

            for n in start..=end {
                if est_invalide_v2(n) {
                    total += n;
                }
            }
        }

        total.to_string()
    }
}


fn est_invalide(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }
    let half = len / 2;
    s[..half] == s[half..]
}

fn est_invalide_v2(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    for pattern_len in 1..len {
        if len % pattern_len != 0 {
            continue;
        }
        let pattern = &s[..pattern_len];
        if pattern.repeat(len / pattern_len) == s {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXEMPLE: &str = include_str!("../../inputs/day_02_test.txt");

    #[test]
    fn test_est_invalide() {
        assert!(est_invalide(11));
        assert!(est_invalide(22));
        assert!(est_invalide(1010));
        assert!(est_invalide(1188511885));
        assert!(!est_invalide(12));
        assert!(!est_invalide(101));
        assert!(!est_invalide(1001)); // "10" != "01"
    }

    #[test]
    fn test_est_invalide_v2() {
        assert!(est_invalide_v2(111));        // "1" * 3
        assert!(est_invalide_v2(999));        // "9" * 3
        assert!(est_invalide_v2(565656));     // "56" * 3
        assert!(est_invalide_v2(824824824));  // "824" * 3
        assert!(est_invalide_v2(2121212121)); // "21" * 5
        assert!(!est_invalide_v2(1698522));
        assert!(!est_invalide_v2(1001)); // "10" != "01"
    }

    #[test]
    fn test_partie1_exemple() {
        assert_eq!(Day02.partie1(EXEMPLE), "1227775554");
    }

    #[test]
    fn test_partie2_exemple() {
        assert_eq!(Day02.partie2(EXEMPLE), "4174379265");
    }
}
