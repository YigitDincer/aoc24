use itertools::{Either, Itertools};

#[derive(Clone, Debug)]
struct Key(Vec<usize>);

#[derive(Clone, Debug)]
struct Lock(Vec<usize>);

fn parse(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|schema_str| schema_str.lines().map(|a| a.to_owned()).collect())
        .collect()
}

fn count_things(schematic: &[String]) -> Vec<usize> {
    (0..5)
        .map(|col_idx| {
            schematic
                .iter()
                .filter(|line| line.as_bytes()[col_idx] == b'#')
                .count()
        })
        .collect()
}

fn get_schematics(schematics: &[Vec<String>]) -> (Vec<Key>, Vec<Lock>) {
    schematics.iter().partition_map(|schema| {
        if schema[0] == "#####" {
            Either::Left(Key(count_things(schema)))
        } else {
            Either::Right(Lock(count_things(schema)))
        }
    })
}

fn match_keys(keys: &[Key], locks: &[Lock]) -> usize {
    keys.iter()
        .cartesian_product(locks)
        .filter(|&(Key(key), Lock(lock))| {
            key.iter()
                .zip(lock)
                .map(|(left, right)| left + right)
                .max()
                .unwrap()
                < 8
        })
        .count()
}

pub fn solve(input: &str) {
    let (keys, locks) = get_schematics(&parse(input));
    println!("{}", match_keys(&keys, &locks));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_parse() {
        let input = parse(EXAMPLE);
        let (keys, locks) = get_schematics(&input);
        assert_eq!(match_keys(&keys, &locks), 3);
    }
}
