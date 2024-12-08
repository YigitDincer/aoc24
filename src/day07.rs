use std::iter::repeat_n;
use rayon::prelude::*;

use itertools::Itertools;


#[derive(Debug, PartialEq)]
struct Equation {
    target: usize,
    operands: Vec<usize>,
}
#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat
}

fn parse(input: &str) -> Vec<Equation> {
    input.lines().map(|line| {
        let (target, right) = line.split_once(':').unwrap();
        let operands = right.split_whitespace().map(|x| x.parse().unwrap()).collect();
        Equation { target: target.parse().unwrap(), operands }
    }).collect()
}

fn apply(in1 : usize, in2 : usize, operator : Operator) -> usize
{
    match operator {
        Operator::Add => in1 + in2,
        Operator::Multiply => in1 * in2,
        Operator::Concat => 10usize.pow(in2.ilog10()+1)*in1+in2
    }
}

fn solve_equation(equation: &Equation, operators: Vec<Operator>) -> usize {
    operators.into_iter().enumerate().fold(equation.operands[0], |acc, (idx, operator)| apply(acc, equation.operands[idx+1], operator))
}

fn is_solvable(equation : &Equation, operators : &[Operator]) -> bool {
    repeat_n(operators.iter().copied(), equation.operands.len()-1).multi_cartesian_product().any(|operator_combination| solve_equation(equation, operator_combination) == equation.target)
}

fn solve_1(equations: &[Equation]) -> usize
{
    let operators = vec![Operator::Add, Operator::Multiply,];
    equations.iter().filter(|eq|is_solvable(eq, &operators)).map(|eq|eq.target).sum()
}

fn solve_2(equations: &[Equation]) -> usize
{
    let operators = vec![Operator::Add, Operator::Multiply, Operator::Concat];
    equations.par_iter().filter(|eq|is_solvable(eq, &operators)).map(|eq|eq.target).sum()
}

pub fn solve(input: &str) {
    println!("{}", solve_1(&parse(input)));
    println!("{}", solve_2(&parse(input)));
}


#[cfg(test)]
mod tests {
    use super::*;

const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    fn example_equations() -> Vec<Equation> {
        vec![
            Equation { target: 190, operands: vec![10, 19] },
            Equation { target: 3267, operands: vec![81, 40, 27] },
            Equation { target: 83, operands: vec![17, 5] },
            Equation { target: 156, operands: vec![15, 6] },
            Equation { target: 7290, operands: vec![6, 8, 6, 15] },
            Equation { target: 161011, operands: vec![16, 10, 13] },
            Equation { target: 192, operands: vec![17, 8, 14] },
            Equation { target: 21037, operands: vec![9, 7, 18, 13] },
            Equation { target: 292, operands: vec![11, 6, 16, 20] },
        ]
    }


    #[test]
    fn parse_example() {
        assert_eq!(parse(EXAMPLE), example_equations());
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(&parse(EXAMPLE)), 3749);
    }
    
    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(&parse(EXAMPLE)), 11387);
    }

}
