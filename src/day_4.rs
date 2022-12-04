// --- Day 4: Camp Cleanup ---
// Space needs to be cleared before the last supplies can be unloaded from the
// ships, and so several Elves have been assigned the job of cleaning up
// sections of the camp. Every section has a unique ID number, and each Elf is
// assigned a range of section IDs.

// However, as some of the Elves compare their section assignments with each
// other, they've noticed that many of the assignments overlap. To try to
// quickly find overlaps and reduce duplicated effort, the Elves pair up and
// make a big list of the section assignments for each pair (your puzzle input).

// For example, consider the following list of section assignment pairs:

// 2-4,6-8
// 2-3,4-5
// 5-7,7-9
// 2-8,3-7
// 6-6,4-6
// 2-6,4-8

// For the first few pairs, this list means:

// Within the first pair of Elves, the first Elf was assigned sections 2-4
// (sections 2, 3, and 4), while the second Elf was assigned sections 6-8
// (sections 6, 7, 8).

// The Elves in the second pair were each assigned two sections.

// The Elves in the third pair were each assigned three sections: one got
// sections 5, 6, and 7, while the other also got 7, plus 8 and 9.

// This example list uses single-digit section IDs to make it easier to draw;
// your actual list might contain larger numbers. Visually, these pairs of
// section assignments look like this:

// .234.....  2-4
// .....678.  6-8

// .23......  2-3
// ...45....  4-5

// ....567..  5-7
// ......789  7-9

// .2345678.  2-8
// ..34567..  3-7

// .....6...  6-6
// ...456...  4-6

// .23456...  2-6
// ...45678.  4-8

// Some of the pairs have noticed that one of their assignments fully contains
// the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by
// 4-6. In pairs where one assignment fully contains the other, one Elf in the
// pair would be exclusively cleaning sections their partner will already be
// cleaning, so these seem like the most in need of reconsideration. In this
// example, there are 2 such pairs.

// In how many assignment pairs does one range fully contain the other?

// --- Part Two ---

// It seems like there is still quite a bit of duplicate work planned. Instead,
// the Elves would like to know the number of pairs that overlap at all.

// In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't
// overlap, while the remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and
// 2-6,4-8) do overlap:

// 5-7,7-9 overlaps in a single section, 7.
// 2-8,3-7 overlaps all of the sections 3 through 7.
// 6-6,4-6 overlaps in a single section, 6.
// 2-6,4-8 overlaps in sections 4, 5, and 6.
// So, in this example, the number of overlapping assignment pairs is 4.

// In how many assignment pairs do the ranges overlap?

fn parse(input: &str) -> Vec<((u64, u64), (u64, u64))> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            let (a1, a2) = a.split_once('-').unwrap();
            let (b1, b2) = b.split_once('-').unwrap();

            (
                (a1.parse().unwrap(), a2.parse().unwrap()),
                (b1.parse().unwrap(), b2.parse().unwrap()),
            )
        })
        .collect()
}

pub fn part_1(inputs: &str) -> u64 {
    let mut fully_contains = 0;
    for (p1, p2) in parse(inputs) {
        let p1_in_p2 = p1.0 >= p2.0 && p1.1 <= p2.1;
        let p2_in_p1 = p2.0 >= p1.0 && p2.1 <= p1.1;
        if p1_in_p2 || p2_in_p1 {
            fully_contains += 1;
        }
    }

    fully_contains
}

pub fn part_2(inputs: &str) -> u64 {
    let mut overlaps = 0;
    for (p1, p2) in parse(inputs) {
        if p1.0.max(p2.0) <= p1.1.min(p2.1) {
            overlaps += 1;
        }
    }

    overlaps
}

#[cfg(test)]
pub mod tests {
    use crate::day_4::{part_1, part_2};

    const INPUTS: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    pub fn test_day_4_example_part1() {
        assert_eq!(part_1(INPUTS), 2);
    }

    #[test]
    pub fn test_day_4_part1() {
        assert_eq!(part_1(include_str!("input/day_4.txt")), 530);
    }

    #[test]
    pub fn test_day_4_example_part2() {
        assert_eq!(part_2(INPUTS), 4);
    }

    #[test]
    pub fn test_day_4_part2() {
        assert_eq!(part_2(include_str!("input/day_4.txt")), 903);
    }
}
