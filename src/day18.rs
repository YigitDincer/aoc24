use core::fmt;
use std::collections::HashMap;

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
    fn get_direct_neighbors(&self, all_parsepos: &Vec<Elem>) -> Vec<Elem> {
        all_parsepos
            .iter()
            .filter(|&pos| {
                let (row, col, _) = (pos.row, pos.col, pos.tile);

                ((row - self.row).abs() == 1 && (col - self.col == 0))
                    || ((row - self.row).abs() == 0 && (col - self.col).abs() == 1)
            })
            .cloned()
            .collect()
    }
}

fn parse(input: &str, lines_to_read: usize) -> Vec<Elem> {
    let mut grid = Vec::new();
    for row in 0..71 {
        for col in 0..71 {
            grid.push(Elem {
                row: row as i64,
                col: col as i64,
                tile: Tile::Empty,
            });
        }
    }

    let mut elements_to_remove = Vec::new();
    for line in input.lines().take(lines_to_read) {
        let parts: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
        elements_to_remove.push(Elem {
            col: parts[0],
            row: parts[1],
            tile: Tile::Empty,
        });
    }

    grid.retain(|elem| {
        !elements_to_remove
            .iter()
            .any(|e| e.row == elem.row && e.col == elem.col)
    });

    for elem in &mut grid {
        if elem.row == 0 && elem.col == 0 {
            elem.tile = Tile::Start;
            break;
        }
    }

    for elem in &mut grid {
        if elem.row == 70 && elem.col == 70 {
            elem.tile = Tile::End;
            break;
        }
    }

    //print_grid(&grid);
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

fn remove_finished_paths_calc_score(paths: &Vec<Vec<Elem>>) -> usize {
    let mut paths = paths.clone();

    paths.retain(|path| path.last().unwrap().tile == Tile::End);

    let mut min_score = usize::MAX;

    for path in &paths {
        min_score = std::cmp::min(min_score, path.len());
    }

    min_score
}

fn remove_inefficient_paths(cache: &mut HashMap<Elem, usize>, current_paths: &mut Vec<Vec<Elem>>) {
    let mut next_paths = Vec::new();

    for current_path in current_paths.iter() {
        let current_loc = current_path.last().unwrap();

        let new_score = current_path.len();

        if let Some(&value) = cache.get(current_loc) {
            if new_score < value {
                cache.insert(current_loc.clone(), new_score);
                next_paths.push(current_path.clone());
            }
        } else {
            cache.insert(current_loc.clone(), new_score);
            next_paths.push(current_path.clone());
        }
    }

    current_paths.clear();
    current_paths.extend(next_paths);
}

fn solve_1(grid: &Vec<Elem>) -> usize {
    let mut possible_paths = Vec::new();

    let start = grid.iter().find(|&&a| a.tile == Tile::Start).unwrap();

    let mut path = Vec::new();
    path.push(start.clone());
    possible_paths.push(path);

    let mut minimum = usize::MAX;

    let mut cache: HashMap<Elem, usize> = HashMap::new();

    while !possible_paths.is_empty() {
        let mut current_paths = visit_next(grid, &mut possible_paths);

        remove_inefficient_paths(&mut cache, &mut current_paths);

        let score = remove_finished_paths_calc_score(&current_paths);

        minimum = std::cmp::min(minimum, score);

        current_paths.retain(|path| path.last().unwrap().tile != Tile::End);

        possible_paths = current_paths;
    }

    minimum - 1
}

pub fn solve(input: &str) {
    for idx in 1024.. {
        if solve_1(&parse(input, idx)) == usize::MAX - 1 {
            println!("{idx}");
            break;
        }
    }

    //println!("{}", 0);
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1";

    #[test]
    fn test_solve() {
        let example = parse(EXAMPLE);
        assert_eq!(solve_1(&example), 22);
    }
}
