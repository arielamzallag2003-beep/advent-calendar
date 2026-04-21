use std::fs;
use advent_calendar::d3::v1;

fn main() {
    let input = fs::read_to_string("inputs/day_03.txt")
        .unwrap_or_else(|e| panic!("Impossible de lire inputs/day_03.txt: {e}"));

    println!("{}", v1::partie1(&input));

    println!("{}", v1::partie2(&input));
}   