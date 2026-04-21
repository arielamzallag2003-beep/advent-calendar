use crate::days::Day;

fn find_highest(bank:&Vec<i32>, index:usize, nb_ToKeep:usize) -> (usize,i32) {
    let mut highest = bank[index];
    let mut new_index:usize = index;
    for i in index..bank.len() - nb_ToKeep {
        if bank[i] > highest {
            highest = bank[i];
            new_index = i;
        }
    }
    (new_index,highest)
}

pub struct Day03;

impl Day for Day03 {
    fn titre(&self) -> &'static str { "Lobby" }
    
    //trouver la combinaison de 2 chiffres (en respectant l'ordre) qui forme le nombre le plus élevé
    fn partie1(&self, _input: &str) -> String {
        let mut result = 0;
        for line in _input.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let bank:Vec<i32> = line.chars().map(|part| {
                part as i32 - '0' as i32
            }).collect();
            let mut index:usize = 0;
            let mut highest = 0;
            for i in 0..bank.len() - 1 {
                if bank[i] > highest {
                    index = i;
                    highest = bank[i];
                }
            }
            let mut scnd_highest = 0;
            for i in index+1..bank.len() {
                if bank[i] > scnd_highest {
                    scnd_highest = bank[i];
                }
            }
            result += highest * 10 + scnd_highest;
        }

        result.to_string()
    }
    
    //trouver la combinaison de 12 chiffres (en respectant l'ordre) qui forme le nombre le plus élevé
    fn partie2(&self, _input: &str) -> String {
        let mut result:i64 = 0;
        for line in _input.lines() {
            if line.trim().is_empty() {
                continue;
            }
            if line.len() <= 12 {
                let a = line.parse::<i64>().unwrap();
                result += a;
                continue;
            }
            let bank: Vec<i32> = line.chars().map(|part| {
                part as i32 - '0' as i32
            }).collect();
            let mut index = 0;
            for i in (0..12).rev() {
                let (new_index, highest) = find_highest(&bank, index, i);
                index = new_index + 1;
                result += highest as i64 * 10_i64.pow(i as u32);
            }
        }

        result.to_string()
    }
}


#[cfg(test)]
mod tests {
    use crate::d3::Day03;
    use super::*;
    const INPUT: &str = include_str!("../../inputs/day_03_test.txt");
    #[test]
    fn test_partie1() {
        let d = Day03;
        d.partie1(INPUT);
        assert_eq!(d.partie1(INPUT), "357");
    }

    #[test]
    fn test_partie2() {
        let d = Day03;
        assert_eq!(d.partie2(INPUT), "3121910778619");
    }
}