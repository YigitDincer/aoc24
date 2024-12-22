use itertools::Itertools;
use rayon::prelude::*;

fn prune(input: usize) -> usize {
    input % 16777216
}

fn mix(in1: usize, in2: usize) -> usize {
    in1 ^ in2
}

fn calc_2000th_secret(initial: usize) -> usize {
    let mut third = 0;

    let mut new_secret = initial;
    for _ in 0..2000 {
        let first = prune(mix(new_secret * 64, new_secret));
        let second = prune(mix(first / 32, first));
        third = prune(mix(second * 2048, second));
        new_secret = third;
    }
    third
}

fn calc_inflation(bananas: &Vec<usize>) -> Vec<i64> {
    bananas
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| (b as i64) - (a as i64))
        .collect()
}

fn get_bananas(initial: usize, secret_to_generate: usize) -> Vec<usize> {
    let mut bananas = Vec::new();
    let mut current_secret = initial;

    for _ in 0..secret_to_generate {
        bananas.push(current_secret % 10);
        let first = prune(mix(current_secret * 64, current_secret));
        let second = prune(mix(first / 32, first));
        let third = prune(mix(second * 2048, second));
        current_secret = third;
    }

    bananas
}

fn get_bananas_for_given_inflation(bananas: &Vec<usize>, inflation_given: &[i64; 4]) -> usize {
    let mut inflation = vec![0];
    inflation.extend(calc_inflation(bananas));

    let idx = inflation
        .iter()
        .enumerate()
        .skip(1)
        .tuple_windows()
        .find(|((_, &a), (_, &b), (_, &c), (_, &d))| [a, b, c, d] == *inflation_given)
        .map(|((_, &_), (_, &_), (_, &_), (idx, &_))| idx);

    if let Some(idx) = idx {
        return bananas[idx];
    } else {
        return 0;
    }
}

fn try_combinations(buyers: &Vec<usize>, price_change_count: usize) -> usize {
    let combos: Vec<_> = (-9..=9)
        .cartesian_product(-9..=9)
        .cartesian_product(-9..=9)
        .cartesian_product(-9..=9)
        .map(|(((a, b), c), d)| [a, b, c, d])
        .collect();

    //let mut max_banana = 0;
    //for combo in combos {
    //    let all_bananas = buyers.iter().map(|&buyer| get_bananas_for_given_inflation(&get_bananas(buyer, price_change_count), &combo)).sum();
    //    max_banana = std::cmp::max(all_bananas, max_banana);
    //}

    combos
        .par_iter()
        .map(|combo| {
            buyers
                .par_iter()
                .map(|&buyer| {
                    get_bananas_for_given_inflation(&get_bananas(buyer, price_change_count), &combo)
                })
                .sum()
        })
        .max()
        .unwrap()
}

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve(input: &str) {
    let buyers = parse(input);

    println!(
        "{}",
        buyers
            .iter()
            .map(|&buyer| calc_2000th_secret(buyer))
            .sum::<usize>()
    );

    println!("{}", try_combinations(&buyers, 2000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1
2
3
2024";

    const ONE_BUYER: &str = "123";

    #[test]
    fn test_solve_1() {
        let buyers = parse(EXAMPLE);
        assert_eq!(try_combinations(&buyers, 2000), 23);
    }

    #[test]
    fn test_banana() {
        let buyers = parse(ONE_BUYER);
        assert_eq!(
            get_bananas_for_given_inflation(&get_bananas(buyers[0], 10), &[-1, -1, 0, 2]),
            6
        );
    }
}
