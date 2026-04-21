use std::fs;
use advent_calendar::d1::v1;

fn main() {
    let input = fs::read_to_string("inputs/day_01.txt")
        .unwrap_or_else(|e| panic!("Impossible de lire inputs/day_01.txt: {e}"));

    println!("{}", v1::partie1(&input));

    println!("{}", v1::partie2(&input));
}   