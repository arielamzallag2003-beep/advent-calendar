use crate::days::Day;
use rayon::prelude::*;

pub struct Day10;

impl Day for Day10 {
    fn titre(&self) -> &'static str { "Factory" }

    fn partie1(&self, input: &str) -> String {
        let mut visited: Vec<bool> = Vec::with_capacity(1024);
        let mut current: Vec<u64>  = Vec::with_capacity(1024);
        let mut next:    Vec<u64>  = Vec::with_capacity(1024);
        let mut buttons: Vec<u64>  = Vec::with_capacity(32);
        let mut total = 0u64;
        for line in input.lines() {
            if line.trim().is_empty() { continue; }
            let target = parse_lights_into(line, &mut buttons);
            total += min_presses_xor(target, &buttons, &mut visited, &mut current, &mut next);
        }
        total.to_string()
    }

    fn partie2(&self, input: &str) -> String {
        input
            .par_lines()
            .filter(|l| !l.trim().is_empty())
            .map(|line| {
                let (target, buttons) = parse_joltage(line);
                solve_p2(&target, &buttons)
            })
            .sum::<u64>()
            .to_string()
    }
}

fn parse_lights_into(line: &str, buttons: &mut Vec<u64>) -> u64 {
    buttons.clear();
    let bytes = line.as_bytes();
    let len   = bytes.len();
    let mut i = 0;

    while i < len && bytes[i] != b'[' { i += 1; }
    i += 1;

    let mut target = 0u64;
    let mut bit = 0;
    while i < len && bytes[i] != b']' {
        if bytes[i] == b'#' { target |= 1 << bit; }
        bit += 1;
        i  += 1;
    }
    i += 1;

    while i < len {
        if bytes[i] == b'{' { break; }
        if bytes[i] != b'(' { i += 1; continue; }
        i += 1;
        let mut mask = 0u64;
        let mut num  = 0u64;
        let mut in_num = false;
        while i < len && bytes[i] != b')' {
            let b = bytes[i];
            if b >= b'0' && b <= b'9' {
                num = num * 10 + (b - b'0') as u64;
                in_num = true;
            } else if b == b',' && in_num {
                mask |= 1 << num;
                num = 0; in_num = false;
            }
            i += 1;
        }
        if in_num { mask |= 1 << num; }
        buttons.push(mask);
        i += 1;
    }
    target
}

fn parse_button_masks(section: &str) -> Vec<u64> {
    let brace = section.find('{').unwrap_or(section.len());
    let mut buttons = Vec::new();
    let mut pos = 0;
    let s = &section[..brace];
    while let Some(rel) = s[pos..].find('(') {
        let abs = pos + rel;
        let end = abs + s[abs..].find(')').unwrap();
        let mask = s[abs + 1..end]
            .split(',')
            .filter_map(|p| p.trim().parse::<usize>().ok())
            .fold(0u64, |acc, i| acc | (1 << i));
        buttons.push(mask);
        pos = end + 1;
    }
    buttons
}

fn min_presses_xor(
    target: u64, buttons: &[u64],
    visited: &mut Vec<bool>,
    current: &mut Vec<u64>,
    next:    &mut Vec<u64>,
) -> u64 {
    if target == 0 { return 0; }

    let all_bits = buttons.iter().fold(target, |acc, &b| acc | b);
    let n_bits   = (64 - all_bits.leading_zeros()) as usize;
    let n_states = 1usize << n_bits;

    if visited.len() < n_states { visited.resize(n_states, false); }
    visited[..n_states].fill(false);
    visited[0] = true;

    current.clear();
    next.clear();
    current.push(0);
    let mut depth = 0u64;

    loop {
        depth += 1;
        next.clear();
        for &state in current.iter() {
            for &btn in buttons {
                let new = state ^ btn;
                if new == target { return depth; }
                if !visited[new as usize] {
                    visited[new as usize] = true;
                    next.push(new);
                }
            }
        }
        if next.is_empty() { return 0; }
        std::mem::swap(current, next);
    }
}

