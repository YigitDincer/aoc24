use std::collections::HashSet;

struct Grid {
    width: usize,
    height: usize,
    data: Vec<usize>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let data = lines
            .iter()
            .flat_map(|line| line.chars())
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        Grid {
            width,
            height,
            data,
        }
    }

    fn get(&self, position: &Position) -> Option<usize> {
        if position.row < self.height && position.col < self.width {
            Some(self.data[position.row * self.width + position.col])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn get_direct_neighbors(&self) -> Vec<Position> {
        [
            (self.row.checked_sub(1), Some(self.col)),
            (self.row.checked_add(1), Some(self.col)),
            (Some(self.row), self.col.checked_sub(1)),
            (Some(self.row), self.col.checked_add(1)),
        ]
        .into_iter()
        .filter_map(|(row, col)| {
            Some(Position {
                row: row?,
                col: col?,
            })
        })
        .collect()
    }
}

fn trailheads_score(origin: &Position, grid: &Grid) -> usize {
    (1..=9).fold(HashSet::from([*origin]), |current_pos, next_level| {
        current_pos
            .into_iter()
            .flat_map(|pos| pos.get_direct_neighbors())
            .filter(|pos| grid.get(pos) == Some(next_level))
            .collect()
    }).len()
}

fn trailheads_rating(origin: &Position, grid: &Grid) -> usize {
    (1..=9).fold(Vec::from([*origin]), |current_pos, next_level| {
        current_pos
            .into_iter()
            .flat_map(|pos| pos.get_direct_neighbors())
            .filter(|pos| grid.get(pos) == Some(next_level))
            .collect()
    }).len()
}

fn sum_over_grid(grid: &Grid, f : fn(&Position, &Grid)-> usize) -> usize {
    grid.data
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == 0)
        .map(|(index, _)| {
            let row = index / grid.width;
            let col = index % grid.width;

            f(&Position { row, col }, grid)
        })
        .sum()
}

pub fn solve(input: &str) {
    let grid = Grid::new(&input);

    println!("{}", sum_over_grid(&grid, trailheads_score));
    println!("{}", sum_over_grid(&grid, trailheads_rating));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_example() {
        let grid = Grid::new(&EXAMPLE);
        assert_eq!(sum_over_grid(&grid, trailheads_score), 36);
        assert_eq!(sum_over_grid(&grid, trailheads_rating), 81);
    }
}
