// --- Day 8: Treetop Tree House ---

// The expedition comes across a peculiar patch of tall trees all planted
// carefully in a grid. The Elves explain that a previous expedition planted
// these trees as a reforestation effort. Now, they're curious if this would be
// a good location for a tree house.

// First, determine whether there is enough tree cover here to keep a tree house
// hidden. To do this, you need to count the number of trees that are visible
// from outside the grid when looking directly along a row or column.

// The Elves have already launched a quadcopter to generate a map with the
// height of each tree (your puzzle input). For example:

// 30373
// 25512
// 65332
// 33549
// 35390

// Each tree is represented as a single digit whose value is its height, where 0
// is the shortest and 9 is the tallest.

// A tree is visible if all of the other trees between it and an edge of the
// grid are shorter than it. Only consider trees in the same row or column; that
// is, only look up, down, left, or right from any given tree.

// All of the trees around the edge of the grid are visible - since they are
// already on the edge, there are no trees to block the view. In this example,
// that only leaves the interior nine trees to consider:

// The top-left 5 is visible from the left and top. (It isn't visible from the
// right or bottom since other trees of height 5 are in the way.)

// The top-middle 5 is visible from the top and right.

// The top-right 1 is not visible from any direction; for it to be visible,
// there would need to only be trees of height 0 between it and an edge.

// The left-middle 5 is visible, but only from the right.

// The center 3 is not visible from any direction; for it to be visible, there
// would need to be only trees of at most height 2 between it and an edge.

// The right-middle 3 is visible from the right.

// In the bottom row, the middle 5 is visible, but the 3 and 4 are not.

// With 16 trees visible on the edge and another 5 visible in the interior, a
// total of 21 trees are visible in this arrangement.

// Consider your map; how many trees are visible from outside the grid?

// --- Part Two ---

// Content with the amount of tree cover available, the Elves just need to know
// the best spot to build their tree house: they would like to be able to see a
// lot of trees.

// To measure the viewing distance from a given tree, look up, down, left, and
// right from that tree; stop if you reach an edge or at the first tree that is
// the same height or taller than the tree under consideration. (If a tree is
// right on the edge, at least one of its viewing distances will be zero.)

// The Elves don't care about distant trees taller than those found by the rules
// above; the proposed tree house has large eaves to keep it dry, so they
// wouldn't be able to see higher than the tree house anyway.

// In the example above, consider the middle 5 in the second row:

// 30373
// 25512
// 65332
// 33549
// 35390

// Looking up, its view is not blocked; it can see 1 tree (of height 3).

// Looking left, its view is blocked immediately; it can see only 1 tree (of
// height 5, right next to it).

// Looking right, its view is not blocked; it can see 2 trees.

// Looking down, its view is blocked eventually; it can see 2 trees (one of
// height 3, then the tree of height 5 that blocks its view).

// A tree's scenic score is found by multiplying together its viewing distance
// in each of the four directions. For this tree, this is 4 (found by
// multiplying 1 * 1 * 2 * 2).

// However, you can do even better: consider the tree of height 5 in the middle
// of the fourth row:

// 30373
// 25512
// 65332
// 33549
// 35390

// Looking up, its view is blocked at 2 trees (by another tree with a height of
// 5).

// Looking left, its view is not blocked; it can see 2 trees.

// Looking down, its view is also not blocked; it can see 1 tree.

// Looking right, its view is blocked at 2 trees (by a massive tree of height
// 9).

// This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the
// tree house.

// Consider each tree on your map. What is the highest scenic score possible for
// any tree?

use std::collections::HashSet;

#[allow(clippy::needless_range_loop)]
pub fn part_1(input: &str) -> usize {
    let grid = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| (c as isize - '0' as isize))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visible = HashSet::new();

    for i in 0..grid.len() {
        let mut v = -1;
        for j in 0..grid[i].len() {
            if grid[i][j] > v {
                visible.insert((i, j));
                v = grid[i][j]
            }
        }
    }

    for i in 0..grid.len() {
        let mut v = -1;
        for j in (0..grid[i].len()).rev() {
            if grid[i][j] > v {
                visible.insert((i, j));
                v = grid[i][j]
            }
        }
    }

    for j in 0..grid[0].len() {
        let mut v = -1;
        for i in 0..grid.len() {
            if grid[i][j] > v {
                visible.insert((i, j));
                v = grid[i][j]
            }
        }
    }

    for j in 0..grid[0].len() {
        let mut v = -1;
        for i in (0..grid.len()).rev() {
            if grid[i][j] > v {
                visible.insert((i, j));
                v = grid[i][j]
            }
        }
    }

    visible.len()
}

fn count_until_including(iter: impl Iterator<Item = bool>) -> usize {
    let mut ct = 0;

    for v in iter {
        ct += 1;
        if !v {
            break;
        }
    }
    ct
}

pub fn part_2(input: &str) -> usize {
    let grid = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| (c as isize - '0' as isize))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut max_score = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let h = grid[i][j];
            let s1 = count_until_including((i + 1..grid.len()).map(|ii| grid[ii][j] < h));
            let s2 = count_until_including((0..i).rev().map(|ii| grid[ii][j] < h));
            let s3 = count_until_including((j + 1..grid[i].len()).map(|jj| grid[i][jj] < h));
            let s4 = count_until_including((0..j).rev().map(|jj| grid[i][jj] < h));

            max_score = max_score.max(s1 * s2 * s3 * s4);
        }
    }

    max_score
}

#[cfg(test)]
pub mod tests {
    use crate::day_8::{part_1, part_2};

    const INPUTS: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    pub fn test_day_8_example_part1() {
        assert_eq!(part_1(INPUTS), 21);
    }

    #[test]
    pub fn test_day_8_part1() {
        assert_eq!(part_1(include_str!("input/day_8.txt")), 1835);
    }

    #[test]
    pub fn test_day_8_example_part2() {
        assert_eq!(part_2(INPUTS), 8);
    }

    #[test]
    pub fn test_day_8_part2() {
        assert_eq!(part_2(include_str!("input/day_8.txt")), 263670);
    }
}
