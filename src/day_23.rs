// See day_23_prompt.txt
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(isize)]
enum Dir {
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7,
}

use Dir::*;

const ALL_DIRS: [Dir; 8] = [N, NE, E, SE, S, SW, W, NW];

fn next_pos(pos: (isize, isize), dir: Dir) -> (isize, isize) {
    let pos = (pos.0 as isize, pos.1 as isize);

    let n_pos = match dir {
        N => (pos.0 - 1, pos.1),
        NE => (pos.0 - 1, pos.1 + 1),
        E => (pos.0, pos.1 + 1),
        SE => (pos.0 + 1, pos.1 + 1),
        S => (pos.0 + 1, pos.1),
        SW => (pos.0 + 1, pos.1 - 1),
        W => (pos.0, pos.1 - 1),
        NW => (pos.0 - 1, pos.1 - 1),
    };

    (n_pos.0 as isize, n_pos.1 as isize)
}

fn run_to_completion(
    elf_positions: &mut HashSet<(isize, isize)>,
    max_steps: Option<usize>,
) -> usize {
    let seq = vec![
        ([N, NE, NW], N),
        ([S, SE, SW], S),
        ([W, NW, SW], W),
        ([E, NE, SE], E),
    ];

    let range = match max_steps {
        Some(v) => Box::new(0..v),
        None => Box::new(0usize..) as Box<dyn Iterator<Item = usize>>,
    };

    let mut new_elf_positions: HashMap<_, Vec<_>> = HashMap::new();
    for r in range {
        for elf in elf_positions.iter() {
            let mut neighbors = [false; 8];

            for (idx, dir) in ALL_DIRS.iter().enumerate() {
                neighbors[idx] = elf_positions.contains(&next_pos(*elf, *dir));
            }

            if neighbors.iter().all(|f| !f) {
                // continue
            } else {
                for i in 0..seq.len() {
                    let (search, dir) = seq[(r + i) % seq.len()];
                    if search.iter().all(|x| !neighbors[*x as usize]) {
                        new_elf_positions
                            .entry(next_pos(*elf, dir))
                            .or_default()
                            .push(*elf);
                        break;
                    }
                }
            };
        }

        let mut updated = false;
        for (new, old) in new_elf_positions.drain() {
            if old.len() == 1 {
                updated = true;
                elf_positions.remove(&old[0]);
                elf_positions.insert(new);
            }
        }

        if !updated {
            return r;
        }
    }
    max_steps.unwrap()
}

pub fn part_1(input: &str) -> usize {
    let grid = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut elf_positions = HashSet::new();

    for (r, line) in grid.iter().enumerate() {
        for (c, v) in line.iter().enumerate() {
            if *v == '#' {
                elf_positions.insert((r as isize, c as isize));
            }
        }
    }

    let _ = run_to_completion(&mut elf_positions, Some(10));

    let min_r = elf_positions.iter().map(|e| e.0).min().unwrap();
    let max_r = elf_positions.iter().map(|e| e.0).max().unwrap();
    let min_c = elf_positions.iter().map(|e| e.1).min().unwrap();
    let max_c = elf_positions.iter().map(|e| e.1).max().unwrap();

    ((max_r - min_r + 1) * (max_c - min_c + 1)) as usize - elf_positions.len()
}

pub fn part_2(input: &str) -> usize {
    let grid = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut elf_positions = HashSet::new();

    for (r, line) in grid.iter().enumerate() {
        for (c, v) in line.iter().enumerate() {
            if *v == '#' {
                elf_positions.insert((r as isize, c as isize));
            }
        }
    }
    run_to_completion(&mut elf_positions, None) + 1
}

#[cfg(test)]
pub mod tests {
    use crate::day_23::{part_1, part_2};
    const INPUTS: &str = r#"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............
"#;

    #[test]
    pub fn test_day_23_example_part1() {
        assert_eq!(part_1(INPUTS), 110);
    }

    #[test]
    pub fn test_day_23_part1() {
        assert_eq!(part_1(include_str!("input/day_23.txt")), 3917);
    }

    #[test]
    pub fn test_day_23_example_part2() {
        assert_eq!(part_2(INPUTS), 20);
    }

    #[test]
    pub fn test_day_23_part2() {
        assert_eq!(part_2(include_str!("input/day_23.txt")), 988);
    }
}