fn parse_joltage(line: &str) -> (Vec<i64>, Vec<u64>) {
    let brace_start = line.find('{').unwrap();
    let brace_end   = line.find('}').unwrap();
    let target: Vec<i64> = line[brace_start + 1..brace_end]
        .split(',')
        .filter_map(|p| p.trim().parse().ok())
        .collect();
    let after_bracket = &line[line.find(']').unwrap() + 1..];
    let buttons = parse_button_masks(after_bracket);
    (target, buttons)
}

fn solve_p2(target: &[i64], buttons: &[u64]) -> u64 {
    if target.iter().all(|&t| t == 0) { return 0; }
    if buttons.is_empty() { return 0; }

    let n = target.len();
    let m = buttons.len();
    let might_help = target.iter().any(|&t| t == 0)
        || (0..n).any(|j| target[j] > 0
            && (0..m).filter(|&i| (buttons[i] >> j) & 1 != 0).count() == 1);

    if might_help {
        if let Some(total) = try_greedy(target, buttons) { return total; }
    }
    solve_with_z3(target, buttons)
}

fn try_greedy(target: &[i64], buttons: &[u64]) -> Option<u64> {
    let n = target.len();
    let m = buttons.len();
    let mut x         = vec![0i64; m];
    let mut decided   = vec![false; m];
    let mut remaining = target.to_vec();

    let mut progress = true;
    while progress {
        progress = false;

        for j in 0..n {
            if remaining[j] != 0 { continue; }
            for i in 0..m {
                if decided[i] || (buttons[i] >> j) & 1 == 0 { continue; }
                for k in 0..n {
                    if (buttons[i] >> k) & 1 != 0 && remaining[k] > 0 {
                        let covered = (0..m)
                            .any(|i2| i2 != i && !decided[i2] && (buttons[i2] >> k) & 1 != 0);
                        if !covered { return None; }
                    }
                }
                x[i]       = 0;
                decided[i] = true;
                progress   = true;
            }
        }

        for j in 0..n {
            if remaining[j] == 0 { continue; }
            if remaining[j] < 0  { return None; }

            let covering: Vec<usize> = (0..m)
                .filter(|&i| !decided[i] && (buttons[i] >> j) & 1 != 0)
                .collect();

            if covering.len() != 1 { continue; }
            let i    = covering[0];
            let need = remaining[j];

            for j2 in 0..n {
                if (buttons[i] >> j2) & 1 != 0 && remaining[j2] < need {
                    return None;
                }
            }

            x[i]       = need;
            decided[i] = true;
            for j2 in 0..n {
                if (buttons[i] >> j2) & 1 != 0 { remaining[j2] -= need; }
            }
            progress = true;
        }
    }

    if remaining.iter().all(|&r| r == 0) {
        Some(x.iter().sum::<i64>() as u64)
    } else {
        None
    }
}

fn solve_with_z3(target: &[i64], buttons: &[u64]) -> u64 {
    let n    = target.len();
    let m    = buttons.len();
    let opt  = z3::Optimize::new();
    let zero = z3::ast::Int::from_i64(0);

    let x: Vec<z3::ast::Int> = (0..m)
        .map(|i| z3::ast::Int::new_const(format!("x{i}")))
        .collect();

    for xi in &x { opt.assert(&xi.ge(&zero)); }

    for j in 0..n {
        let covering: Vec<z3::ast::Int> = (0..m)
            .filter(|&i| (buttons[i] >> j) & 1 != 0)
            .map(|i| x[i].clone())
            .collect();
        let lhs = if covering.is_empty() {
            zero.clone()
        } else {
            z3::ast::Int::add(&covering)
        };
        let rhs = z3::ast::Int::from_i64(target[j]);
        opt.assert(&lhs.eq(rhs));
    }

    let objective = z3::ast::Int::add(&x);
    opt.minimize(&objective);

    match opt.check(&[]) {
        z3::SatResult::Sat => {
            let model = opt.get_model().unwrap();
            model.eval(&objective, true).unwrap().as_u64().unwrap_or(0)
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXEMPLE: &str = include_str!("../../inputs/day_10_test.txt");

    #[test]
    fn test_partie1_exemple() {
        assert_eq!(Day10.partie1(EXEMPLE), "7");
    }

    #[test]
    fn test_partie2_exemple() {
        assert_eq!(Day10.partie2(EXEMPLE), "33");
    }
}
