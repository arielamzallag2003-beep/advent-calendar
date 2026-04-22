use crate::days::Day;

fn parse_pairs(input: &str) -> Vec<(i64, i64)> {
    input.trim().split(',').map(|pair| {
            let mut parts = pair.split('-');
            let a = parts.next().unwrap().parse::<i64>().unwrap();
            let b = parts.next().unwrap().parse::<i64>().unwrap();
            (a, b)
        }).collect()
}

fn is_repeated1(n: i64) -> bool {
    let s = n.to_string();
    let mid = s.len() / 2;
    s.len() % 2 == 0 && s[..mid] == s[mid..]
}

fn is_repeated2(n: i64) -> bool {
    let s = n.to_string();
    for pattern_len in 1..=s.len() / 2 {
        if s.len() % pattern_len != 0 {
            continue;
        }
        let pattern = &s[..pattern_len];
        let all_match = s
            .as_bytes()
            .chunks(pattern_len)
            .all(|chunk| chunk == pattern.as_bytes());
        if all_match {
            return true;
        }
    }
    false
}

pub struct Day02;

impl Day for Day02 {
    fn titre(&self) -> &'static str { "Gift Shop" }
    
    //compter le nombre d'ID à l'intérieur de chaque range qui est constitué de deux sous-séquence identique
    fn partie1(&self, _input: &str) -> String {
        let mut result:i64 = 0;
        let pairs = parse_pairs(_input);
        for (a, b) in &pairs {
            for i in *a..=*b
            {
                if is_repeated1(i)
                {
                    result += i;
                }
            }
        }

        result.to_string()
    }


    //compter le nombre d'ID à l'intérieur de chaque range qui est constitué de deux sous-séquence ou plus identique
    fn partie2(&self, _input: &str) -> String {
        let mut result:i64 = 0;
        let pairs = parse_pairs(_input);
        for (a, b) in &pairs {
            for i in *a..=*b
            {
                if is_repeated2(i)
                {
                    result += i;
                }
            }
        }

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::d2::Day02;
    use super::*;
    const INPUT: &str = include_str!("../../inputs/day_02_test.txt");
    #[test]
    fn test_partie1() {
        let d = Day02;
        d.partie1(INPUT);
        assert_eq!(d.partie1(INPUT), "1227775554");
    }

    #[test]
    fn test_partie2() {
        let d = Day02;
        assert_eq!(d.partie2(INPUT), "4174379265");
    }
}