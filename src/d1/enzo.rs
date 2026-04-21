
use crate::days::{Day};

fn parse_instruction(line: &str) -> (char, i32) {
    let direction = line.chars().next().unwrap();
    let distance: i32 = line[1..].trim().parse::<i32>().unwrap();
    (direction, distance)
}

fn apply_rotation(position: i32, direction: char, distance: i32) -> (i32, i32) {
    let mut new_pos = match direction {
        'L' => position - distance,
        'R' => position + distance,
        _ => panic!("Direction inconnue : {}", direction),
    };
    let mut nbZero = new_pos.div_euclid(100).abs();
    new_pos = new_pos.rem_euclid(100);
    if distance > 0 && ((new_pos == 0 && direction == 'R') || (position == 0 && direction == 'L'))
    {
        nbZero -= 1;
    }
    (new_pos, nbZero)
}

pub struct Day01;

impl Day for Day01 {
    fn titre(&self) -> &'static str {
        "Secret Entrance"
    }


    //compter le nombre de fois où le pointeur indique 0 après chaque rotation
    fn partie1(&self, _input: &str) -> String {
        let mut position: i32 = 50;
        let mut zero_count: u32 = 0;

        for line in _input.lines()
        {
            if line.trim().is_empty() {
                continue;
            }
            let (direction, distance) = parse_instruction(line);
            let (newPos, nbZero) = apply_rotation(position, direction, distance);
            position = newPos;
            if position == 0 {
                zero_count += 1;
            }
        }
        zero_count.to_string()
    }

    //compter le nombre de fois où le pointeur passe ou s'arrête sur 0
    fn partie2(&self, _input: &str) -> String {
        let mut position: i32 = 50;
        let mut zero_count: u32 = 0;
        for line in _input.lines()
        {
            if line.trim().is_empty() {
                continue;
            }
            let (direction, distance) = parse_instruction(line);
            let (newPos, nbZero) = apply_rotation(position, direction, distance);
            position = newPos;

            zero_count = zero_count + nbZero as u32;
            if position == 0 {
                zero_count += 1;
            }
        }
        zero_count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../inputs/day_01_test.txt");
    #[test]
    fn test_partie1() {
        let d = Day01;
        d.partie1(INPUT);
        assert_eq!(d.partie1(INPUT), "3");
    }

    #[test]
    fn test_partie2() {
        let d = Day01;
        assert_eq!(d.partie2(INPUT), "6");
    }
}

