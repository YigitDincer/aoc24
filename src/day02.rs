
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn get_adj_difference(&self) -> Vec<u32> {
        self.levels.iter().tuple_windows().map(|(a, b)| a.abs_diff(*b) ).collect()
    }

    fn is_strictly_monoton(&self) -> bool {
        self.levels.iter().tuple_windows().all(|(a, b)| a < b) || 
        self.levels.iter().tuple_windows().all(|(a, b)| a > b)
    }

    fn is_safe(&self) -> bool {
        self.is_strictly_monoton() && 
        self.get_adj_difference().iter().all(|&x| (1..=3).contains(&x))
    }

    fn is_safe_more_tolerant(&self) -> bool {
        (0..self.levels.len()).any(|index_to_ignore|{
            let (left, right) = self.levels.split_at(index_to_ignore);
            let levels = left.iter().chain(right.iter().skip(1)).copied().collect();
            (Report { levels }).is_safe()
        })
    }
}

fn solve_1(reports : Vec<Report>) -> u32 {
    // count the number of reports that are safe
    reports.iter().filter(|report| report.is_safe()).count() as u32
}

fn solve_2(reports : Vec<Report>) -> u32 {
    // count again, but be more tolerant
    reports.iter().filter(|report| report.is_safe_more_tolerant()).count() as u32

}

fn parse(input: &str) -> Vec<Report> {
    input.lines().map(|line| {
        Report {
            levels: line.split_whitespace().map(|x| x.parse().unwrap()).collect()
        }
    }).collect()
}


pub fn solve(input: &str) {
    println!("{}", solve_1(parse(input)));
    println!("{}", solve_2(parse(input)));
}


#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;


const EXAMPLE : &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_parse() {
        let expected = vec![
            Report { levels: vec![7, 6, 4, 2, 1] },
            Report { levels: vec![1, 2, 7, 8, 9] },
            Report { levels: vec![9, 7, 6, 2, 1] },
            Report { levels: vec![1, 3, 2, 4, 5] },
            Report { levels: vec![8, 6, 4, 4, 1] },
            Report { levels: vec![1, 3, 6, 7, 9] },
        ];
        assert_eq!(parse(EXAMPLE), expected);
    }

    #[test]
    fn test_is_safe() {
        assert!(parse(EXAMPLE)[0].is_safe());
        assert!(!parse(EXAMPLE)[1].is_safe());
        assert!(!parse(EXAMPLE)[2].is_safe());
        assert!(!parse(EXAMPLE)[3].is_safe());
        assert!(!parse(EXAMPLE)[4].is_safe());
        assert!(parse(EXAMPLE)[5].is_safe());
    }

    #[test]
    fn test_get_adj_difference() {
        assert_eq!(parse(EXAMPLE)[0].get_adj_difference(), vec![1, 2, 2, 1]);
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(parse(EXAMPLE)), 2);
    }

    #[test]
    fn test_solve_2() {
        assert!(parse(EXAMPLE)[0].is_safe_more_tolerant());
        assert!(!parse(EXAMPLE)[1].is_safe_more_tolerant());
        assert!(!parse(EXAMPLE)[2].is_safe_more_tolerant());
        assert!(parse(EXAMPLE)[3].is_safe_more_tolerant());
        assert!(parse(EXAMPLE)[4].is_safe_more_tolerant());
        assert!(parse(EXAMPLE)[5].is_safe_more_tolerant());
    }

}
