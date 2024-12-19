use std::collections::HashMap;

use regex::Regex;

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut parts = input.split("\n\n");
    let first_part = parts.next().unwrap().split(", ").collect::<Vec<&str>>();
    let second_part = parts.next().unwrap().split('\n').collect::<Vec<&str>>();
    (first_part, second_part)
}

type Catalogue = HashMap<char, Vec<String>>;

fn create_catalogue(towels: &Vec<&str>) -> Catalogue {
    let mut catalogue = Catalogue::new();

    for &towel in towels {
        let first_char = towel.chars().next().unwrap();
        catalogue
            .entry(first_char)
            .and_modify(|mats| mats.push(towel.to_string()))
            .or_insert_with(|| vec![towel.to_string()]);
    }

    catalogue
}

fn how_many_times_producable(
    cat: &Catalogue,
    stack: &str,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if cache.contains_key(stack) {
        return *cache.get(stack).unwrap();
    }

    if let Some(first_letter) = stack.chars().next() {
        if let Some(patterns) = cat.get(&first_letter) {
            let mut ctr = 0;

            for pattern in patterns {
                if let Some(remaining_pattern) = stack.strip_prefix(pattern) {
                    let so_many_times = how_many_times_producable(cat, remaining_pattern, cache);
                    ctr += so_many_times;
                    if so_many_times > 0 {
                        cache.insert(remaining_pattern.to_string(), so_many_times);
                    }
                }
            }

            cache.insert(stack.to_string(), ctr);
            return ctr;
        }
        cache.insert(stack.to_string(), 0);
        return 0;
    }

    cache.insert(stack.to_string(), 1);
    return 1;
}

// fn regexp_solution(towels: &Vec<&str>, stacks: &Vec<&str>) -> usize {
//     let regexp_pat = format!("^({})*$", towels.iter().join("|"));
//     let regexp = Regex::new(&regexp_pat).unwrap();
//     stacks.iter().filter(|pr| regexp.is_match(pr)).count()
// }

pub fn solve(input: &str) {
    let (towels, stacks) = parse(&input);
    let cat = create_catalogue(&towels);

    println!(
        "{}",
        stacks
            .iter()
            .filter(|&item| how_many_times_producable(&cat, item, &mut HashMap::new()) > 0)
            .count()
    );

    println!(
        "{}",
        stacks
            .iter()
            .map(|&item| how_many_times_producable(&cat, item, &mut HashMap::new()))
            .sum::<usize>()
    );

    //println!("{}", regexp_solution(&towels, &stacks));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    const EXAMPLE2: &str = "b, wu, bw, u

bwu";

    #[test]
    fn test_is_producable() {
        let (towels, _) = parse(EXAMPLE);
        let catalogue = create_catalogue(&towels);

        assert!(how_many_times_producable(&catalogue, "brwrr", &mut HashMap::new()) > 0);
        assert!(how_many_times_producable(&catalogue, "bggr", &mut HashMap::new()) > 0);
        assert!(how_many_times_producable(&catalogue, "gbbr", &mut HashMap::new()) > 0);
        assert!(how_many_times_producable(&catalogue, "rrbgbr", &mut HashMap::new()) > 0);
        assert_eq!(
            how_many_times_producable(&catalogue, "ubwu", &mut HashMap::new()),
            0
        );
        assert!(how_many_times_producable(&catalogue, "bwurrg", &mut HashMap::new()) > 0);
        assert!(how_many_times_producable(&catalogue, "brgr", &mut HashMap::new()) > 0);
        assert_eq!(
            how_many_times_producable(&catalogue, "bbrgwb", &mut HashMap::new()),
            0
        );
    }

    #[test]
    fn test_solve_2() {
        let (towels, stacks) = parse(EXAMPLE2);
        let catalogue = create_catalogue(&towels);

        assert_eq!(
            stacks
                .iter()
                .map(|&item| how_many_times_producable(&catalogue, item, &mut HashMap::new()))
                .sum::<usize>(),
            2
        );
    }

    #[test]
    fn test_solve_1() {
        let (towels, stacks) = parse(EXAMPLE);
        let catalogue = create_catalogue(&towels);

        assert_eq!(
            stacks
                .iter()
                .filter(
                    |&item| how_many_times_producable(&catalogue, item, &mut HashMap::new()) > 0
                )
                .count(),
            6
        );
    }

    #[test]
    fn test_solve_22() {
        let (towels, stacks) = parse(EXAMPLE);
        let catalogue = create_catalogue(&towels);

        assert_eq!(
            stacks
                .iter()
                .map(|&item| how_many_times_producable(&catalogue, item, &mut HashMap::new()))
                .sum::<usize>(),
            16
        );
    }
}
