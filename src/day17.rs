fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|a| a.parse().unwrap()).collect()
}

fn get_possible_inputs_for(possible_prev_inputs: &Vec<usize>, expected_end: usize) -> Vec<usize> {
    let mut new_possibilities = Vec::new();

    for A in possible_prev_inputs.clone() {
        for n in 0..8 as usize {
            if (((n + A) % 8) ^ ((A + n) / (2_usize.pow((((A + n) % 8) ^ 2) as u32))) ^ 5) % 8
                == expected_end
            {
                new_possibilities.push(A + n);
            }
        }
    }

    new_possibilities
}

fn find_original_A(nums: &Vec<usize>) -> usize {
    let mut input = vec![1].to_vec();
    let mut expected = nums.clone();
    expected.reverse();
    for level in 0..nums.len() {
        input = get_possible_inputs_for(&input, expected[level]);
        if level != nums.len() - 1 {
            input = input.iter().map(|a| a << 3).collect();
        }
    }
    *input.iter().min().unwrap()
}

fn run_program(register_A: usize) -> Vec<usize> {
    todo!()
}

pub fn solve(input: &str) {
    println!(
        "{}",
        find_original_A(&parse("2,4,1,2,7,5,1,7,4,4,0,3,5,5,3,0"))
    ); //1524746487980152 to high
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,4,1,2,7,5,1,7,4,4,0,3,5,5,3,0";
    const EXAMPLE: &str = "3,1,5,3,7,4,2,7,5";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(INPUT),
            vec![2, 4, 1, 2, 7, 5, 1, 7, 4, 4, 0, 3, 5, 5, 3, 0],
        );
    }

    // #[test]
    // fn test_input() {
    //     let input = parse(EXAMPLE);
    //     assert_eq!(find_original_A(&input)41644071]); // 010 011 110 110 111 000 000 100 111
    // }
}
