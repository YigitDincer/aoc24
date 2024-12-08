use std::collections::HashSet;
use std::iter::successors;
use itertools::Itertools;

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let data = lines.iter().flat_map(|line| line.chars()).collect();

        Grid {
            width,
            height,
            data,
        }
    }

    fn get(&self, row: Option<usize>, col: Option<usize>) -> Option<char> {
        if row? < self.height && col? < self.width {
            Some(self.data[row? * self.width + col?])
        } else {
            None
        }
    }

    fn find_starting_point(&self) -> (usize, usize) {
        (0..self.height)
            .cartesian_product(0..self.width)
            .find(|&(row, col)| self.get(Some(row), Some(col)).unwrap() == '^')
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotated_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    row: usize,
    col: usize,
    direction: Direction,
}

fn step(state: &State, grid: &Grid) -> Option<State> {
    let next_pos = match state.direction {
        Direction::Up => (state.row.checked_sub(1), Some(state.col)),
        Direction::Down => (state.row.checked_add(1), Some(state.col)),
        Direction::Left => (Some(state.row), state.col.checked_sub(1)),
        Direction::Right => (Some(state.row), state.col.checked_add(1)),
    };

    if let Some(c) = grid.get(next_pos.0, next_pos.1) {
        if c == '#' {
            let new_state = State {
                row: state.row,
                col: state.col,
                direction: state.direction.rotated_right(),
            };
            step(&new_state, grid)
        } else {
            Some(State {
                row: next_pos.0.unwrap(),
                col: next_pos.1.unwrap(),
                direction: state.direction,
            })
        }
    } else {
        None
    }
}

fn parse(input: &str) -> (State, Grid) {
    let grid = Grid::new(input);
    let (row, col) = grid.find_starting_point();
    let state = State {
        row,
        col,
        direction: Direction::Up,
    };

    (state, grid)
}

fn get_path(state: State, grid: &Grid) -> impl Iterator<Item = State> + use<'_> {
    successors(Some(state), move |state| step(state, grid))
}

fn count_distinct_positions(state: State, grid: &Grid) -> usize {
    let unique_pos: HashSet<_> = get_path(state, grid)
        .map(|state| (state.row, state.col))
        .collect();
    unique_pos.len()
}

fn is_loop(state: State, grid: &Grid) -> bool {
    let mut visited = HashSet::new();
    get_path(state, grid).any(|state| !visited.insert(state))
}

fn count_different_loops(state: State, grid: &Grid) -> usize {
    (0..grid.width * grid.height)
        .map(|idx| {
            let mut new_grid = grid.clone();
            new_grid.data[idx] = '#';
            new_grid
        })
        .filter(|grid| is_loop(state.clone(), grid))
        .count()
}

pub fn solve(input: &str) {
    let (state, grid) = parse(input);

    println!("{}", count_distinct_positions(state.clone(), &grid));
    println!("{}", count_different_loops(state.clone(), &grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_example() {
        let (state, grid) = parse(EXAMPLE);
        assert_eq!(count_distinct_positions(state.clone(), &grid), 41);
        assert_eq!(count_different_loops(state.clone(), &grid), 6);
    }

}
