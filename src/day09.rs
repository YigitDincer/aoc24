use itertools::Itertools;

fn check_sum(disk: &[i64]) -> usize {
    disk.iter()
        .enumerate()
        .filter(|&(_, content)| *content != -1)
        .map(|(idx, content)| idx * (*content as usize))
        .sum()
}

fn expand(disk: &str) -> Vec<i64> {
    let mut disk_parsed: Vec<usize> = disk
        .chars()
        .map(|a| a.to_digit(10).unwrap() as usize)
        .collect();

    disk_parsed.push(0);

    disk_parsed
        .iter()
        .tuples()
        .enumerate()
        .flat_map(|(idx, (file_len, free_space_len))| {
            std::iter::repeat_n(idx as i64, *file_len)
                .chain(std::iter::repeat_n(-1 as i64, *free_space_len))
        })
        .collect()
}

fn move_disk(disk: &Vec<i64>) -> Vec<i64> {
    let mut disk_copy = disk.clone();

    let mut removed_elem_ctr = 0;
    let disk_length = disk_copy.len();

    for i in 0..disk_length {
        while *disk_copy.last().unwrap() == -1 {
            disk_copy.pop();
            removed_elem_ctr += 1;
        }

        if disk_copy[i] == -1 {
            disk_copy.swap_remove(i);
            removed_elem_ctr += 1;
        }

        if i + removed_elem_ctr + 1 == disk_length {
            break;
        }
    }

    disk_copy
}

fn fragment_disk(disk: &Vec<i64>) -> Vec<i64> {
    let mut disk_copy = disk.clone();

    let mut end_idx = disk_copy.len() - 1;
    let mut begin_idx: usize = 0;

    let mut file_length_ctr = 0;
    let mut last_file_id = 0;

    while end_idx > 0 {
        if file_length_ctr > 0 {
            while begin_idx < end_idx {
                while disk_copy[begin_idx] != -1 {
                    begin_idx += 1;
                    if begin_idx > end_idx {
                        break;
                    }
                }

                let mut free_space_ctr = 0;
                while disk_copy[begin_idx] == -1 {
                    free_space_ctr += 1;
                    begin_idx += 1; //begin_idx bos hanenin sagini gösteriyor
                    if begin_idx > end_idx {
                        break;
                    }
                }

                if free_space_ctr >= file_length_ctr {
                    for idx in 0..file_length_ctr {
                        disk_copy[begin_idx + idx - free_space_ctr] = last_file_id;
                        disk_copy[end_idx + idx + 1] = -1;
                    }
                    break;
                }
            }
        }

        while end_idx > 0 && disk_copy[end_idx] == -1 {
            end_idx -= 1;
        }

        last_file_id = disk_copy[end_idx];

        file_length_ctr = 0;

        while end_idx > 0 && disk_copy[end_idx] == last_file_id {
            end_idx -= 1; // en sonki dosyanin solunu gösteriyor
            file_length_ctr += 1;
        }

        begin_idx = 0;
    }

    disk_copy
}

pub fn solve(input: &str) {
    println!("{}", check_sum(&move_disk(&expand(input))));
    println!("{}", check_sum(&fragment_disk(&expand(input))));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2333133121414131402";

    fn get_disk_image() -> Vec<i64> {
        [
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5,
            5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ]
        .to_vec()
    }

    fn get_moved_disk() -> Vec<i64> {
        [
            0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6,
        ]
        .to_vec()
    }

    fn get_fragment_disk() -> Vec<i64> {
        [
            0, 0, 9, 9, 2, 1, 1, 1, 7, 7, 7, -1, 4, 4, -1, 3, 3, 3, -1, -1, -1, -1, 5, 5, 5, 5, -1,
            6, 6, 6, 6, -1, -1, -1, -1, -1, 8, 8, 8, 8, -1, -1,
        ]
        .to_vec()
    }

    #[test]
    fn test_expand() {
        assert_eq!(expand(&EXAMPLE_INPUT), get_disk_image());
    }

    #[test]
    fn test_check_sum() {
        assert_eq!(check_sum(&get_moved_disk()), 1928);
    }

    #[test]
    fn test_move() {
        assert_eq!(get_moved_disk(), move_disk(&get_disk_image()));
    }

    #[test]
    fn test_fragment() {
        assert_eq!(fragment_disk(&get_disk_image()), get_fragment_disk());
    }
}
