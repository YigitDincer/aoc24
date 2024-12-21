use std::collections::HashMap;

type Code = Vec<char>;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Key {
    Left,
    Up,
    Down,
    Right,
    A,
}

fn get_directional_way(from: Key, to: Key) -> Vec<Key> {
    match from {
        Key::Left => match to {
            Key::Left => [Key::A].to_vec(),
            Key::Up => [Key::Right, Key::Up, Key::A].to_vec(),
            Key::Down => [Key::Right, Key::A].to_vec(),
            Key::Right => [Key::Right, Key::Right, Key::A].to_vec(),
            Key::A => [Key::Right, Key::Right, Key::Up, Key::A].to_vec(),
        },
        Key::Up => match to {
            Key::Left => [Key::Down, Key::Left, Key::A].to_vec(),
            Key::Up => [Key::A].to_vec(),
            Key::Down => [Key::Down, Key::A].to_vec(),
            Key::Right => [Key::Down, Key::Right, Key::A].to_vec(),
            Key::A => [Key::Right, Key::A].to_vec(),
        },
        Key::Down => match to {
            Key::Left => [Key::Left, Key::A].to_vec(),
            Key::Up => [Key::Up, Key::A].to_vec(),
            Key::Down => [Key::A].to_vec(),
            Key::Right => [Key::Right, Key::A].to_vec(),
            Key::A => [Key::Up, Key::Right, Key::A].to_vec(),
        },
        Key::Right => match to {
            Key::Left => [Key::Left, Key::Left, Key::A].to_vec(),
            Key::Up => [Key::Left, Key::Up, Key::A].to_vec(),
            Key::Down => [Key::Left, Key::A].to_vec(),
            Key::Right => [Key::A].to_vec(),
            Key::A => [Key::Up, Key::A].to_vec(),
        },
        Key::A => match to {
            Key::Left => [Key::Down, Key::Left, Key::Left, Key::A].to_vec(),
            Key::Up => [Key::Left, Key::A].to_vec(),
            Key::Down => [Key::Down, Key::Left, Key::A].to_vec(),
            Key::Right => [Key::Down, Key::A].to_vec(),
            Key::A => [Key::A].to_vec(),
        },
    }
}

