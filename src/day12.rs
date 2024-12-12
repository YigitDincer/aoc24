use std::collections::HashSet;

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

    fn get(&self, position: &Position) -> Option<char> {
        if (position.row as usize) < self.height && (position.col as usize) < self.width {
            Some(self.data[(position.row * self.width as i64 + position.col) as usize])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: i64,
    col: i64,
}

enum Direction {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

impl Position {
    fn get_direct_neighbors(&self) -> HashSet<Position> {
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

    fn get_neighbor(&self, direction: Direction) -> Position {
        match direction {
            Direction::NorthWest => Position {
                row: self.row - 1,
                col: self.col - 1,
            },
            Direction::North => Position {
                row: self.row - 1,
                col: self.col,
            },
            Direction::NorthEast => Position {
                row: self.row - 1,
                col: self.col + 1,
            },
            Direction::East => Position {
                row: self.row,
                col: self.col + 1,
            },
            Direction::SouthEast => Position {
                row: self.row + 1,
                col: self.col + 1,
            },
            Direction::South => Position {
                row: self.row + 1,
                col: self.col,
            },
            Direction::SouthWest => Position {
                row: self.row + 1,
                col: self.col - 1,
            },
            Direction::West => Position {
                row: self.row,
                col: self.col - 1,
            },
        }
    }
}

fn segment(grid: &Grid) -> Vec<HashSet<Position>> {
    let mut unvisited_pos: HashSet<_> = (0..grid.data.len())
        .map(|index| {
            let row = (index / grid.width) as i64;
            let col = (index % grid.width) as i64;

            Position { row, col }
        })
        .collect();

    let mut segments = Vec::new();

    while let Some(seed) = unvisited_pos.iter().next() {
        let mut segment = HashSet::new();
        let segment_name = grid.get(seed).unwrap();

        let mut todo = vec![*seed];
        while let Some(pos) = todo.pop() {
            if grid.get(&pos) == Some(segment_name) {
                todo.extend(
                    pos.get_direct_neighbors()
                        .into_iter()
                        .filter(|neighbor| !segment.contains(neighbor)),
                );
                segment.insert(pos);
                unvisited_pos.remove(&pos);
            }
        }

        segments.push(segment);
    }

    segments
}

fn calculate_perimeter(group: &HashSet<Position>) -> usize {
    group
        .iter()
        .map(|&pos| {
            4 - pos
                .get_direct_neighbors()
                .intersection(&group)
                .cloned()
                .count()
        })
        .sum()
}

fn total_price(grid: &Grid) -> usize {
    segment(grid)
        .iter()
        .map(|segment| segment.len() * calculate_perimeter(&segment))
        .sum()
}

fn count_corners(segment: &HashSet<Position>) -> usize {
    segment
        .iter()
        .map(|cur| {
            let mut sum = 0;

            if !segment.contains(&cur.get_neighbor(Direction::North))
                && !segment.contains(&cur.get_neighbor(Direction::West))
            {
                sum += 1;
            }

            if !segment.contains(&cur.get_neighbor(Direction::North))
                && !segment.contains(&cur.get_neighbor(Direction::East))
            {
                sum += 1;
            }

            if !segment.contains(&cur.get_neighbor(Direction::South))
                && !segment.contains(&cur.get_neighbor(Direction::West))
            {
                sum += 1;
            }

            if !segment.contains(&cur.get_neighbor(Direction::South))
                && !segment.contains(&cur.get_neighbor(Direction::East))
            {
                sum += 1;
            }

            if segment.contains(&cur.get_neighbor(Direction::North))
                && segment.contains(&cur.get_neighbor(Direction::West))
                && !segment.contains(&cur.get_neighbor(Direction::NorthWest))
            {
                sum += 1;
            }

            if segment.contains(&cur.get_neighbor(Direction::North))
                && segment.contains(&cur.get_neighbor(Direction::East))
                && !segment.contains(&cur.get_neighbor(Direction::NorthEast))
            {
                sum += 1;
            }

            if segment.contains(&cur.get_neighbor(Direction::South))
                && segment.contains(&cur.get_neighbor(Direction::West))
                && !segment.contains(&cur.get_neighbor(Direction::SouthWest))
            {
                sum += 1;
            }

            if segment.contains(&cur.get_neighbor(Direction::South))
                && segment.contains(&cur.get_neighbor(Direction::East))
                && !segment.contains(&cur.get_neighbor(Direction::SouthEast))
            {
                sum += 1;
            }
            sum
        })
        .sum()
}

fn total_price_bulk_discount(grid: &Grid) -> usize {
    segment(grid)
        .iter()
        .map(|segment| segment.len() * count_corners(&segment))
        .sum()
}

pub fn solve(input: &str) {
    let grid = Grid::new(input);
    println!("{}", total_price(&grid));
    println!("{}", total_price_bulk_discount(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "AAAA
BBCD
BBCC
EEEC";

    const EXAMPLE2: &str = "AAAA";

    #[test]
    fn test_example() {
        let grid = Grid::new(EXAMPLE);
        assert_eq!(total_price(&grid), 140);
    }

    #[test]
    fn test_segment() {
        let grid = Grid::new(EXAMPLE);
        assert_eq!(segment(&grid).len(), 5);
    }

    #[test]
    fn test_calculate_perimeter() {
        let grid = Grid::new(EXAMPLE2);
        let segment = segment(&grid);
        assert_eq!(calculate_perimeter(&segment.iter().next().unwrap()), 10);
    }

    #[test]
    fn test_bulk_discount() {
        let grid = Grid::new(EXAMPLE);
        assert_eq!(total_price_bulk_discount(&grid), 80);
    }
}
