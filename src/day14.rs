use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Velocity {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    pos: Position,
    vel: Velocity,
}

impl Position {
    fn apply_restrictions(&mut self) {
        while self.x < 0 {
            self.x += 101; //width
        }

        while self.y < 0 {
            self.y += 103; //height
        }

        while self.x >= 101 {
            //width
            self.x -= 101;
        }

        while self.y >= 103 {
            //height
            self.y -= 103;
        }
    }

    fn get_neighbors(&self) -> HashSet<Position> {
        let mut neighbors = HashSet::new();
        neighbors.insert(Position {
            x: self.x - 1,
            y: self.y - 1,
        });
        neighbors.insert(Position {
            x: self.x,
            y: self.y - 1,
        });
        neighbors.insert(Position {
            x: self.x + 1,
            y: self.y - 1,
        });
        neighbors.insert(Position {
            x: self.x - 1,
            y: self.y,
        });
        neighbors.insert(Position {
            x: self.x + 1,
            y: self.y,
        });
        neighbors.insert(Position {
            x: self.x - 1,
            y: self.y + 1,
        });
        neighbors.insert(Position {
            x: self.x,
            y: self.y + 1,
        });
        neighbors.insert(Position {
            x: self.x + 1,
            y: self.y + 1,
        });
        neighbors
    }
}

impl Robot {
    fn move_it(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        self.pos.apply_restrictions()
    }

    fn quadrant(&self) -> usize {
        if self.pos.x < 101 / 2 && self.pos.y < 103 / 2 {
            return 0;
        }
        if self.pos.x > 101 / 2 && self.pos.y < 103 / 2 {
            return 1;
        }
        if self.pos.x < 101 / 2 && self.pos.y > 103 / 2 {
            return 2;
        }
        if self.pos.x > 101 / 2 && self.pos.y > 103 / 2 {
            return 3;
        }
        return 4;
    }
}

fn print(robots: &[Robot]) {
    for y in 0..103 {
        for x in 0..101 {
            if robots
                .iter()
                .any(|robot| robot.pos.x == x && robot.pos.y == y)
            {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            let left = left.strip_prefix("p=").unwrap();
            let (px, py) = left.split_once(',').unwrap();

            let right = right.strip_prefix("v=").unwrap();
            let (vx, vy) = right.split_once(',').unwrap();

            Robot {
                pos: Position {
                    x: px.parse().unwrap(),
                    y: py.parse().unwrap(),
                },
                vel: Velocity {
                    x: vx.parse().unwrap(),
                    y: vy.parse().unwrap(),
                },
            }
        })
        .collect()
}

fn count_quadrants(robots: &[Robot]) -> usize {
    robots
        .iter()
        .counts_by(Robot::quadrant)
        .iter()
        .filter(|&(key, _)| *key != 4)
        .map(|(_, frequency)| frequency)
        .product()
}

fn move_robots(robots: &mut [Robot], freq: usize) {
    for robot in robots {
        for _ in 0..freq {
            robot.move_it();
        }
    }
}

fn is_christmas_tree(robots: &[Robot]) -> bool {
    let robots_pos: HashSet<Position> = robots.iter().map(|robot| robot.pos).collect();

    let touching_robots_count = robots_pos
        .iter()
        .filter(|pos| pos.get_neighbors().intersection(&robots_pos).count() >= 2)
        .count();

    if touching_robots_count as f64 / robots_pos.len() as f64 >= 0.6 {
        return true;
    }

    return false;
}

fn form_christmas_tree(robots: &mut [Robot]) -> usize {
    let mut ctr = 0;

    while !is_christmas_tree(robots) {
        ctr += 1;
        for robot in &mut *robots {
            robot.move_it();
        }
    }

    print(robots);
    ctr
}

fn solve_1(robots: &[Robot]) -> usize {
    let mut robots = robots.to_vec();

    move_robots(&mut robots, 100);
    count_quadrants(&robots)
}

fn solve_2(robots: &[Robot]) -> usize {
    let mut robots_clone = robots.to_vec();

    form_christmas_tree(&mut robots_clone)
}

pub fn solve(input: &str) {
    let robots = parse(input);

    println!("{}", solve_1(&robots));
    println!("{}", solve_2(&robots));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    fn get_example() -> Vec<Robot> {
        vec![
            Robot {
                pos: Position { x: 0, y: 4 },
                vel: Velocity { x: 3, y: -3 },
            },
            Robot {
                pos: Position { x: 6, y: 3 },
                vel: Velocity { x: -1, y: -3 },
            },
            Robot {
                pos: Position { x: 10, y: 3 },
                vel: Velocity { x: -1, y: 2 },
            },
            Robot {
                pos: Position { x: 2, y: 0 },
                vel: Velocity { x: 2, y: -1 },
            },
            Robot {
                pos: Position { x: 0, y: 0 },
                vel: Velocity { x: 1, y: 3 },
            },
            Robot {
                pos: Position { x: 3, y: 0 },
                vel: Velocity { x: -2, y: -2 },
            },
            Robot {
                pos: Position { x: 7, y: 6 },
                vel: Velocity { x: -1, y: -3 },
            },
            Robot {
                pos: Position { x: 3, y: 0 },
                vel: Velocity { x: -1, y: -2 },
            },
            Robot {
                pos: Position { x: 9, y: 3 },
                vel: Velocity { x: 2, y: 3 },
            },
            Robot {
                pos: Position { x: 7, y: 3 },
                vel: Velocity { x: -1, y: 2 },
            },
            Robot {
                pos: Position { x: 2, y: 4 },
                vel: Velocity { x: 2, y: -3 },
            },
            Robot {
                pos: Position { x: 9, y: 5 },
                vel: Velocity { x: -3, y: -3 },
            },
        ]
    }

    #[test]
    fn parse_example() {
        assert_eq!(parse(EXAMPLE), get_example());
    }

    #[test]
    fn test_move() {
        let mut robot = Robot {
            pos: Position { x: 2, y: 4 },
            vel: Velocity { x: 2, y: -3 },
        };
        robot.move_it();

        assert_eq!(robot.pos, Position { x: 4, y: 1 });
        assert_eq!(robot.vel, Velocity { x: 2, y: -3 });
    }

    #[test]
    fn test_count_quadrants() {
        let mut robots = get_example();
        move_robots(&mut robots, 100);

        assert_eq!(count_quadrants(&robots), 12);
    }
}
