
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Operation {
    left: usize,
    right: usize
}


fn apply_operation(operations : &[Operation]) -> usize {
    operations.iter().map(|op| op.left * op.right).sum()
}

fn get_operations (input: &str) -> Vec<Operation> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            Operation {
                left: cap[1].parse().unwrap(),
                right: cap[2].parse().unwrap(),
            }
        })
        .collect()
}

fn clean_input (input: &str) -> String {
    let mut cleaned_input = input.to_string();
    cleaned_input.push_str("do()");

    let re = Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap();

    while let Some(mat) = re.find(&cleaned_input) {
        cleaned_input.replace_range(mat.range(), "");
    }

    cleaned_input
}

pub fn solve(input: &str) {
    println!("{}", apply_operation(&get_operations(input)));
    println!("{}", apply_operation(&get_operations(&clean_input(input))));
}

#[cfg(test)]
mod tests {
    use super::*;

const EXAMPLE: &str = "mul(832,177)";
const EXAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn parse_example() {
        assert_eq!(get_operations(EXAMPLE), vec![Operation{left : 832, right: 177}]);
    }
    
    #[test]
    fn test_apply_operation() {
        assert_eq!(apply_operation(&[Operation{left : 832, right: 177}]), 832*177);
    }

    #[test]
    fn test_clean() {
        assert_eq!(apply_operation(&get_operations(&clean_input("do()mul(832,177)"))), 832*177);
        assert_eq!(apply_operation(&get_operations(&clean_input("don't()mul(10,17)"))), 0);
        assert_eq!(apply_operation(&get_operations(&clean_input(EXAMPLE2))), 48);
    }
}
