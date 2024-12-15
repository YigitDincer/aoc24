#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        if position.row < self.height && position.col < self.width {
            Some(self.data[position.row * self.width + position.col])
        } else {
            None
        }
    }
}

struct Input {
    map: Grid,
    sequence: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn move_into(direction: Direction, position: &Position) -> Position {
    match direction {
        Direction::Up => Position {
            row: position.row - 1,
            col: position.col,
        },
        Direction::Right => Position {
            row: position.row,
            col: position.col + 1,
        },
        Direction::Down => Position {
            row: position.row + 1,
            col: position.col,
        },
        Direction::Left => Position {
            row: position.row,
            col: position.col - 1,
        },
    }
}

fn move_elem_into(grid: &mut Grid, elem_pos: &Position, direction: Direction, elem_to_move: char) {
    let grid_original = grid.data.clone();

    if elem_to_move == '.' {
        return;
    }

    let old_pos_idx = elem_pos.row * grid.width + elem_pos.col;

    let new_pos = move_into(direction, elem_pos);
    let new_pos_idx = new_pos.row * grid.width + new_pos.col;

    if grid.get(&new_pos).unwrap() == '.' {
        grid.data[new_pos_idx] = elem_to_move;
        grid.data[old_pos_idx] = '.';
    } else if grid.get(&new_pos).unwrap() == 'O' {
        move_elem_into(grid, &new_pos, direction, 'O');

        if grid.data != grid_original {
            move_elem_into(grid, &elem_pos, direction, elem_to_move);
        }
    } else {
        return;
    }
}

fn move_robot(grid: &Grid, sequence: &str) -> Grid {
    let mut grid = grid.clone();

    for dir in sequence.chars() {
        let robot_pos = grid
            .data
            .iter()
            .position(|&c| c == '@')
            .map(|index| Position {
                row: index / grid.width,
                col: index % grid.width,
            })
            .unwrap();

        let direction = match dir {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("Invalid direction in sequence"),
        };
        move_elem_into(&mut grid, &robot_pos, direction, '@');
    }

    grid
}

fn calculate_grid(grid: &Grid) -> usize {
    grid.data
        .iter()
        .enumerate()
        .filter(|(_, ch)| **ch == 'O')
        .map(|(index, _)| Position {
            row: index / grid.width,
            col: index % grid.width,
        })
        .map(|pos| (pos.row) * 100 + pos.col)
        .sum()
}

fn parse(input: &str) -> Input {
    let lines: Vec<&str> = input.lines().collect();
    let grid_lines: Vec<&str> = lines
        .iter()
        .take_while(|&&line| !line.is_empty())
        .cloned()
        .collect();
    let sequence_lines: Vec<&str> = lines
        .iter()
        .skip_while(|&&line| !line.is_empty())
        .skip(1)
        .cloned()
        .collect();
    let sequence = sequence_lines.join("");

    let grid_input = grid_lines.join("\n");
    let map = Grid::new(&grid_input);

    Input { map, sequence }
}

pub fn solve(input: &str) {
    let parsed = parse(input);
    let grid = move_robot(&parsed.map, &parsed.sequence);

    println!("{}", calculate_grid(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const EXAMPLE_END: &str = "########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########

<^^>>>vv<v>>v<<";

    const SMALL: &str = "#######
#...O..
#......

<";

    #[test]
    fn parse_example() {
        let parsed = parse(EXAMPLE);
        assert_eq!(parsed.map.width, 8);
        assert_eq!(parsed.map.height, 8);
        assert_eq!(parsed.sequence.len(), 15);
    }

    #[test]
    fn test_move_robot() {
        let parsed = parse(EXAMPLE);
        let grid = move_robot(&parsed.map, &parsed.sequence);
        assert_eq!(calculate_grid(&grid), 2028);
    }

    #[test]
    fn test_calculate_grid_small() {
        let parsed = parse(SMALL);
        dbg!(&parsed.map);
        dbg!(&parsed.map.width);
        dbg!(&parsed.map.height);
        assert_eq!(calculate_grid(&parsed.map), 104);
    }

    #[test]
    fn test_calculate_grid() {
        let parsed = parse(EXAMPLE_END);
        assert_eq!(calculate_grid(&parsed.map), 2028);
    }
}
