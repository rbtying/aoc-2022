// --- Day 14: Regolith Reservoir ---

// The distress signal leads you to a giant waterfall! Actually, hang on - the
// signal seems like it's coming from the waterfall itself, and that doesn't
// make any sense. However, you do notice a little path that leads behind the
// waterfall.

// Correction: the distress signal leads you behind a giant waterfall! There
// seems to be a large cave system here, and the signal definitely leads further
// inside.

// As you begin to make your way deeper underground, you feel the ground rumble
// for a moment. Sand begins pouring into the cave! If you don't quickly figure
// out where the sand is going, you could quickly become trapped!

// Fortunately, your familiarity with analyzing the path of falling material
// will come in handy here. You scan a two-dimensional vertical slice of the
// cave above you (your puzzle input) and discover that it is mostly air with
// structures made of rock.

// Your scan traces the path of each solid rock structure and reports the x,y
// coordinates that form the shape of the path, where x represents distance to
// the right and y represents distance down. Each path appears as a single line
// of text in your scan. After the first point of each path, each point
// indicates the end of a straight horizontal or vertical line to be drawn from
// the previous point. For example:

// 498,4 -> 498,6 -> 496,6
// 503,4 -> 502,4 -> 502,9 -> 494,9

// This scan means that there are two paths of rock; the first path consists of
// two straight lines, and the second path consists of three straight lines.
// (Specifically, the first path consists of a line of rock from 498,4 through
// 498,6 and another line of rock from 498,6 through 496,6.)

// The sand is pouring into the cave from point 500,0.

// Drawing rock as #, air as ., and the source of the sand as +, this becomes:

//   4     5  5
//   9     0  0
//   4     0  3
// 0 ......+...
// 1 ..........
// 2 ..........
// 3 ..........
// 4 ....#...##
// 5 ....#...#.
// 6 ..###...#.
// 7 ........#.
// 8 ........#.
// 9 #########.

// Sand is produced one unit at a time, and the next unit of sand is not
// produced until the previous unit of sand comes to rest. A unit of sand is
// large enough to fill one tile of air in your scan.

// A unit of sand always falls down one step if possible. If the tile
// immediately below is blocked (by rock or sand), the unit of sand attempts to
// instead move diagonally one step down and to the left. If that tile is
// blocked, the unit of sand attempts to instead move diagonally one step down
// and to the right. Sand keeps moving as long as it is able to do so, at each
// step trying to move down, then down-left, then down-right. If all three
// possible destinations are blocked, the unit of sand comes to rest and no
// longer moves, at which point the next unit of sand is created back at the
// source.

// So, drawing sand that has come to rest as o, the first unit of sand simply
// falls straight down and then stops:

// ......+...
// ..........
// ..........
// ..........
// ....#...##
// ....#...#.
// ..###...#.
// ........#.
// ......o.#.
// #########.

// The second unit of sand then falls straight down, lands on the first one, and
// then comes to rest to its left:

// ......+...
// ..........
// ..........
// ..........
// ....#...##
// ....#...#.
// ..###...#.
// ........#.
// .....oo.#.
// #########.

// After a total of five units of sand have come to rest, they form this
// pattern:

// ......+...
// ..........
// ..........
// ..........
// ....#...##
// ....#...#.
// ..###...#.
// ......o.#.
// ....oooo#.
// #########.

// After a total of 22 units of sand:

// ......+...
// ..........
// ......o...
// .....ooo..
// ....#ooo##
// ....#ooo#.
// ..###ooo#.
// ....oooo#.
// ...ooooo#.
// #########.

// Finally, only two more units of sand can possibly come to rest:

// ......+...
// ..........
// ......o...
// .....ooo..
// ....#ooo##
// ...o#ooo#.
// ..###ooo#.
// ....oooo#.
// .o.ooooo#.
// #########.

// Once all 24 units of sand shown above have come to rest, all further sand
// flows out the bottom, falling into the endless void. Just for fun, the path
// any new sand takes before falling forever is shown here with ~:

// .......+...
// .......~...
// ......~o...
// .....~ooo..
// ....~#ooo##
// ...~o#ooo#.
// ..~###ooo#.
// ..~..oooo#.
// .~o.ooooo#.
// ~#########.
// ~..........
// ~..........
// ~..........

// Using your scan, simulate the falling sand. How many units of sand come to
// rest before sand starts flowing into the abyss below?

// --- Part Two ---

// You realize you misread the scan. There isn't an endless void at the bottom
// of the scan - there's floor, and you're standing on it!

// You don't have time to scan the floor, so assume the floor is an infinite
// horizontal line with a y coordinate equal to two plus the highest y
// coordinate of any point in your scan.

// In the example above, the highest y coordinate of any point is 9, and so the
// floor is at y=11. (This is as if your scan contained one extra rock path like
// -infinity,11 -> infinity,11.) With the added floor, the example above now
// looks like this:

//         ...........+........
//         ....................
//         ....................
//         ....................
//         .........#...##.....
//         .........#...#......
//         .......###...#......
//         .............#......
//         .............#......
//         .....#########......
//         ....................
// <-- etc #################### etc -->

