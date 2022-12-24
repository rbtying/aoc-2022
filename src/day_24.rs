// See day_24_prompt.txt

use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct P {
    row: isize,
    col: isize,
}

impl P {
    fn wrap(self, limits: P) -> P {
        P {
            row: self.row.rem_euclid(limits.row),
            col: self.col.rem_euclid(limits.col),
        }
    }
}

impl std::ops::Add for P {
    type Output = P;
    fn add(self, rhs: Self) -> Self::Output {
        P {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl std::ops::Mul for P {
    type Output = P;
    fn mul(self, rhs: Self) -> Self::Output {
        P {
            row: self.row * rhs.row,
            col: self.col * rhs.col,
        }
    }
}

impl From<(usize, usize)> for P {
    fn from(x: (usize, usize)) -> Self {
        P {
            row: x.0 as isize,
            col: x.1 as isize,
        }
    }
}

impl From<(isize, isize)> for P {
    fn from(x: (isize, isize)) -> Self {
        P { row: x.0, col: x.1 }
    }
}

const R: P = P { row: 0, col: 1 };
const L: P = P { row: 0, col: -1 };
const U: P = P { row: -1, col: 0 };
const D: P = P { row: 1, col: 0 };
const NOOP: P = P { row: 0, col: 0 };

fn parse(input: &str) -> (HashSet<P>, HashSet<(P, P)>, P, P, P) {
    let mut walls = HashSet::new();
    let mut blizzards = HashSet::new();

    let mut start = None;
    let mut end = None;

    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            let p = P::from((r as isize - 1, c as isize - 1));
            match ch {
                '#' => {
                    walls.insert(p);
                }
                '>' => {
                    blizzards.insert((p, R));
                }
                '<' => {
                    blizzards.insert((p, L));
                }
                '^' => {
                    blizzards.insert((p, U));
                }
                'v' => {
                    blizzards.insert((p, D));
                }
                '.' if r == 0 && start.is_none() => {
                    start = Some(p);
                }
                '.' => {
                    end = Some(p);
                }
                _ => continue,
            };
        }
    }

    let min_c = walls.iter().map(|p| p.col).min().unwrap();
    let max_c = walls.iter().map(|p| p.col).max().unwrap();
    let min_r = walls.iter().map(|p| p.row).min().unwrap();
    let max_r = walls.iter().map(|p| p.row).max().unwrap();

    // Fill in a wall above the start so we never need to check bounds.
    for c in min_c..=max_c {
        walls.insert((min_r - 1, c).into());
    }

    (
        walls,
        blizzards,
        P {
            row: max_r,
            col: max_c,
        },
        start.unwrap(),
        end.unwrap(),
    )
}

fn solve(
    walls: HashSet<P>,
    blizzards: HashSet<(P, P)>,
    max: P,
    start: P,
    mut goals: Vec<P>,
) -> usize {
    let mut candidates = HashSet::new();
    candidates.insert(start);

    for step in 0.. {
        if goals.is_empty() {
            return step - 1;
        }
        let blizzard_locs = blizzards
            .iter()
            .map(|(p, d)| (*p + P::from((step, step)) * *d).wrap(max))
            .collect::<HashSet<_>>();
        let next = candidates
            .iter()
            .flat_map(|p| [R, L, U, D, NOOP].map(|d| (*p + d)))
            .collect::<HashSet<_>>();
        candidates = next
            .difference(&blizzard_locs)
            .filter(|p| !walls.contains(p))
            .copied()
            .collect();

        if candidates.contains(&goals[0]) {
            let v = goals.remove(0);
            candidates = HashSet::new();
            candidates.insert(v);
        }
    }
    unreachable!()
}

pub fn part_1(input: &str) -> usize {
    let (walls, blizzards, max, start, end) = parse(input);
    solve(walls, blizzards, max, start, vec![end])
}

pub fn part_2(input: &str) -> usize {
    let (walls, blizzards, max, start, end) = parse(input);
    solve(walls, blizzards, max, start, vec![end, start, end])
}

#[cfg(test)]
pub mod tests {
    use crate::day_24::{part_1, part_2};
    const INPUTS: &str = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;

    #[test]
    pub fn test_day_24_example_part1() {
        assert_eq!(part_1(INPUTS), 18);
    }

    #[test]
    pub fn test_day_24_part1() {
        assert_eq!(part_1(include_str!("input/day_24.txt")), 230);
    }

    #[test]
    pub fn test_day_24_example_part2() {
        assert_eq!(part_2(INPUTS), 54);
    }

    #[test]
    pub fn test_day_24_part2() {
        assert_eq!(part_2(include_str!("input/day_24.txt")), 713);
    }
}
