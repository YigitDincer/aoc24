use std::collections::HashMap;

use itertools::Itertools;
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

fn is_producable(cat: &Catalogue, prod: &str) -> bool {
    if let Some(first_letter) = prod.chars().next() {
        if let Some(mats) = cat.get(&first_letter) {
            let mut flag = false;
            for mat in mats {
                if let Some(new_prod) = prod.strip_prefix(mat) {
                    flag |= is_producable(cat, new_prod);
                }
                if flag {
                    break;
                }
            }
            return flag;
        }
        return false;
    }

    return true;
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
            .filter(|&item| is_producable(&cat, *item))
            .count()
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

    #[test]
    fn test_is_producable() {
        let (towels, _) = parse(EXAMPLE);
        let catalogue = create_catalogue(&towels);

        assert!(is_producable(&catalogue, "brwrr"));
        assert!(is_producable(&catalogue, "bggr"));
        assert!(is_producable(&catalogue, "gbbr"));
        assert!(is_producable(&catalogue, "rrbgbr"));
        assert_eq!(is_producable(&catalogue, "ubwu"), false);
        assert!(is_producable(&catalogue, "bwurrg"));
        assert!(is_producable(&catalogue, "brgr"));
        assert_eq!(is_producable(&catalogue, "bbrgwb"), false);
    }

    #[test]
    fn test_solve_1() {
        let (towels, stacks) = parse(EXAMPLE);
        let catalogue = create_catalogue(&towels);

        assert_eq!(
            stacks
                .iter()
                .filter(|&item| is_producable(&catalogue, *item))
                .count(),
            6
        );
    }
}
