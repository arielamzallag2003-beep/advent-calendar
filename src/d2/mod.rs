use crate::days::Day;

pub mod enzo;
pub mod v1;
pub mod v2;

pub struct Day02;

impl Day for Day02 {
    fn titre(&self) -> &'static str { "Gift Shop" }

    fn partie1(&self, input: &str) -> String {
        parse_ranges(input)
            .map(|(s, e)| sum_half_repeat(s, e))
            .sum::<u64>()
            .to_string()
    }

    fn partie2(&self, input: &str) -> String {
        parse_ranges(input)
            .map(|(s, e)| sum_repeating(s, e))
            .sum::<u64>()
            .to_string()
    }
}

fn parse_ranges(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    input.split(',').filter_map(|r| {
        let r = r.trim();
        let (s, e) = r.split_once('-')?;
        let start = s.trim().parse::<u64>().ok()?;
        let end   = e.trim().parse::<u64>().ok()?;
        Some((start, end))
    })
}

fn sum_half_repeat(start: u64, end: u64) -> u64 {
    let mut total = 0u128;
    for k in 1u32..=9 {
        let m     = pow10(k) + 1;
        let a_lo  = if k == 1 { 1 } else { pow10(k - 1) };
        let a_hi  = pow10(k) - 1;
        let lo    = div_ceil(start, m).max(a_lo);
        let hi    = (end / m).min(a_hi);
        if lo > hi { continue; }
        total += m as u128 * arith_sum(lo, hi);
    }
    total as u64
}

fn sum_repeating(start: u64, end: u64) -> u64 {
    let mut total = 0u128;
    for p in 1u32..=9 {
        for r in 2..=(19 / p) {
            let rep  = repunit(p, r);
            let a_lo = if p == 1 { 1 } else { pow10(p - 1) };
            let a_hi = pow10(p) - 1;
            let lo   = div_ceil(start, rep).max(a_lo);
            let hi   = (end / rep).min(a_hi);
            if lo > hi { continue; }
            let s = sum_prim(p, lo, hi);
            total += rep as u128 * s as u128;
        }
    }
    total as u64
}

fn sum_prim(p: u32, lo: u64, hi: u64) -> u128 {
    let mut result = 0i128;
    for &d in divisors(p) {
        let mu = mobius(p / d);
        if mu == 0 { continue; }
        let irep      = repunit(d, p / d);
        let b_lo_min  = if d == 1 { 1 } else { pow10(d - 1) };
        let b_hi_max  = pow10(d) - 1;
        let bl = div_ceil(lo, irep).max(b_lo_min);
        let bh = (hi / irep).min(b_hi_max);
        if bl > bh { continue; }
        result += mu as i128 * irep as i128 * arith_sum(bl, bh) as i128;
    }
    result as u128
}

#[inline]
fn pow10(k: u32) -> u64 { 10u64.pow(k) }

#[inline]
fn div_ceil(a: u64, b: u64) -> u64 { (a + b - 1) / b }

fn arith_sum(lo: u64, hi: u64) -> u128 {
    (hi - lo + 1) as u128 * (lo as u128 + hi as u128) / 2
}

fn repunit(p: u32, r: u32) -> u64 {
    let base = pow10(p);
    let mut val = 1u64;
    let mut term = 1u64;
    for _ in 1..r {
        term *= base;
        val  += term;
    }
    val
}

fn divisors(p: u32) -> &'static [u32] {
    match p {
        1 => &[1],
        2 => &[1, 2],
        3 => &[1, 3],
        4 => &[1, 2, 4],
        5 => &[1, 5],
        6 => &[1, 2, 3, 6],
        7 => &[1, 7],
        8 => &[1, 2, 4, 8],
        9 => &[1, 3, 9],
        _ => &[],
    }
}

fn mobius(n: u32) -> i8 {
    match n {
        1 => 1,
        2 | 3 | 5 | 7 => -1,
        6 => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXEMPLE: &str = include_str!("../../inputs/day_02_test.txt");

    #[test]
    fn test_est_invalide() {
        assert!(sum_half_repeat(11, 11) == 11);
        assert!(sum_half_repeat(22, 22) == 22);
        assert!(sum_half_repeat(1010, 1010) == 1010);
        assert!(sum_half_repeat(1188511885, 1188511885) == 1188511885);
        assert!(sum_half_repeat(12, 12) == 0);
        assert!(sum_half_repeat(101, 101) == 0);
        assert!(sum_half_repeat(1001, 1001) == 0);
    }

    #[test]
    fn test_est_invalide_v2() {
        assert!(sum_repeating(111, 111) == 111);
        assert!(sum_repeating(999, 999) == 999);
        assert!(sum_repeating(565656, 565656) == 565656);
        assert!(sum_repeating(824824824, 824824824) == 824824824);
        assert!(sum_repeating(2121212121, 2121212121) == 2121212121);
        assert!(sum_repeating(1698522, 1698522) == 0);
        assert!(sum_repeating(1001, 1001) == 0);
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
