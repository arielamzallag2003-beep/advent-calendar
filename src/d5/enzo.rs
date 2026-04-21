use crate::days::Day;
use std::collections::HashSet;

pub struct Day05;

impl Day for Day05 {
    fn titre(&self) -> &'static str { "Cafeteria" }
    
    //Trouver le nombre d'ingrédients (ID) encore frais parmi les ranges données (qui peuvent se chevaucher)
    fn partie1(&self, _input: &str) -> String {
        let mut ranges:Vec<(i64,i64)> = Vec::new();
        let mut ids:Vec<i64> = Vec::new();
        let mut is_range = true;
        let mut nb_good_id = 0;
        for line in _input.lines() {
            if line.trim().is_empty() {
                is_range = false;
                continue;
            }
            if is_range {
                let mut tuple = line.trim().split('-');
                let a = tuple.next().unwrap().parse::<i64>().unwrap();
                let b = tuple.next().unwrap().parse::<i64>().unwrap();
                ranges.push((a, b));
            }
            else {
                ids.push(line.trim().parse::<i64>().unwrap());
            }
        }
        for i in 0..ids.len() {
            for j in 0..ranges.len() {
                if ids[i] >= ranges[j].0 && ids[i] <= ranges[j].1 {
                    nb_good_id += 1;
                    break;
                }
            }
        }

        nb_good_id.to_string()
    }
    
    //compter tous les ingrédients unique encore frais parmi les ranges données (qui peuvent se chevaucher)
    fn partie2(&self, _input: &str) -> String {
        let mut ranges:Vec<(i64,i64)> = Vec::new();
        let mut nb_good_id = 0;
        //let mut set:HashSet<i64> = HashSet::new();

        for line in _input.lines() {
            if line.trim().is_empty() {
                break;
            }
            let mut tuple = line.trim().split('-');
            let a = tuple.next().unwrap().parse::<i64>().unwrap();
            let b = tuple.next().unwrap().parse::<i64>().unwrap();
            ranges.push((a, b));
        }
        /*for range in ranges {
            for i in range.0..=range.1 {
                set.insert(i);
            }
        }
        for i in set {
            nb_good_id += 1;
        }*/
        ranges.sort_by_key(|r| r.0);

        let mut current_end: i64 = i64::MIN;

        for (start, end) in &ranges {
            let start = (*start).max(current_end + 1);
            if start <= *end {
                nb_good_id += end - start + 1;
            }
            current_end = current_end.max(*end);
        }

        nb_good_id.to_string()
    }
}


#[cfg(test)]
mod tests {
    use crate::d5::Day05;
    use super::*;
    const INPUT: &str = include_str!("../../inputs/day_05_test.txt");
    #[test]
    fn test_partie1() {
        let d = Day05;
        d.partie1(INPUT);
        assert_eq!(d.partie1(INPUT), "3");
    }

    #[test]
    fn test_partie2() {
        let d = Day05;
        assert_eq!(d.partie2(INPUT), "14");
    }
}