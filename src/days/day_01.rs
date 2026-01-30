use super::Day;

pub struct Day01;

impl Day for Day01 {
    fn day_number(&self) -> u8 {
        1
    }

    fn title(&self) -> &str {
        "Template - À implémenter"
    }

    fn solve_part1(&self, _input: &str) -> String {
        // TODO: Implémenter la solution partie 1
        "Non implémenté".to_string()
    }

    fn solve_part2(&self, _input: &str) -> String {
        // TODO: Implémenter la solution partie 2
        "Non implémenté".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let day = Day01;
        let input = "";
        assert_eq!(day.solve_part1(input), "Non implémenté");
    }

    #[test]
    fn test_part2() {
        let day = Day01;
        let input = "";
        assert_eq!(day.solve_part2(input), "Non implémenté");
    }
}
