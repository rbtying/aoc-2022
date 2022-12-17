use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Rock {
    HLine,
    Cross,
    Corner,
    VLine,
    Square,
}

impl Rock {
    fn nth_rock(idx: usize) -> Rock {
        match idx % 5 {
            0 => Rock::HLine,
            1 => Rock::Cross,
            2 => Rock::Corner,
            3 => Rock::VLine,
            4 => Rock::Square,
            _ => unreachable!(),
        }
    }

    fn height(self) -> usize {
        match self {
            Rock::HLine => 1,
            Rock::Cross => 3,
            Rock::Corner => 3,
            Rock::VLine => 4,
            Rock::Square => 2,
        }
    }

    fn iter(self) -> impl Iterator<Item = (usize, usize)> {
        match self {
            Rock::HLine => vec![(0, 0), (1, 0), (2, 0), (3, 0)].into_iter(),
            Rock::Cross => vec![(0, 1), (1, 1), (2, 1), (1, 2), (1, 0)].into_iter(),
            Rock::Corner => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)].into_iter(),
            Rock::VLine => vec![(0, 0), (0, 1), (0, 2), (0, 3)].into_iter(),
            Rock::Square => vec![(0, 0), (0, 1), (1, 0), (1, 1)].into_iter(),
        }
    }
}

struct Room {
    arr: Vec<u8>,
}

impl Room {
    fn new() -> Self {
        Room { arr: vec![] }
    }

    fn emplace(&mut self, pos: (usize, usize), rock: Rock) {
        while self.arr.len() < pos.1 + rock.height() {
            self.arr.push(0);
        }

        for (dx, dy) in rock.iter() {
            let (x, y) = (pos.0 + dx, pos.1 + dy);
            assert!(!self.occupied(x, y));

            self.arr[y] |= 1 << x;
        }
    }

    fn highest_occupied_row(&self) -> usize {
        self.arr.len()
    }

    fn valid(&self, pos: (usize, usize), rock: Rock) -> bool {
        for (dx, dy) in rock.iter() {
            let (x, y) = (pos.0 + dx, pos.1 + dy);
            if self.occupied(x, y) || x >= 7 {
                return false;
            }
        }
        true
    }

    fn occupied(&self, x: usize, y: usize) -> bool {
        if y >= self.arr.len() {
            false
        } else if x >= 7 {
            true
        } else {
            self.arr[y] & (1 << x) != 0
        }
    }

    fn top_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.arr
            .iter()
            .rev()
            .take(20)
            .collect::<Vec<_>>()
            .hash(&mut hasher);
        hasher.finish()
    }
}

fn go_right(room: &Room, pos: (usize, usize), rock: Rock) -> Option<(usize, usize)> {
    if room.valid((pos.0 + 1, pos.1), rock) {
        Some((pos.0 + 1, pos.1))
    } else {
        None
    }
}

fn go_left(room: &Room, pos: (usize, usize), rock: Rock) -> Option<(usize, usize)> {
    if pos.0 > 0 && room.valid((pos.0 - 1, pos.1), rock) {
        Some((pos.0 - 1, pos.1))
    } else {
        None
    }
}

fn go_down(room: &Room, pos: (usize, usize), rock: Rock) -> Option<(usize, usize)> {
    if pos.1 > 0 && room.valid((pos.0, pos.1 - 1), rock) {
        Some((pos.0, pos.1 - 1))
    } else {
        None
    }
}

fn place_rock(
    room: &mut Room,
    gas_iter: &mut impl Iterator<Item = (usize, bool)>,
    rock: Rock,
) -> usize {
    let mut pos = (2, room.highest_occupied_row() + 3);
    loop {
        let (d_idx, r) = gas_iter.next().unwrap();
        if r {
            if let Some(new_pos) = go_right(room, pos, rock) {
                pos = new_pos;
            }
        } else if let Some(new_pos) = go_left(room, pos, rock) {
            pos = new_pos;
        }

        if let Some(new_pos) = go_down(room, pos, rock) {
            pos = new_pos;
        } else {
            room.emplace(pos, rock);
            break d_idx;
        }
    }
}

fn solve(input: &str, num_iter: usize) -> usize {
    let mut room = Room::new();
    let mut gas_iter = input
        .chars()
        .filter(|c| *c == '>' || *c == '<')
        .map(|c| c == '>')
        .enumerate()
        .cycle();

    let mut cache = HashMap::new();

    for i in 0..num_iter {
        let rock = Rock::nth_rock(i);
        let d_idx = place_rock(&mut room, &mut gas_iter, rock);

        // We notice that there's a repeating cycle in the output, so let's try
        // to find it via the janky method. This makes sense, because the two
        // inputs are themselves cyclic -- a cycle of gas and a cycle of rocks.
        if let Some((first_observed_at, highest_row_when_observed)) =
            cache.get(&(rock, d_idx, room.top_hash()))
        {
            // p is the observed period of the cycle, since we should hit the
            // cache at the second cycle.
            let p = i - first_observed_at;

            // Do some modular arithmetic to find the period we are in at the
            // end point.
            if (num_iter - 1) % p == i % p {
                let rows_gained_per_cycle = room.highest_occupied_row() - highest_row_when_observed;
                let remaining_cycles = (num_iter - i - 1) / p;
                return room.highest_occupied_row() + remaining_cycles * rows_gained_per_cycle;
            }
        } else {
            // The two cycle indexes are the rock and the gas index. We also
            // hash the top of the room so far.
            cache.insert(
                (rock, d_idx, room.top_hash()),
                // Store the current iterator and the highest row at the time in
                // the cache.
                (i, room.highest_occupied_row()),
            );
        }
    }

    room.highest_occupied_row()
}

pub fn part_1(input: &str) -> usize {
    solve(input, 2022)
}

pub fn part_2(input: &str) -> usize {
    solve(input, 1000000000000)
}

#[cfg(test)]
pub mod tests {
    use crate::day_17::{part_1, part_2};

    const INPUTS: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    #[test]
    pub fn test_day_17_example_part1() {
        assert_eq!(part_1(INPUTS), 3068);
    }

    #[test]
    pub fn test_day_17_part1() {
        assert_eq!(part_1(include_str!("input/day_17.txt")), 3106);
    }

    #[test]
    pub fn test_day_17_example_part2() {
        assert_eq!(part_2(INPUTS), 1514285714288);
    }

    #[test]
    pub fn test_day_17_part2() {
        assert_eq!(part_2(include_str!("input/day_17.txt")), 1537175792495);
    }
}
