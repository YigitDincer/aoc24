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

fn get_shortest_sequence(code: &Code) -> Vec<Key> {
    let end_seq = if code == &vec!['6', '7', '1', 'A'] {
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
    } else {
        Vec::new()
    };

    // v<<A => <
    // v<A => v

    let mut seq = Vec::new();
    let mut current = Key::A;
    for sub_seq in end_seq {
        let keys_to_press = get_directional_way(current, sub_seq);
        seq.extend(keys_to_press);
        current = sub_seq;
    }

    current = Key::A;
    let mut whole_seq = Vec::new();
    for sub_seq in seq {
        let keys_to_press = get_directional_way(current, sub_seq);
        whole_seq.extend(keys_to_press);
        current = sub_seq;
    }

    whole_seq
}

fn calculate_complexity(code: &Code) -> usize {
    get_shortest_sequence(code).len()
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

    #[test]
    fn test_shortest_sequence() {
        let codes: Vec<Code> = parse(EXAMPLE);

        assert_eq!(get_shortest_sequence(&codes[0]).len(), 68);
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
}
