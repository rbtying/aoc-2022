// --- Day 12: Hill Climbing Algorithm ---

// You try contacting the Elves using your handheld device, but the river you're
// following must be too low to get a decent signal.

// You ask the device for a heightmap of the surrounding area (your puzzle
// input). The heightmap shows the local area from above broken into a grid; the
// elevation of each square of the grid is given by a single lowercase letter,
// where a is the lowest elevation, b is the next-lowest, and so on up to the
// highest elevation, z.

// Also included on the heightmap are marks for your current position (S) and
// the location that should get the best signal (E). Your current position (S)
// has elevation a, and the location that should get the best signal (E) has
// elevation z.

// You'd like to reach E, but to save energy, you should do it in as few steps
// as possible. During each step, you can move exactly one square up, down,
// left, or right. To avoid needing to get out your climbing gear, the elevation
// of the destination square can be at most one higher than the elevation of
// your current square; that is, if your current elevation is m, you could step
// to elevation n, but not to elevation o. (This also means that the elevation
// of the destination square can be much lower than the elevation of your
// current square.)

// For example:

// Sabqponm
// abcryxxl
// accszExk
// acctuvwj
// abdefghi

// Here, you start in the top-left corner; your goal is near the middle. You
// could start by moving down or right, but eventually you'll need to head
// toward the e at the bottom. From there, you can spiral around to the goal:

// v..v<<<<
// >v.vv<<^
// .>vv>E^^
// ..v>>>^^
// ..>>>>>^

// In the above diagram, the symbols indicate whether the path exits each square
// moving up (^), down (v), left (<), or right (>). The location that should get
// the best signal is still E, and . marks unvisited squares.

// This path reaches the goal in 31 steps, the fewest possible.

// What is the fewest steps required to move from your current position to the
// location that should get the best signal?

// The first half of this puzzle is complete! It provides one gold star: *

// --- Part Two ---

// As you walk up the hill, you suspect that the Elves will want to turn this
// into a hiking trail. The beginning isn't very scenic, though; perhaps you can
// find a better starting point.

// To maximize exercise while hiking, the trail should start as low as possible:
// elevation a. The goal is still the square marked E. However, the trail should
// still be direct, taking the fewest steps to reach its goal. So, you'll need
// to find the shortest path from any square at elevation a to the square marked
// E.

// Again consider the example from above:

// Sabqponm
// abcryxxl
// accszExk
// acctuvwj
// abdefghi

// Now, there are six choices for starting position (five marked a, plus the
// square marked S that counts as being at elevation a). If you start at the
// bottom-left square, you can reach the goal most quickly:

// ...v<<<<
// ...vv<<^
// ...v>E^^
// .>v>>>^^
// >^>>>>>^

// This path reaches the goal in only 29 steps, the fewest possible.

// What is the fewest steps required to move starting from any square with
// elevation a to the location that should get the best signal?

use std::collections::{BinaryHeap, HashMap};

fn find_ch(grid: &[Vec<char>], ch: char) -> (usize, usize) {
    for (i, r) in grid.iter().enumerate() {
        for (j, ch_) in r.iter().enumerate() {
            if *ch_ == ch {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn raw_neighbors(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + '_ {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(move |d| (pos.0 as isize + d.0, pos.1 as isize + d.1))
        .filter(move |p| {
            p.0 >= 0
                && p.1 >= 0
                && (p.0 as usize) < grid.len()
                && (p.1 as usize) < grid[p.0 as usize].len()
        })
        .map(|p| (p.0 as usize, p.1 as usize))
}

fn neighbors(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + '_ {
    raw_neighbors(grid, pos)
        .filter(move |p| elevation(grid[p.0][p.1]) <= elevation(grid[pos.0][pos.1]) + 1)
}

fn rev_neighbors(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + '_ {
    raw_neighbors(grid, pos).filter(move |p| neighbors(grid, *p).any(|pp| pp == pos))
}

fn elevation(c: char) -> usize {
    let c = if c == 'S' {
        'a'
    } else if c == 'E' {
        'z'
    } else {
        c
    };

    c as usize - 'a' as usize
}

fn dijkstra<
    'a,
    N: Fn(&'a Vec<Vec<char>>, (usize, usize)) -> I,
    I: Iterator<Item = (usize, usize)>,
>(
    grid: &'a Vec<Vec<char>>,
    start: char,
    neighbor_func: N,
) -> HashMap<(usize, usize), usize> {
    let mut dist = HashMap::new();
    for pos in positions(grid) {
        dist.insert(pos, usize::MAX);
    }
    let start = find_ch(grid, start);
    dist.insert(start, 0);

    let mut heap = BinaryHeap::new();
    heap.push((0, start));

    while let Some((d, p)) = heap.pop() {
        if d > dist[&p] {
            continue;
        }

        for neighbor in neighbor_func(grid, p) {
            if d + 1 < dist[&neighbor] {
                heap.push((d + 1, neighbor));
                dist.insert(neighbor, d + 1);
            }
        }
    }

    dist
}

pub fn part_1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let dist = dijkstra(&grid, 'S', neighbors);
    dist[&find_ch(&grid, 'E')]
}

fn positions(grid: &Vec<Vec<char>>) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..grid.len()).flat_map(move |i| (0..grid[i].len()).map(move |j| (i, j)))
}

pub fn part_2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();
    let dist = dijkstra(&grid, 'E', rev_neighbors);

    let mut mins: Vec<_> = positions(&grid)
        .filter(|p| elevation(grid[p.0][p.1]) == 0)
        .map(|p| dist[&p])
        .collect();
    mins.sort();
    mins[0]
}

#[cfg(test)]
pub mod tests {
    use crate::day_12::{part_1, part_2};

    const INPUTS: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    pub fn test_day_12_example_part1() {
        assert_eq!(part_1(INPUTS), 31);
    }

    #[test]
    pub fn test_day_12_part1() {
        assert_eq!(part_1(include_str!("input/day_12.txt")), 339);
    }

    #[test]
    pub fn test_day_12_example_part2() {
        assert_eq!(part_2(INPUTS), 29);
    }

    #[test]
    pub fn test_day_12_part2() {
        assert_eq!(part_2(include_str!("input/day_12.txt")), 332);
    }
}
