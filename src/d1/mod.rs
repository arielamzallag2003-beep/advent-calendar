use crate::days::Day;

pub mod v1;
pub mod v2;
pub mod v3;

pub struct Day01;

impl Day for Day01 {
    fn titre(&self) -> &'static str {
        "Day 1"
    }

    fn partie1(&self, input: &str) -> String {
        v2::partie1(input)
    }

    fn partie2(&self, input: &str) -> String {
        v2::partie2(input)
    }
}