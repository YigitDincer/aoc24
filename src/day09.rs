use itertools::Itertools;

type Disk = Vec<i64>;

fn check_sum(disk: &[i64]) -> usize {
    disk.iter()
        .enumerate()
        .filter(|&(_, content)| *content != -1)
        .map(|(idx, content)| idx * (*content as usize))
        .sum()
}

fn expand(disk: &[u8]) -> Vec<i64> {
    disk.iter()
        .chain(b"0")
        .map(|ch| (ch - b'0') as usize)
        .tuples()
        .enumerate()
        .flat_map(|(idx, (file_len, free_space_len))| {
            std::iter::repeat_n(idx as i64, file_len).chain(std::iter::repeat_n(-1, free_space_len))
        })
        .collect()
}

fn move_disk(disk: &mut [u8]) {
    todo!()
}

pub fn solve(input: &str) {
    // let mut disk = input.to_string().into_bytes();
    // move_disk(&mut disk);

    // println!("{}", check_sum(&disk));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &[u8] = b"2333133121414131402";

    const DISK_IMAGE: &[i64] = &[
        0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5, 5,
        -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
    ];

    const MOVED_EXAMPLE: &[i64] = &[
        0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6,
    ];

    #[test]
    fn test_expand() {
        assert_eq!(expand(&EXAMPLE_INPUT), DISK_IMAGE);
    }

    #[test]
    fn test_check_sum() {
        assert_eq!(check_sum(&MOVED_EXAMPLE), 1928);
    }

    #[test]
    fn test_move() {
        // let mut example_disk = EXAMPLE_INPUT.to_vec();
        // move_disk(&mut example_disk);

        // assert_eq!(example_disk, MOVED_EXAMPLE);
    }
}
