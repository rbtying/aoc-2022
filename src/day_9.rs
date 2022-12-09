// The day 9 prompt's examples are too long, see day_9_prompt.txt

use std::collections::HashSet;

const DEBUG: bool = false;

fn print(poses: &[(isize, isize)]) {
    if DEBUG {
        let min_x = poses.iter().map(|p| p.0).min().unwrap().min(0);
        let min_y = poses.iter().map(|p| p.1).min().unwrap().min(0);

        let max_x = poses.iter().map(|p| p.0).max().unwrap().max(0);
        let max_y = poses.iter().map(|p| p.1).max().unwrap().max(0);

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if (x, y) == poses[0] {
                    eprint!("H");
                } else if (x, y) == poses[1] && poses.len() == 2 {
                    eprint!("T");
                } else if let Some(idx) = poses.iter().position(|p| *p == (x, y)) {
                    eprint!("{}", idx + 1);
                } else if (x, y) == (0, 0) {
                    eprint!("s");
                } else {
                    eprint!(".");
                }
            }
            eprintln!();
        }
        eprintln!();
    }
}

fn advance(head_pos: (isize, isize), dir: &str) -> (isize, isize) {
    match dir {
        "L" => (head_pos.0 - 1, head_pos.1),
        "R" => (head_pos.0 + 1, head_pos.1),
        "U" => (head_pos.0, head_pos.1 + 1),
        "D" => (head_pos.0, head_pos.1 - 1),
        _ => unreachable!(),
    }
}

fn follow(head_pos: (isize, isize), mut tail_pos: (isize, isize)) -> (isize, isize) {
    let delta_x = head_pos.0 - tail_pos.0;
    let delta_y = head_pos.1 - tail_pos.1;

    if delta_x.abs() > 1 || delta_y.abs() > 1 {
        if delta_x != 0 {
            tail_pos.0 += delta_x / delta_x.abs();
        }
        if delta_y != 0 {
            tail_pos.1 += delta_y / delta_y.abs();
        }
    }

    tail_pos
}

pub fn part_1(input: &str) -> usize {
    let mut head_pos = (0isize, 0isize);
    let mut tail_pos = head_pos;

    let mut tail_poses = HashSet::new();

    for line in input.lines() {
        tail_poses.insert(tail_pos);

        let mut iter = line.split_ascii_whitespace();
        let dir = iter.next().unwrap();
        let dist = iter.next().unwrap().parse::<isize>().unwrap();

        for _ in 0..dist {
            head_pos = advance(head_pos, dir);
            tail_pos = follow(head_pos, tail_pos);
            tail_poses.insert(tail_pos);
            print(&[head_pos, tail_pos]);
        }
    }

    tail_poses.len()
}

pub fn part_2(input: &str) -> usize {
    let mut poses = [
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    let mut tail_poses = HashSet::new();

    for line in input.lines() {
        tail_poses.insert(poses[poses.len() - 1]);

        let mut iter = line.split_ascii_whitespace();
        let dir = iter.next().unwrap();
        let dist = iter.next().unwrap().parse::<isize>().unwrap();

        for _ in 0..dist {
            poses[0] = advance(poses[0], dir);

            for idx in 1..poses.len() {
                poses[idx] = follow(poses[idx - 1], poses[idx]);
            }
            tail_poses.insert(poses[poses.len() - 1]);
            print(&poses);
        }
    }

    tail_poses.len()
}

#[cfg(test)]
pub mod tests {
    use crate::day_9::{part_1, part_2};

    const INPUTS: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    #[test]
    pub fn test_day_9_example_part1() {
        assert_eq!(part_1(INPUTS), 13);
    }

    #[test]
    pub fn test_day_9_part1() {
        assert_eq!(part_1(include_str!("input/day_9.txt")), 6023);
    }

    #[test]
    pub fn test_day_9_example_part2() {
        assert_eq!(part_2(INPUTS), 1);
    }

    #[test]
    pub fn test_day_9_part2() {
        assert_eq!(part_2(include_str!("input/day_9.txt")), 2533);
    }
}
