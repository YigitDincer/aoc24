use std::{collections::HashMap, mem::swap};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Operator {
    XOR,
    AND,
    OR,
}

#[derive(Debug, Clone)]
struct Operation {
    op1: (String, i64),
    op2: (String, i64),
    operator: Operator,
    result: (String, i64),
}

#[derive(Debug, Clone)]
struct Input {
    wires: HashMap<String, i64>,
    operations: Vec<Operation>,
}

fn substitute_variables(input: &mut Input) {
    let wires = &mut input.wires;

    for operation in &mut input.operations {
        if wires.contains_key(&operation.op1.0) {
            operation.op1.1 = *wires.get(&operation.op1.0).unwrap();
        } else {
            wires.insert(operation.op1.0.clone(), -1);
        }
        if wires.contains_key(&operation.op2.0) {
            operation.op2.1 = *wires.get(&operation.op2.0).unwrap();
        } else {
            wires.insert(operation.op2.0.clone(), -1);
        }

        if !wires.contains_key(&operation.result.0) {
            wires.insert(operation.result.0.clone(), -1);
        }
    }
}

fn apply_gate(op1: i64, op2: i64, operator: Operator) -> i64 {
    match operator {
        Operator::XOR => op1 ^ op2,
        Operator::AND => op1 & op2,
        Operator::OR => op1 | op2,
    }
}

fn apply_operations(input: &mut Input) {
    for operation in &mut input.operations {
        if operation.op1.1 != -1 && operation.op2.1 != -1 {
            operation.result.1 =
                apply_gate(operation.op1.1, operation.op2.1, operation.operator.clone());

            input
                .wires
                .insert(operation.result.0.clone(), operation.result.1);
        }
    }
}

fn calculate_z(input: &Input) -> i64 {
    (0..46).fold(0i64, |acc, idx| {
        acc + input.wires.get(&format!("z{idx:02}")).unwrap() * (2i64.pow(idx))
    })
}

fn calculate(input: Input) -> i64 {
    let mut input = input.clone();

    substitute_variables(&mut input);

    while input.wires.iter().any(|(_, b)| *b == -1) {
        substitute_variables(&mut input);
        apply_operations(&mut input);
    }

    calculate_z(&input)
}

fn parse_input(lines: &str) -> Input {
    let mut initial_values: HashMap<String, i64> = HashMap::new();
    let mut operations = Vec::new();

    for line in lines.lines() {
        if let Some((key, value)) = line.split_once(": ") {
            let value = value.parse().unwrap();
            initial_values.insert(key.trim().to_string(), value);
        } else if let Some((part1, result)) = line.split_once(" -> ") {
            let parts: Vec<&str> = part1.split_whitespace().collect();
            if parts.len() == 3 {
                let op = match parts[1] {
                    "XOR" => Operator::XOR,
                    "AND" => Operator::AND,
                    "OR" => Operator::OR,
                    _ => continue,
                };
                let input1 = parts[0].to_string();
                let input2 = parts[2].to_string();
                let output = result.trim().to_string();
                operations.push(Operation {
                    op1: (input1, -1),
                    op2: (input2, -1),
                    operator: op,
                    result: (output, -1),
                });
            }
        }
    }
    Input {
        wires: initial_values,
        operations,
    }
}

fn add_numbers_detect_error(input: &Input) -> usize {
    let mut input = input.clone();
    for (name, val) in &mut input.wires {
        *val = 0;
    }

    dbg!(calculate(input));

    3
}

pub fn solve(input: &str) {
    let input: Input = parse_input(input);
    //println!("{}", calculate(input));
    println!("{}", add_numbers_detect_error(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    const EXAMPLE2: &str = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";

    #[test]
    fn parse_example() {
        let input: Input = parse_input(EXAMPLE);
        assert_eq!(2024, calculate(input));
    }

    #[test]
    fn parse_example_part2() {
        add_numbers_detect_error(&parse_input(EXAMPLE2));
        assert!(false);
    }
}
