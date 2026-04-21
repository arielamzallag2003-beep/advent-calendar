use crate::days::Day;

pub struct Day07;

impl Day for Day07 {
    fn titre(&self) -> &'static str { "Laboratories" }

    fn partie1(&self, input: &str) -> String {
        let (start_col, start_row, splitters, num_rows, num_cols) = parse_manifold(input);
        let stride = num_rows + 1;

        let mut activated = vec![false; num_cols * stride];
        let mut visited   = vec![false; num_cols * stride];
        let mut stack     = vec![(start_col, start_row)];
        let mut count     = 0usize;

        while let Some((col, row)) = stack.pop() {
            let vi = col * stride + row;
            if visited[vi] { continue; }
            visited[vi] = true;

            let pos = splitters[col].partition_point(|&r| r < row);
            if pos >= splitters[col].len() { continue; }
            let split_row = splitters[col][pos];

            let ai = col * stride + split_row;
            if !activated[ai] {
                activated[ai] = true;
                count += 1;
                if col > 0             { stack.push((col - 1, split_row + 1)); }
                if col + 1 < num_cols  { stack.push((col + 1, split_row + 1)); }
            }
        }

        count.to_string()
    }

    fn partie2(&self, input: &str) -> String {
        let (start_col, start_row, splitters, num_rows, num_cols) = parse_manifold(input);
        let stride = num_rows + 1;

        let mut memo = vec![u64::MAX; num_cols * stride];
        count_timelines(start_col, start_row, &splitters, num_cols, stride, &mut memo)
            .to_string()
    }
}

fn parse_manifold(input: &str) -> (usize, usize, Vec<Vec<usize>>, usize, usize) {
    let lines: Vec<&[u8]> = input.as_bytes()
        .split(|&b| b == b'\n')
        .map(|l| if l.last() == Some(&b'\r') { &l[..l.len() - 1] } else { l })
        .filter(|l| !l.is_empty())
        .collect();
    let num_rows = lines.len();
    let num_cols = lines.first().map_or(0, |r| r.len());

    let (start_row, start_col) = lines
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, &b)| (r, c, b)))
        .find(|&(_, _, b)| b == b'S')
        .map(|(r, c, _)| (r, c))
        .unwrap_or((0, 0));

    let mut splitters: Vec<Vec<usize>> = vec![vec![]; num_cols];
    for (r, row) in lines.iter().enumerate() {
        for (c, &b) in row.iter().enumerate() {
            if b == b'^' { splitters[c].push(r); }
        }
    }
    for s in &mut splitters { s.sort_unstable(); }

    (start_col, start_row, splitters, num_rows, num_cols)
}

fn count_timelines(
    col: usize,
    start_row: usize,
    splitters: &[Vec<usize>],
    num_cols: usize,
    stride: usize,
    memo: &mut Vec<u64>,
) -> u64 {
    let key = col * stride + start_row;
    if memo[key] != u64::MAX { return memo[key]; }

    let pos = splitters[col].partition_point(|&r| r < start_row);
    let result = if pos >= splitters[col].len() {
        1
    } else {
        let split_row = splitters[col][pos];
        let left  = if col > 0 { count_timelines(col - 1, split_row + 1, splitters, num_cols, stride, memo) } else { 1 };
        let right = if col + 1 < num_cols { count_timelines(col + 1, split_row + 1, splitters, num_cols, stride, memo) } else { 1 };
        left + right
    };

    memo[key] = result;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXEMPLE: &str = include_str!("../../inputs/day_07_test.txt");

    #[test]
    fn test_partie1_exemple() {
        assert_eq!(Day07.partie1(EXEMPLE), "21");
    }

    #[test]
    fn test_partie2_exemple() {
        assert_eq!(Day07.partie2(EXEMPLE), "40");
    }
}
