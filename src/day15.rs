use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn go_to(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Right => Position {
                row: self.row,
                col: self.col + 1,
            },
            Direction::Down => Position {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Position {
                row: self.row,
                col: self.col - 1,
            },
        }
    }
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

    fn set(&mut self, position: &Position, value: char) {
        self.data[position.row * self.width + position.col] = value;
    }

    fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                print!("{}", self.data[row * self.width + col]);
            }
            println!();
        }
    }
}

#[derive(Clone)]
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

impl Direction {
    fn is_vertical(self) -> bool {
        self == Direction::Up || self == Direction::Down
    }

    fn is_horizontal(self) -> bool {
        self == Direction::Left || self == Direction::Right
    }
}

fn move_boxes_in_direction(
    grid: &mut Grid,
    elem_pos: &Position,
    direction: Direction,
    elem_to_move: char,
) {
    let grid_original = grid.data.clone();

    let old_pos_idx = elem_pos.row * grid.width + elem_pos.col;

    let new_pos = elem_pos.go_to(direction);
    let new_pos_idx = new_pos.row * grid.width + new_pos.col;

    if grid.get(&new_pos).unwrap() == '.' {
        grid.data[new_pos_idx] = elem_to_move;
        grid.data[old_pos_idx] = '.';
    } else if grid.get(&new_pos).unwrap() == '#' {
        return;
    } else {
        let new_elem = grid.get(&new_pos).unwrap();
        move_boxes_in_direction(grid, &new_pos, direction, new_elem);

        if direction == Direction::Up || direction == Direction::Down {
            if new_elem == ']' {
                move_boxes_in_direction(
                    grid,
                    &Position {
                        row: new_pos.row,
                        col: new_pos.col - 1,
                    },
                    direction,
                    '[',
                );
            } else if new_elem == '[' {
                move_boxes_in_direction(
                    grid,
                    &Position {
                        row: new_pos.row,
                        col: new_pos.col + 1,
                    },
                    direction,
                    ']',
                );
            }
        }

        if grid.data != grid_original {
            move_boxes_in_direction(grid, &elem_pos, direction, elem_to_move);
        }
    }
}

fn apply_sequence(grid: &Grid, sequence: &str) -> Grid {
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
        move_boxes_in_direction(&mut grid, &robot_pos, direction, '@');
    }

    grid
}

fn is_wall(grid: &Grid, position: &Position) -> bool {
    grid.get(position).unwrap() == '#'
}

fn is_empty(grid: &Grid, position: &Position) -> bool {
    grid.get(position).unwrap() == '.'
}

fn get_robot_pos(grid: &Grid) -> Position {
    grid.data
        .iter()
        .position(|&c| c == '@')
        .map(|index| Position {
            row: index / grid.width,
            col: index % grid.width,
        })
        .unwrap()
}

fn can_it_move(
    grid: &Grid,
    initial_pos: &Position,
    direction: Direction,
    consider_these: Vec<Position>,
) -> bool {
    let mut is_ok = true;
    for pos in consider_these {
        let new_pos = pos.go_to(direction);

        if grid.get(&new_pos).unwrap() == '#' {
            is_ok = false;
            break;
        }

        if grid.get(&new_pos).unwrap() == '.' {
            continue;
        }

        let mut consider_these_new = Vec::new();

        if grid.get(&new_pos).unwrap() == ']' {
            consider_these_new.push(new_pos);
            consider_these_new.push(new_pos.go_to(Direction::Left));
        } else if grid.get(&new_pos).unwrap() == '[' {
            consider_these_new.push(new_pos);
            consider_these_new.push(new_pos.go_to(Direction::Right));
        }
        is_ok &= can_it_move(grid, &new_pos, direction, consider_these_new);
    }

    is_ok
}

fn move_robot(grid: &mut Grid, direction: Direction) {
    let robot_pos = get_robot_pos(grid);
    if direction == Direction::Left || direction == Direction::Right {
        move_boxes_in_direction(grid, &robot_pos, direction, '@');
        return;
    }

    // direction is Up OR Down
    let old_pos_idx = robot_pos.row * grid.width + robot_pos.col;

    let new_pos = robot_pos.go_to(direction);
    let new_pos_idx = new_pos.row * grid.width + new_pos.col;

    // move robot safely to new position if new position is empty
    if grid.data[new_pos_idx] == '.' {
        move_boxes_in_direction(grid, &robot_pos, direction, '@');
        return;
    } else if grid.data[new_pos_idx] == '#' {
        return;
    }

    if can_it_move(grid, &robot_pos, direction, vec![robot_pos].to_vec()) {
        move_boxes_in_direction(grid, &robot_pos, direction, '@');
    }
}