// To find somewhere safe to stand, you'll need to simulate falling sand until a
// unit of sand comes to rest at 500,0, blocking the source entirely and
// stopping the flow of sand into the cave. In the example above, the situation
// finally looks like this after 93 units of sand come to rest:

// ............o............
// ...........ooo...........
// ..........ooooo..........
// .........ooooooo.........
// ........oo#ooo##o........
// .......ooo#ooo#ooo.......
// ......oo###ooo#oooo......
// .....oooo.oooo#ooooo.....
// ....oooooooooo#oooooo....
// ...ooo#########ooooooo...
// ..ooooo.......ooooooooo..
// #########################

// Using your scan, simulate the falling sand until the source of the sand
// becomes blocked. How many units of sand come to rest?
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum Segment {
    R(isize),
    L(isize),
    D(isize),
    U(isize),
}

impl Segment {
    fn compute(prev: (isize, isize), next: (isize, isize)) -> Segment {
        match (next.0.cmp(&prev.0), next.1.cmp(&prev.1)) {
            (Ordering::Equal, Ordering::Greater) => Segment::D(next.1 - prev.1),
            (Ordering::Equal, Ordering::Less) => Segment::U(prev.1 - next.1),
            (Ordering::Greater, Ordering::Equal) => Segment::R(next.0 - prev.0),
            (Ordering::Less, Ordering::Equal) => Segment::L(prev.0 - next.0),
            _ => unreachable!(),
        }
    }

    fn points(self, start: (isize, isize)) -> Vec<(isize, isize)> {
        match self {
            Segment::R(v) => (1..=v).map(|d| (start.0 + d, start.1)).collect(),
            Segment::L(v) => (1..=v).map(|d| (start.0 - d, start.1)).collect(),
            Segment::D(v) => (1..=v).map(|d| (start.0, start.1 + d)).collect(),
            Segment::U(v) => (1..=v).map(|d| (start.0, start.1 - d)).collect(),
        }
    }
}

fn parse(tuple: &str) -> (isize, isize) {
    let (a, b) = tuple.split_once(',').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

#[derive(Debug, Clone, Copy)]
enum V {
    S,
    R,
}

fn parse_grid(input: &str) -> HashMap<(isize, isize), V> {
    let mut grid = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut iter = line.split(" -> ");
        let start = parse(iter.next().unwrap());
        let mut prev = start;

        grid.insert(start, V::R);

        for next in iter {
            let next = parse(next);
            let s = Segment::compute(prev, next);
            for p in s.points(prev) {
                grid.insert(p, V::R);
            }
            prev = next;
        }
    }
    grid
}

pub fn part_1(input: &str) -> usize {
    let sand_source = (500, 0);
    let mut grid = parse_grid(input);
    let lowest_rock = *grid.keys().map(|(_, y)| y).max().unwrap();

    for i in 1.. {
        let mut sand_pos = sand_source;

        'inner: loop {
            let seq = [
                (sand_pos.0, sand_pos.1 + 1),
                (sand_pos.0 - 1, sand_pos.1 + 1),
                (sand_pos.0 + 1, sand_pos.1 + 1),
                sand_pos,
            ];
            'search: for pos in seq {
                if pos.1 > lowest_rock {
                    return i - 1;
                }
                if grid.get(&pos).is_none() {
                    let at_rest = sand_pos == pos;
                    if at_rest {
                        grid.insert(pos, V::S);
                        break 'inner;
                    }
                    sand_pos = pos;
                    break 'search;
                }
            }
        }
    }
    unreachable!()
}

pub fn part_2(input: &str) -> usize {
    let sand_source = (500, 0);
    let mut grid = parse_grid(input);
    let lowest_rock = *grid.keys().map(|(_, y)| y).max().unwrap();

    for i in 1.. {
        let mut sand_pos = sand_source;

        if grid.get(&sand_pos).is_some() {
            return i - 1;
        }

        'inner: loop {
            let seq = [
                (sand_pos.0, sand_pos.1 + 1),
                (sand_pos.0 - 1, sand_pos.1 + 1),
                (sand_pos.0 + 1, sand_pos.1 + 1),
                sand_pos,
            ];
            'search: for pos in seq {
                let filled = grid.get(&pos).is_some() || pos.1 >= lowest_rock + 2;
                if !filled {
                    let at_rest = sand_pos == pos;
                    if at_rest {
                        grid.insert(pos, V::S);
                        break 'inner;
                    }
                    sand_pos = pos;
                    break 'search;
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
pub mod tests {
    use crate::day_14::{part_1, part_2};

    const INPUTS: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    #[test]
    pub fn test_day_14_example_part1() {
        assert_eq!(part_1(INPUTS), 24);
    }

    #[test]
    pub fn test_day_14_part1() {
        assert_eq!(part_1(include_str!("input/day_14.txt")), 644);
    }

    #[test]
    pub fn test_day_14_example_part2() {
        assert_eq!(part_2(INPUTS), 93);
    }

    #[test]
    pub fn test_day_14_part2() {
        assert_eq!(part_2(include_str!("input/day_14.txt")), 27324);
    }
}
