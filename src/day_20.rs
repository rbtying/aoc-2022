// --- Day 20: Grove Positioning System ---

// It's finally time to meet back up with the Elves. When you try to contact
// them, however, you get no reply. Perhaps you're out of range?

// You know they're headed to the grove where the star fruit grows, so if you
// can figure out where that is, you should be able to meet back up with them.

// Fortunately, your handheld device has a file (your puzzle input) that
// contains the grove's coordinates! Unfortunately, the file is encrypted - just
// in case the device were to fall into the wrong hands.

// Maybe you can decrypt it?

// When you were still back at the camp, you overheard some Elves talking about
// coordinate file encryption. The main operation involved in decrypting the
// file is called mixing.

// The encrypted file is a list of numbers. To mix the file, move each number
// forward or backward in the file a number of positions equal to the value of
// the number being moved. The list is circular, so moving a number off one end
// of the list wraps back around to the other end as if the ends were connected.

// For example, to move the 1 in a sequence like 4, 5, 6, 1, 7, 8, 9, the 1
// moves one position forward: 4, 5, 6, 7, 1, 8, 9. To move the -2 in a sequence
// like 4, -2, 5, 6, 7, 8, 9, the -2 moves two positions backward, wrapping
// around: 4, 5, 6, 7, 8, -2, 9.

// The numbers should be moved in the order they originally appear in the
// encrypted file. Numbers moving around during the mixing process do not change
// the order in which the numbers are moved.

// Consider this encrypted file:

// 1
// 2
// -3
// 3
// -2
// 0
// 4

// Mixing this file proceeds as follows:

// Initial arrangement:
// 1, 2, -3, 3, -2, 0, 4

// 1 moves between 2 and -3:
// 2, 1, -3, 3, -2, 0, 4

// 2 moves between -3 and 3:
// 1, -3, 2, 3, -2, 0, 4

// -3 moves between -2 and 0:
// 1, 2, 3, -2, -3, 0, 4

// 3 moves between 0 and 4:
// 1, 2, -2, -3, 0, 3, 4

// -2 moves between 4 and 1:
// 1, 2, -3, 0, 3, 4, -2

// 0 does not move:
// 1, 2, -3, 0, 3, 4, -2

// 4 moves between -3 and 0:
// 1, 2, -3, 4, 0, 3, -2

// Then, the grove coordinates can be found by looking at the 1000th, 2000th,
// and 3000th numbers after the value 0, wrapping around the list as necessary.
// In the above example, the 1000th number after 0 is 4, the 2000th is -3, and
// the 3000th is 2; adding these together produces 3.

// Mix your encrypted file exactly once. What is the sum of the three numbers
// that form the grove coordinates?

// --- Part Two ---

// The grove coordinate values seem nonsensical. While you ponder the mysteries
// of Elf encryption, you suddenly remember the rest of the decryption routine
// you overheard back at camp.

// First, you need to apply the decryption key, 811589153. Multiply each number
// by the decryption key before you begin; this will produce the actual list of
// numbers to mix.

// Second, you need to mix the list of numbers ten times. The order in which the
// numbers are mixed does not change during mixing; the numbers are still moved
// in the order they appeared in the original, pre-mixed list. (So, if -3
// appears fourth in the original list of numbers to mix, -3 will be the fourth
// number to move during each round of mixing.)

// [...]

// After 10 rounds of mixing:
// 0, -2434767459, 1623178306, 3246356612, -1623178306, 2434767459, 811589153

// The grove coordinates can still be found in the same way. Here, the 1000th
// number after 0 is 811589153, the 2000th is 2434767459, and the 3000th is
// -1623178306; adding these together produces 1623178306.

// Apply the decryption key and mix your encrypted file ten times. What is the
// sum of the three numbers that form the grove coordinates?

fn decrypt(nums: &[i64], iterations: usize, decryption_key: i64) -> i64 {
    let nums = nums.iter().map(|x| x * decryption_key).collect::<Vec<_>>();
    let len = nums.len();
    let ilen = len as i64;
    let mut indices = (0..nums.len()).collect::<Vec<_>>();

    for _ in 0..iterations {
        for (i, &x) in nums.iter().enumerate() {
            let pos = indices.iter().position(|&y| y == i).unwrap();
            indices.remove(pos);
            let new_i = (pos as i64 + x).rem_euclid(ilen - 1) as usize;
            indices.insert(new_i, i);
        }
    }

    let z_i = indices.iter().position(|&i| nums[i] == 0).unwrap();
    nums[indices[(z_i + 1000) % len]]
        + nums[indices[(z_i + 2000) % len]]
        + nums[indices[(z_i + 3000) % len]]
}

pub fn part_1(input: &str) -> i64 {
    let file: Vec<i64> = input
        .split_whitespace()
        .map(|l| l.parse().unwrap())
        .collect();
    decrypt(&file, 1, 1)
}

pub fn part_2(input: &str) -> i64 {
    let file: Vec<i64> = input
        .split_whitespace()
        .map(|l| l.parse().unwrap())
        .collect();
    decrypt(&file, 10, 811589153)
}

#[cfg(test)]
pub mod tests {
    use crate::day_20::{part_1, part_2};

    const INPUTS: &str = r#"1
2
-3
3
-2
0
4"#;
    #[test]
    pub fn test_day_20_example_part1() {
        assert_eq!(part_1(INPUTS), 3);
    }

    #[test]
    pub fn test_day_20_part1() {
        assert_eq!(part_1(include_str!("input/day_20.txt")), 5904);
    }

    #[test]
    pub fn test_day_20_example_part2() {
        assert_eq!(part_2(INPUTS), 1623178306);
    }

    #[test]
    pub fn test_day_20_part2() {
        assert_eq!(part_2(include_str!("input/day_20.txt")), 8332585833851);
    }
}