fn apply_sequence_for_extended_map(grid: &Grid, sequence: &str) -> Grid {
    let mut grid = grid.clone();

    for dir in sequence.chars() {
        let direction = match dir {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("Invalid direction in sequence"),
        };

        move_robot(&mut grid, direction)
    }

    grid
}

fn calculate_grid(grid: &Grid) -> usize {
    grid.data
        .iter()
        .enumerate()
        .filter(|(_, ch)| **ch == 'O' || **ch == '[')
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

fn extend(grid: &mut Grid) {
    grid.width *= 2;
    grid.data = grid
        .data
        .iter()
        .flat_map(|&ch| match ch {
            '#' => ['#', '#'],
            'O' => ['[', ']'],
            '.' => ['.', '.'],
            '@' => ['@', '.'],
            _ => unreachable!(),
        })
        .collect();

    grid.print();
}

fn solve_1(input: &Input) -> usize {
    let grid = apply_sequence(&input.map, &input.sequence);
    calculate_grid(&grid)
}

fn solve_2(input: &Input) -> usize {
    let mut input = input.clone();

    extend(&mut input.map);
    let grid = apply_sequence_for_extended_map(&input.map, &input.sequence);
    calculate_grid(&grid)
}

pub fn solve(input: &str) {
    let parsed = parse(input);
    println!("{}", solve_1(&parsed));
    println!("{}", solve_2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = "#######
#...O..
#......

<";

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

    const LARGE_EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########";

    const EXAMPLE2: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    const EXAMPLE2_EXTENDED: &str = "##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############

<vv<<^^<<^^";

    const EXAMPLE2_EXTENDED_END: &str = "##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############

<vv<<^^<<^^";

    const SMALL_EXAMPLE_EXTENDED: &str = "##########
##...[]...
##........

<";

    const LARGE_EXAMPLE_EXTENDED: &str = "####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const LARGE_EXAMPLE_EXTENDED_END: &str = "####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################";

    const CUSTOM: &str = "#########
#........
#..[]..#.
#.#...[].
#..[]....
#....@...
#........

^<>>>>^<<<v<>^^<>vv<v<<^>^>^^";

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
        let grid = apply_sequence(&parsed.map, &parsed.sequence);
        assert_eq!(calculate_grid(&grid), 2028);
    }

    #[test]
    fn test_calculate_grid() {
        let parsed = parse(SMALL_EXAMPLE);
        assert_eq!(calculate_grid(&parsed.map), 104);

        let parsed = parse(EXAMPLE_END);
        assert_eq!(calculate_grid(&parsed.map), 2028);

        let parsed = parse(SMALL_EXAMPLE_EXTENDED);
        assert_eq!(calculate_grid(&parsed.map), 105);

        let parsed = parse(LARGE_EXAMPLE_EXTENDED_END);
        assert_eq!(calculate_grid(&parsed.map), 9021);
    }

    #[test]
    fn test_extend() {
        let mut parsed = parse(LARGE_EXAMPLE);
        extend(&mut parsed.map);

        let parsed_extended = parse(LARGE_EXAMPLE_EXTENDED);
        assert_eq!(parsed.map, parsed_extended.map);
    }

    #[test]
    fn test_extended_move_robot() {
        let mut parsed = parse(EXAMPLE2);
        extend(&mut parsed.map);

        let parsed_extended = parse(EXAMPLE2_EXTENDED);
        assert_eq!(parsed.map, parsed_extended.map);

        let grid = apply_sequence_for_extended_map(&parsed_extended.map, &parsed.sequence);
        let parsed_extended_end = parse(EXAMPLE2_EXTENDED_END);
        assert_eq!(grid, parsed_extended_end.map);
    }

    #[test]
    fn test_large_extended_move_robot() {
        let parsed = parse(LARGE_EXAMPLE_EXTENDED);
        let grid = apply_sequence_for_extended_map(&parsed.map, &parsed.sequence);
        let parsed_extended_end = parse(LARGE_EXAMPLE_EXTENDED_END);
        assert_eq!(grid, parsed_extended_end.map);
    }

    #[test]
    fn test_custom() {
        let parsed = parse(CUSTOM);
        let grid = apply_sequence_for_extended_map(&parsed.map, &parsed.sequence);
        grid.print();

        assert!(false);
    }
}
