use crate::days::Day;

pub mod v1;
pub mod v2;

pub struct Day03;

impl Day for Day03 {
    fn titre(&self) -> &'static str {
        "Day 3"
    }

    fn partie1(&self, input: &str) -> String {
        v1::partie1(input)
    }

    fn partie2(&self, input: &str) -> String {
        v1::partie2(input)
    }
}