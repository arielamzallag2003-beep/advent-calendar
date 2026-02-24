use crate::days::Day;

pub struct Day01;

impl Day for Day01 {
    fn titre(&self) -> &'static str {
        "Template"
    }

    fn partie1(&self, _input: &str) -> String {
        "Non implemente".to_string()
    }

    fn partie2(&self, _input: &str) -> String {
        "Non implemente".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partie1() {
        let d = Day01;
        assert_eq!(d.partie1(""), "Non implemente");
    }

    #[test]
    fn test_partie2() {
        let d = Day01;
        assert_eq!(d.partie2(""), "Non implemente");
    }
}

