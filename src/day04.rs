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
}

fn count_xmas(input: &str) -> usize {
    let grid = Grid::new(input);

    grid.data
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == 'X')
        .flat_map(|(index, _)| {
            let row = index / grid.width;
            let col = index % grid.width;

            [
                [
                    grid.get(Some(row), col.checked_add(1)),
                    grid.get(Some(row), col.checked_add(2)),
                    grid.get(Some(row), col.checked_add(3)),
                ],
                [
                    grid.get(row.checked_add(1), Some(col)),
                    grid.get(row.checked_add(2), Some(col)),
                    grid.get(row.checked_add(3), Some(col)),
                ],
                [
                    grid.get(row.checked_add(1), col.checked_add(1)),
                    grid.get(row.checked_add(2), col.checked_add(2)),
                    grid.get(row.checked_add(3), col.checked_add(3)),
                ],
                [
                    grid.get(row.checked_sub(1), col.checked_sub(1)),
                    grid.get(row.checked_sub(2), col.checked_sub(2)),
                    grid.get(row.checked_sub(3), col.checked_sub(3)),
                ],
                [
                    grid.get(Some(row), col.checked_sub(1)),
                    grid.get(Some(row), col.checked_sub(2)),
                    grid.get(Some(row), col.checked_sub(3)),
                ],
                [
                    grid.get(row.checked_sub(1), Some(col)),
                    grid.get(row.checked_sub(2), Some(col)),
                    grid.get(row.checked_sub(3), Some(col)),
                ],
                [
                    grid.get(row.checked_sub(1), col.checked_add(1)),
                    grid.get(row.checked_sub(2), col.checked_add(2)),
                    grid.get(row.checked_sub(3), col.checked_add(3)),
                ],
                [
                    grid.get(row.checked_add(1), col.checked_sub(1)),
                    grid.get(row.checked_add(2), col.checked_sub(2)),
                    grid.get(row.checked_add(3), col.checked_sub(3)),
                ],
            ]
        })
        .filter(|ray| ray == &[Some('M'), Some('A'), Some('S')])
        .count()
}

fn count_mas(input: &str) -> usize {
    let grid = Grid::new(input);

    grid.data
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == 'A')
        .flat_map(|(index, _)| {
            let row = index / grid.width;
            let col = index % grid.width;

            [
                [
                    grid.get(row.checked_sub(1), col.checked_sub(1)),
                    grid.get(row.checked_add(1), col.checked_add(1)),
                    grid.get(row.checked_sub(1), col.checked_add(1)),
                    grid.get(row.checked_add(1), col.checked_sub(1)),
                ],
                [
                    grid.get(row.checked_sub(1), col.checked_sub(1)),
                    grid.get(row.checked_add(1), col.checked_add(1)),
                    grid.get(row.checked_add(1), col.checked_sub(1)),
                    grid.get(row.checked_sub(1), col.checked_add(1)),
                ],
                [
                    grid.get(row.checked_add(1), col.checked_add(1)),
                    grid.get(row.checked_sub(1), col.checked_sub(1)),
                    grid.get(row.checked_add(1), col.checked_sub(1)),
                    grid.get(row.checked_sub(1), col.checked_add(1)),
                ],
                [
                    grid.get(row.checked_add(1), col.checked_add(1)),
                    grid.get(row.checked_sub(1), col.checked_sub(1)),
                    grid.get(row.checked_sub(1), col.checked_add(1)),
                    grid.get(row.checked_add(1), col.checked_sub(1)),
                ],
            ]
        })
        .filter(|x| x == &[Some('M'), Some('S'), Some('M'), Some('S')])
        .count()
}

pub fn solve(input: &str) {
    println!("{}", count_xmas(input));
    println!("{}", count_mas(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_parse() {
        let grid = Grid::new(EXAMPLE);
        assert_eq!(grid.get(Some(0), Some(0)), Some('M'));
        assert_eq!(grid.get(Some(9), Some(9)), Some('X'));
    }

    #[test]
    fn test_count_xmas() {
        assert_eq!(count_xmas(EXAMPLE), 18);
    }
}
