use std::{cmp::Ordering, collections::HashSet};

fn parse(input: &str) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let mut parts = input.split("\n\n");
    let first_part = parts.next().unwrap();
    let second_part = parts.next().unwrap();

    let set = first_part
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            let first_number: u32 = parts.next().unwrap().parse().unwrap();
            let second_number: u32 = parts.next().unwrap().parse().unwrap();
            (first_number, second_number)
        })
        .collect();

    let vec = second_part
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    (set, vec)
}

fn check_if_sorted(pages: &[u32], order: &HashSet<(u32, u32)>) -> bool {
    pages.is_sorted_by(|a, b| {
        if a == b {
            true
        } else { order.contains(&(*a, *b)) }
    })
}

fn solve_1(pages: &[Vec<u32>], order: &HashSet<(u32, u32)>) -> u32 {
    pages
        .iter()
        .filter(|x| check_if_sorted(x, order))
        .map(|x| x[x.len() / 2])
        .sum()
}

fn solve_2(pages: &[Vec<u32>], order: &HashSet<(u32, u32)>) -> u32 {
    pages
        .iter()
        .filter(|x| !check_if_sorted(x, order))
        .map(|x| {
            let mut x_clone = x.clone();
            x_clone.sort_by(|a, b| {
                if a == b {
                    Ordering::Equal
                } else if order.contains(&(*a, *b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            x_clone
        })
        .map(|x| x[x.len() / 2])
        .sum()
}

pub fn solve(input: &str) {
    let order = parse(input).0;
    let pages = parse(input).1;

    println!("{}", solve_1(&pages, &order));
    println!("{}", solve_2(&pages, &order));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    fn example_locations() -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
        let mut set = HashSet::new();
        set.insert((47, 53));
        set.insert((97, 13));
        set.insert((97, 61));
        set.insert((97, 47));
        set.insert((75, 29));
        set.insert((61, 13));
        set.insert((75, 53));
        set.insert((29, 13));
        set.insert((97, 29));
        set.insert((53, 29));
        set.insert((61, 53));
        set.insert((97, 53));
        set.insert((61, 29));
        set.insert((47, 13));
        set.insert((75, 47));
        set.insert((97, 75));
        set.insert((47, 61));
        set.insert((75, 61));
        set.insert((47, 29));
        set.insert((75, 13));
        set.insert((53, 13));

        let vec = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];

        (set, vec)
    }

    #[test]
    fn parse_example() {
        assert_eq!(parse(EXAMPLE), example_locations());
    }

    #[test]
    fn test_is_sorted() {
        let (order, pages) = example_locations();
        assert!(check_if_sorted(&pages[0], &order));
        assert!(check_if_sorted(&pages[1], &order));
        assert!(check_if_sorted(&pages[2], &order));
        assert!(!check_if_sorted(&pages[3], &order));
        assert!(!check_if_sorted(&pages[4], &order));
        assert!(!check_if_sorted(&pages[5], &order));
    }
}