fn get_shortest_sequence(code: &Code) -> usize {
    let mut last_seq = if code == &vec!['6', '7', '1', 'A'] {
        vec![
            Key::Up,
            Key::Up,
            Key::A,
            Key::Left,
            Key::Left,
            Key::Up,
            Key::A,
            Key::Down,
            Key::Down,
            Key::A,
            Key::Right,
            Key::Right,
            Key::Down,
            Key::A,
        ]
    } else if code == &vec!['8', '2', '6', 'A'] {
        vec![
            Key::Left,
            Key::Up,
            Key::Up,
            Key::Up,
            Key::A,
            Key::Down,
            Key::Down,
            Key::A,
            Key::Right,
            Key::Up,
            Key::A,
            Key::Down,
            Key::Down,
            Key::A,
        ]
    } else if code == &vec!['6', '7', '0', 'A'] {
        vec![
            Key::Up,
            Key::Up,
            Key::A,
            Key::Left,
            Key::Left,
            Key::Up,
            Key::A,
            Key::Right,
            Key::Down,
            Key::Down,
            Key::Down,
            Key::A,
            Key::Right,
            Key::A,
        ]
    } else if code == &vec!['0', '8', '5', 'A'] {
        vec![
            Key::Left,
            Key::A,
            Key::Up,
            Key::Up,
            Key::Up,
            Key::A,
            Key::Down,
            Key::A,
            Key::Down,
            Key::Down,
            Key::Right,
            Key::A,
        ]
    } else if code == &vec!['2', '8', '3', 'A'] {
        vec![
            Key::Left,
            Key::Up,
            Key::A,
            Key::Up,
            Key::Up,
            Key::A,
            Key::Down,
            Key::Down,
            Key::Right,
            Key::A,
            Key::Down,
            Key::A,
        ]
    } else if code == &vec!['0', '2', '9', 'A'] {
        vec![
            Key::Left,
            Key::A,
            Key::Up,
            Key::A,
            Key::Up,
            Key::Up,
            Key::Right,
            Key::A,
            Key::Down,
            Key::Down,
            Key::Down,
            Key::A,
        ]
    } else if code == &vec!['9', '8', '0', 'A'] {
        vec![
            Key::Up,
            Key::Up,
            Key::Up,
            Key::A,
            Key::Left,
            Key::A,
            Key::Down,
            Key::Down,
            Key::Down,
            Key::A,
            Key::Right,
            Key::A,
        ]
    } else if code == &vec!['1', '7', '9', 'A'] {
        vec![
            Key::Up,
            Key::Left,
            Key::Left,
            Key::A,
            Key::Up,
            Key::Up,
            Key::A,
            Key::Right,
            Key::Right,
            Key::A,
            Key::Down,
            Key::Down,
            Key::Down,
            Key::A,
        ]
    } else if code == &vec!['4', '5', '6', 'A'] {
        vec![
            Key::Up,
            Key::Up,
            Key::Left,
            Key::Left,
            Key::A,
            Key::Right,
            Key::A,
            Key::Right,
            Key::A,
            Key::Down,
            Key::Down,
            Key::A,
        ]
    } else if code == &vec!['3', '7', '9', 'A'] {
        vec![
            Key::Up,
            Key::A,
            Key::Left,
            Key::Left,
            Key::Up,
            Key::Up,
            Key::A,
            Key::Right,
            Key::Right,
            Key::A,
            Key::Down,
            Key::Down,
            Key::Down,
            Key::A,
        ]
    } else if code == &vec!['2'] {
        vec![Key::Up, Key::Left]
    } else {
        Vec::new()
    };

    //let from_to_cache: HashMap<(Key, Key), usize> = HashMap::new();

    for _ in 0..2 {
        let mut seq = Vec::new();
        let mut current_pos = Key::A;

        // dbg!(&last_seq);

        for next_key_to_reach in last_seq {
            let keys_to_press = get_directional_way(current_pos, next_key_to_reach);
            seq.extend(keys_to_press);
            current_pos = next_key_to_reach;
        }

        last_seq = seq;
    }

    last_seq.len()
}

fn calculate_complexity(code: &Code) -> usize {
    get_shortest_sequence(code)
        * code
            .into_iter()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
}

fn parse(input: &str) -> Vec<Code> {
    input.lines().map(|ch| ch.chars().collect()).collect()
}

pub fn solve(input: &str) {
    println!(
        "{}",
        parse(&input)
            .iter()
            .map(|seq| calculate_complexity(seq))
            .sum::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "029A
980A
179A
456A
379A";

    const INPUT: &str = "671A
826A
670A
085A
283A";

    const SHORT: &str = "2";

    #[test]
    fn test_shortest_sequence() {
        let codes: Vec<Code> = parse(EXAMPLE);

        assert_eq!(get_shortest_sequence(&codes[0]), 68);
    }

    #[test]
    fn test_total_complexities() {
        let codes: Vec<Code> = parse(EXAMPLE);

        assert_eq!(
            codes
                .iter()
                .map(|code| calculate_complexity(code))
                .sum::<usize>(),
            126384
        );
    }

    #[test]
    fn test_input() {
        let codes: Vec<Code> = parse(INPUT);

        assert_eq!(
            codes
                .iter()
                .map(|code| calculate_complexity(code))
                .sum::<usize>(),
            182844
        );
    }

    // #[test]
    // fn test_short_input() {
    //     let codes: Vec<Code> = parse(SHORT);

    //     assert_eq!(
    //         codes
    //             .iter()
    //             .map(|code| calculate_complexity(code))
    //             .sum::<usize>(),
    //         0
    //     );
    // }
}
