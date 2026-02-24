use crate::days::Day;

pub struct Day01;

impl Day for Day01 {
    fn titre(&self) -> &'static str {
        "Secret Entrance"
    }

    fn partie1(&self, input: &str) -> String {
        let mut position: i32 = 50;
        let mut count: u32 = 0;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let (dir, dist_str) = line.split_at(1);
            let Ok(dist) = dist_str.parse::<i32>() else {
                continue;
            };

            position = match dir {
                "L" => (position - dist).rem_euclid(100),
                "R" => (position + dist).rem_euclid(100),
                _ => continue,
            };

            if position == 0 {
                count += 1;
            }
        }

        count.to_string()
    }

    fn partie2(&self, input: &str) -> String {
        let mut position: i32 = 50;
        let mut count: u64 = 0;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let (dir, dist_str) = line.split_at(1);
            let Ok(dist) = dist_str.parse::<i32>() else {
                continue;
            };

            let first_hit = match dir {
                "L" => if position == 0 { 100 } else { position },
                "R" => if position == 0 { 100 } else { 100 - position },
                _ => continue,
            };

            if dist >= first_hit {
                count += ((dist - first_hit) / 100 + 1) as u64;
            }

            position = match dir {
                "L" => (position - dist).rem_euclid(100),
                "R" => (position + dist).rem_euclid(100),
                _ => continue,
            };
        }

        count.to_string()
    }
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
