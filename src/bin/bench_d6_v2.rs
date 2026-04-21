use std::fs;
use advent_calendar::d6::v2;

fn main() {
    let input = fs::read_to_string("inputs/day_06.txt")
        .unwrap_or_else(|e| panic!("Impossible de lire inputs/day_06.txt: {e}"));

    println!("{}", v2::partie1(&input));

    println!("{}", v2::partie2(&input));
}   