use core::fmt;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Start,
    End,
    NotEmpty,
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

fn print_grid(grid: &Vec<Elem>) {
    let mut grid_map = HashMap::new();
    for elem in grid {
        grid_map.insert((elem.row, elem.col), elem.tile);
    }

    for row in 0..70 {
        for col in 0..70 {
            let tile = grid_map.get(&(row, col)).unwrap_or(&Tile::NotEmpty);
            let symbol = match tile {
                Tile::Empty => ".",
                Tile::Start => "S",
                Tile::End => "E",
                Tile::NotEmpty => "#",
            };
            print!("{}", symbol);
        }
        println!();
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

    fn cheat(&self, grid: &Vec<Elem>) -> Vec<Elem> {
        grid.iter()
            .filter(|&pos| {
                let (row, col, _) = (pos.row, pos.col, pos.tile);

                ((row - self.row).abs() == 2 && (col - self.col == 0))
                    || ((row - self.row).abs() == 0 && (col - self.col).abs() == 2)
            })
            .cloned()
            .collect()
    }

    fn cheat_manhattan(&self, grid: &Vec<Elem>) -> Vec<Elem> {
        grid.iter()
            .filter(|&pos| {
                let (row, col, _) = (pos.row, pos.col, pos.tile);

                (row - self.row).abs() + (col - self.col).abs() <= 20
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

fn remove_finished_paths_calc_score(paths: &Vec<Vec<Elem>>) -> (usize, Vec<Elem>) {
    let mut paths = paths.clone();

    paths.retain(|path| path.last().unwrap().tile == Tile::End);

    let mut min_score = usize::MAX;

    let mut best_path = Vec::new();
    for path in &paths {
        min_score = std::cmp::min(min_score, path.len());
        if min_score == path.len() {
            best_path = path.clone();
        }
    }

    (min_score, best_path)
}

fn solve_1(grid: &Vec<Elem>) -> (usize, Vec<Elem>) {
    let mut possible_paths = Vec::new();

    let start = grid.iter().find(|&&a| a.tile == Tile::Start).unwrap();

    let mut path = Vec::new();
    path.push(start.clone());
    possible_paths.push(path);

    while !possible_paths.is_empty() {
        let mut current_paths = visit_next(grid, &mut possible_paths);
        let (_, best_path) = remove_finished_paths_calc_score(&current_paths);

        if best_path.len() > 0 {
            return (best_path.len() - 1, best_path);
        }
        current_paths.retain(|path| path.last().unwrap().tile != Tile::End);
        possible_paths = current_paths;
    }

    unreachable!()
}

fn do_sth(grid: &Vec<Elem>) -> usize {
    let (_, path) = solve_1(&grid);
    let mut without_cheat: HashMap<Elem, usize> = HashMap::new();

    let mut ctr = 0;
    for path_segment in path.clone() {
        without_cheat.insert(path_segment, ctr);
        ctr += 1;
    }

    let mut big_ctr = 0;

    let mut ctr = 0;
    for elem in path {
        let neigbors = elem.cheat(&grid);

        for neigbor in neigbors {
            if without_cheat.contains_key(&neigbor) {
                if *without_cheat.get(&neigbor).unwrap() >= ctr + 2 + 100 {
                    big_ctr += 1;
                }
            }
        }

        ctr += 1;
    }

    big_ctr
}

fn do_sth2(grid: &Vec<Elem>) -> usize {
    let (_, path) = solve_1(&grid);
    let mut without_cheat: HashMap<Elem, usize> = HashMap::new();

    let mut ctr = 0;
    for path_segment in path.clone() {
        without_cheat.insert(path_segment, ctr);
        ctr += 1;
    }

    let mut big_ctr = 0;

    let mut ctr = 0;

    for elem in path {
        let neigbors = elem.cheat_manhattan(&grid);

        for neigbor in neigbors {
            let dis =
                (neigbor.col - elem.col).abs() as usize + (neigbor.row - elem.row).abs() as usize;
            if without_cheat.contains_key(&neigbor) {
                if *without_cheat.get(&neigbor).unwrap() >= ctr + dis + 100 {
                    big_ctr += 1;
                }
            }
        }

        ctr += 1;
    }

    big_ctr
}

pub fn solve(input: &str) {
    println!("{}", do_sth(&parse(&input)));
    println!("{}", do_sth2(&parse(&input)));
}

#[cfg(test)]
mod tests {

    const EXAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    use super::*;

    #[test]
    fn test_solve_1() {
        assert_eq!(do_sth(&parse(EXAMPLE)), 0);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(do_sth2(&parse(EXAMPLE)), 0);
    }
}
