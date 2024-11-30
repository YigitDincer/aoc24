use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Locations {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Locations{
    fn sort_and_abs_diff(&self) -> u32 {
        let left_sorted = self.left.iter().sorted();
        let right_sorted = self.right.iter().sorted();
        std::iter::zip(left_sorted, right_sorted).map(|(&l, &r)| l.abs_diff(r)).sum()
    }

    fn calculate_similarity_score(&self) -> u32 {
        self.left.iter().map(|&x| self.calculate_frequency_in_right(x)*x).sum()
    }

    fn calculate_frequency_in_right(&self, value: u32) -> u32 {
        self.right.iter().filter(|&&x| x == value).count() as u32
    }
}


fn parse(input: &str) -> Locations {
    let (left, right): (Vec<_>, Vec<_>) = input.lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let first_number : u32 = parts.next().unwrap().parse().unwrap();
            let second_number : u32 = parts.next().unwrap().parse().unwrap();
            (first_number, second_number)
        })
        .unzip();

    Locations { left, right }
}


pub fn solve(input: &str) {
    println!("{}", parse(input).sort_and_abs_diff());
    println!("{}", parse(input).calculate_similarity_score());
}

#[cfg(test)]
mod tests {
    use super::*;

const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    fn example_locations() -> Locations {
        Locations {
            left: vec![3, 4, 2, 1, 3, 3],
            right: vec![4, 3, 5, 3, 9, 3],
        }
    }


    #[test]
    fn parse_example() {
        assert_eq!(parse(EXAMPLE), example_locations());
    }
    
    #[test]
    fn test_sort_and_abs_diff() {
        assert_eq!(example_locations().sort_and_abs_diff(), 11);
    }

    #[test]
    fn test_frequency_in_right() {
        assert_eq!(example_locations().calculate_frequency_in_right(3), 3);
    }

    #[test]
    fn test_similarity_score() {
        assert_eq!(example_locations().calculate_similarity_score(), 31);
    }
}
