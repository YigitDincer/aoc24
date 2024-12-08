use std::collections::{HashMap, HashSet};
use itertools::Itertools;

type Coordinate = (i64, i64);

#[derive(Debug, Clone, PartialEq, Eq)]
struct City {
    antenna: HashMap<Coordinate, char>,
    width: usize,
    height: usize,
}

impl City {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        let antenna = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c != '.')
                    .map(move |(x, c)| ((x as i64, y as i64), c))
            })
            .collect();

        City {
            antenna,
            width: lines[0].len(),
            height: lines.len(),
        }
    }
}

fn mark_antinodes(city: &City, range: impl IntoIterator<Item = i64> + Clone) -> HashSet<Coordinate> {
    city.antenna
        .iter()
        .permutations(2)
        .filter(|pair| {
            let [left, right] = &pair[..] else {unreachable!()};
            left.1 == right.1
        })
        .flat_map(|pair| {
            let (left, right) = (pair[0], pair[1]);
            let (left_x , left_y) = left.0;
            let (right_x, right_y) = right.0;

            let (delta_x, delta_y) = (right_x - left_x, right_y - left_y);

            range.clone().into_iter()
                .map(move |idx| {
                    let new_x = right_x + delta_x * idx;
                    let new_y = right_y + delta_y * idx;
                    (new_x, new_y)
                })
                .take_while(|&(x,y)| x >= 0 && y >= 0 && x < city.width as i64 && y < city.height as i64)
        })
        .collect()
}

pub fn solve(input: &str) {
    let city = City::new(input);

    println!("{}", mark_antinodes(&city, [1]).len());
    println!("{}", mark_antinodes(&city, 0..).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    fn example_locations() -> City {
        let mut grid: HashMap<Coordinate, char> = HashMap::new();
        grid.insert((8, 1), '0');
        grid.insert((5, 2), '0');
        grid.insert((7, 3), '0');
        grid.insert((4, 4), '0');
        grid.insert((6, 5), 'A');
        grid.insert((8, 8), 'A');
        grid.insert((9, 9), 'A');
        City {
            antenna: grid,
            width: 12,
            height: 12,
        }
    }

    #[test]
    fn test_parse() {
        assert_eq!(City::new(EXAMPLE), example_locations());
    }

    #[test]
    fn test_1() {
        assert_eq!(mark_antinodes(&City::new(EXAMPLE), 1..2).len(), 14);
    }

    #[test]
    fn test_2() {
        assert_eq!(mark_antinodes(&City::new(EXAMPLE), 0..12).len(), 34);
    }
}
