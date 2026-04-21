use crate::days::Day;

pub struct Day01;

impl Day for Day01 {
    fn titre(&self) -> &'static str { "Secret Entrance" }

    fn partie1(&self, input: &str) -> String {
        let mut pos = 50i32;
        let mut count = 0u32;
        for line in input.as_bytes().split(|&b| b == b'\n') {
            let line = trim_cr(line);
            if line.is_empty() { continue; }
            let dist = parse_int(&line[1..]);
            pos = match line[0] {
                b'L' => (pos - dist).rem_euclid(100),
                b'R' => (pos + dist).rem_euclid(100),
                _ => continue,
            };
            if pos == 0 { count += 1; }
        }
        count.to_string()
    }

    fn partie2(&self, input: &str) -> String {
        let mut pos = 50i32;
        let mut count = 0u64;
        for line in input.as_bytes().split(|&b| b == b'\n') {
            let line = trim_cr(line);
            if line.is_empty() { continue; }
            let dist = parse_int(&line[1..]);
            let (first_hit, new_pos) = match line[0] {
                b'L' => (if pos == 0 { 100 } else { pos },
                         (pos - dist).rem_euclid(100)),
                b'R' => (if pos == 0 { 100 } else { 100 - pos },
                         (pos + dist).rem_euclid(100)),
                _ => continue,
            };
            if dist >= first_hit {
                count += ((dist - first_hit) / 100 + 1) as u64;
            }
            pos = new_pos;
        }
        count.to_string()
    }
}

#[inline]
fn trim_cr(line: &[u8]) -> &[u8] {
    if line.last() == Some(&b'\r') { &line[..line.len() - 1] } else { line }
}

#[inline]
fn parse_int(bytes: &[u8]) -> i32 {
    bytes.iter().fold(0i32, |acc, &b| acc * 10 + (b - b'0') as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXEMPLE: &str = include_str!("../../inputs/day_01_test.txt");

    #[test]
    fn test_partie1_exemple() {
        assert_eq!(Day01.partie1(EXEMPLE), "3");
    }

    #[test]
    fn test_partie2_exemple() {
        assert_eq!(Day01.partie2(EXEMPLE), "6");
    }
}
