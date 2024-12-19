use core::fmt;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Start,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Elem {
    row: i64,
    col: i64,
    tile: Tile,
}

impl fmt::Display for Elem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Elem {
    fn get_direct_neighbors(&self, grid: &Vec<Elem>) -> Vec<Elem> {
        grid.iter()
            .filter(|&pos| {
                let (row, col, _) = (pos.row, pos.col, pos.tile);

                ((row - self.row).abs() == 1 && (col - self.col == 0))
                    || ((row - self.row).abs() == 0 && (col - self.col).abs() == 1)
            })
            .cloned()
            .collect()
    }
}

fn parse(input: &str) -> Vec<Elem> {
    let mut grid = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Empty,
                'S' => Tile::Start,
                'E' => Tile::End,
                _ => continue,
            };
            grid.push(Elem {
                row: row as i64,
                col: col as i64,
                tile,
            });
        }
    }
    grid
}

fn visit_next(grid: &Vec<Elem>, possible_paths: &Vec<Vec<Elem>>) -> Vec<Vec<Elem>> {
    let mut current_paths = Vec::new();

    for current_path in possible_paths {
        if current_path.last().unwrap().tile == Tile::End {
            continue;
        }

        let neigbors = current_path.last().unwrap().get_direct_neighbors(&grid);
        let unvisited_neigbors: Vec<_> = neigbors
            .into_iter()
            .filter(|neighbor| !current_path.contains(&neighbor))
            .collect();

        for neigbor in unvisited_neigbors {
            let mut new_path = current_path.clone();
            new_path.push(neigbor.clone());
            current_paths.push(new_path);
        }
    }

    current_paths
}

fn calc_score(path: &Vec<Elem>) -> (usize, usize) {
    let mut score = 0;

    let mut prev_orientation = 0;

    for window in path.windows(2) {
        if let [prev, next] = window {
            let row_diff = (next.row - prev.row).abs() as usize;
            let col_diff = (next.col - prev.col).abs() as usize;
            if col_diff == 1 {
                if prev_orientation == 0 {
                    score += 1;
                } else if prev_orientation == 1 {
                    score += 1001;
                }
                prev_orientation = 0;
            } else if row_diff == 1 {
                if prev_orientation == 1 {
                    score += 1;
                } else if prev_orientation == 0 {
                    score += 1001;
                }
                prev_orientation = 1;
            }
        }
    }

    (score, prev_orientation)
}

fn remove_inefficient_paths(
    cache: &mut HashMap<(Elem, usize), usize>,
    paths: &Vec<Vec<Elem>>,
) -> Vec<Vec<Elem>> {
    let mut next_paths = Vec::new();

    for path in paths.iter() {
        let current_pos = path.last().unwrap();

        let (new_score, last_orientation) = calc_score(path);

        if let Some(&score) = cache.get(&(*current_pos, last_orientation)) {
            if new_score <= score {
                cache.insert((current_pos.clone(), last_orientation), new_score);
                next_paths.push(path.clone());
                continue;
            }
        } else {
            cache.insert((current_pos.clone(), last_orientation), new_score);
            next_paths.push(path.clone());
        }
    }

    next_paths
}

fn solve_1(grid: &Vec<Elem>) -> (usize, usize) {
    let mut possible_paths = Vec::new();

    let start = grid.iter().find(|&&a| a.tile == Tile::Start).unwrap();

    let mut path = Vec::new();
    path.push(start.clone());
    possible_paths.push(path);

    let mut cache: HashMap<(Elem, usize), usize> = HashMap::new();

    let mut finished_paths = Vec::new();

    while !possible_paths.is_empty() {
        let mut next_paths = visit_next(grid, &mut possible_paths);

        finished_paths.extend(
            next_paths
                .iter()
                .filter(|&path| path.last().unwrap().tile == Tile::End)
                .map(|path| (calc_score(path), path.clone())),
        );

        next_paths = remove_inefficient_paths(&mut cache, &mut next_paths);

        next_paths.retain(|path| path.last().unwrap().tile != Tile::End);

        possible_paths = next_paths;
    }

    let minimum = finished_paths
        .iter()
        .map(|((score, _), _)| score)
        .min()
        .unwrap();

    let best_paths: Vec<_> = finished_paths
        .iter()
        .filter(|&((score, _), _)| score == minimum)
        .map(|((_, _), path)| path)
        .collect();

    (
        *minimum,
        best_paths
            .iter()
            .flat_map(|best_path| *best_path)
            .collect::<HashSet<&Elem>>()
            .len(),
    )
}

pub fn solve(input: &str) {
    //println!("{}", solve_1(&parse(input)).0);
    println!("{}", solve_1(&parse(input)).1);
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_solve() {
        assert_eq!(solve_1(&parse(EXAMPLE)).0, 7036);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve_1(&parse(EXAMPLE2)).0, 11048);
    }

    #[test]
    fn test_solve_tiles_on_best_path() {
        assert_eq!(solve_1(&parse(EXAMPLE)).1, 45);
    }

    #[test]
    fn test_solve_tiles_on_best_path2() {
        assert_eq!(solve_1(&parse(EXAMPLE2)).1, 64);
    }
}
