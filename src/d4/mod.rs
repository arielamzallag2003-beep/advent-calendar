use crate::days::Day;

pub struct Day04;

impl Day for Day04 {
    fn titre(&self) -> &'static str { "Printing Department" }

    fn partie1(&self, input: &str) -> String {
        let raw: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
        let rows = raw.len();
        if rows == 0 { return "0".to_string(); }
        let cols = raw[0].len();

        let mut grid = vec![false; rows * cols];
        for r in 0..rows {
            for c in 0..cols {
                if is_roll(raw[r][c]) { grid[r * cols + c] = true; }
            }
        }

        let mut count = 0usize;
        for r in 0..rows {
            let base = r * cols;
            for c in 0..cols {
                if grid[base + c] && neighbor_count_flat(&grid, r, c, rows, cols) < 4 {
                    count += 1;
                }
            }
        }
        count.to_string()
    }

    fn partie2(&self, input: &str) -> String {
        let raw: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
        let rows = raw.len();
        if rows == 0 { return "0".to_string(); }
        let cols = raw[0].len();

        let mut grid = vec![false; rows * cols];
        for r in 0..rows {
            for c in 0..cols {
                if is_roll(raw[r][c]) { grid[r * cols + c] = true; }
            }
        }

        let mut counts = vec![0u8; rows * cols];
        for r in 0..rows {
            for c in 0..cols {
                counts[r * cols + c] = neighbor_count_flat(&grid, r, c, rows, cols);
            }
        }

        let mut queue: Vec<(usize, usize)> = Vec::new();
        let mut queued = vec![false; rows * cols];
        for r in 0..rows {
            for c in 0..cols {
                let i = r * cols + c;
                if grid[i] && counts[i] < 4 {
                    queue.push((r, c));
                    queued[i] = true;
                }
            }
        }

        let mut total = 0usize;
        let mut qi = 0;

        while qi < queue.len() {
            let (r, c) = queue[qi];
            qi += 1;
            let i = r * cols + c;

            if !grid[i] { continue; }
            grid[i] = false;
            total += 1;

            for dr in -1i32..=1 {
                for dc in -1i32..=1 {
                    if dr == 0 && dc == 0 { continue; }
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 { continue; }
                    let ni = nr as usize * cols + nc as usize;
                    if counts[ni] > 0 { counts[ni] -= 1; }
                    if grid[ni] && counts[ni] < 4 && !queued[ni] {
                        queued[ni] = true;
                        queue.push((nr as usize, nc as usize));
                    }
                }
            }
        }

        total.to_string()
    }
}

#[inline]
fn is_roll(b: u8) -> bool { b == b'@' || b == b'x' }

fn neighbor_count_flat(grid: &[bool], r: usize, c: usize, rows: usize, cols: usize) -> u8 {
    let mut count = 0u8;
    for dr in -1i32..=1 {
        for dc in -1i32..=1 {
            if dr == 0 && dc == 0 { continue; }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                if grid[nr as usize * cols + nc as usize] { count += 1; }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXEMPLE: &str = include_str!("../../inputs/day_04_test.txt");

    #[test]
    fn test_partie1_exemple() {
        assert_eq!(Day04.partie1(EXEMPLE), "13");
    }

    #[test]
    fn test_partie2_exemple() {
        assert_eq!(Day04.partie2(EXEMPLE), "43");
    }
}
