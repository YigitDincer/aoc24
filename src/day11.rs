use std::collections::HashMap;

use rayon::prelude::*;

fn apply_rule(current_stone: usize) -> Vec<usize> {
    let str = current_stone.to_string();
    let str_len = str.chars().count();

    if current_stone == 0 {
        return [1].to_vec();
    } else if str_len % 2 == 0 {
        let mid = str_len / 2;
        let (first_half, second_half) = str.split_at(mid);
        return [first_half.parse().unwrap(), second_half.parse().unwrap()].to_vec();
    } else {
        return [current_stone * 2024].to_vec();
    }
}

fn next_generation_cached(
    current_num: usize,
    remaining_iterations: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if cache.contains_key(&(current_num, remaining_iterations)) {
        *cache.get(&(current_num, remaining_iterations)).unwrap()
    } else {
        let new_val = if remaining_iterations == 0 {
            1
        } else {
            let new_nums = apply_rule(current_num);
            if new_nums.len() == 1 {
                next_generation_cached(new_nums[0], remaining_iterations - 1, cache)
            } else {
                next_generation_cached(new_nums[0], remaining_iterations - 1, cache)
                    + next_generation_cached(new_nums[1], remaining_iterations - 1, cache)
            }
        };

        cache.insert((current_num, remaining_iterations), new_val);
        new_val
    }
}



fn manipulate_stone(initial_state: &Vec<usize>, iteration_count : usize) -> usize {
    let initial_state = initial_state.clone();
    initial_state
        .par_iter()
        .map(|&stone| next_generation_cached(stone, iteration_count, &mut HashMap::new()))
        .sum()
}


pub fn solve(_input: &str) {
    let v = vec![2, 54, 992917, 5270417, 2514, 28561, 0, 990];
    println!("{}", manipulate_stone(&v, 25));
    println!("{}", manipulate_stone(&v, 75));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example() -> Vec<usize> {
        vec![125, 17]
    }

    #[test]
    fn test_get_stone_state() {
        assert_eq!(manipulate_stone(&get_example(), 25), 55312);
    }

}
