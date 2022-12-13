// The day 13 prompt's examples are too long, see day_13_prompt.txt

use std::cmp::Ordering;
use std::iter::Peekable;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Val {
    L(Vec<Val>),
    V(usize),
}

// Note: this problem is interesting pretty much only because of the need to
// implement recursive parsing. Otherwise, it'd be pretty easy; just a
// comparator.
fn parse<I: Iterator<Item = char>>(s: &mut Peekable<I>) -> Val {
    match s.peek().unwrap() {
        '[' => {
            s.next().unwrap();

            let mut values = vec![];

            loop {
                match s.peek() {
                    None => unreachable!("Unexpected EOF"),
                    Some(']') => {
                        s.next().unwrap();
                        break;
                    }
                    _ => {
                        values.push(parse(s));
                        if let Some(',') = s.peek() {
                            s.next().unwrap();
                        }
                    }
                }
            }

            Val::L(values)
        }
        c if c.is_ascii_digit() => {
            let mut v = 0;
            while let Some(c) = s.peek() {
                if c.is_ascii_digit() {
                    let c = s.next().unwrap().to_digit(10).unwrap();
                    v = v * 10 + c as usize;
                } else {
                    break;
                }
            }
            Val::V(v)
        }
        c => unreachable!("Unexpected next char {}", c),
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Val {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Val::V(a), Val::V(b)) => a.cmp(b),
            (Val::L(a), Val::V(_)) => recursive_cmp(a.iter(), std::iter::once(other)),
            (Val::V(_), Val::L(b)) => recursive_cmp(std::iter::once(self), b.iter()),
            (Val::L(a), Val::L(b)) => recursive_cmp(a.iter(), b.iter()),
        }
    }
}

fn recursive_cmp<'a, 'b>(
    mut a_iter: impl Iterator<Item = &'a Val>,
    mut b_iter: impl Iterator<Item = &'b Val>,
) -> Ordering {
    match (a_iter.next(), b_iter.next()) {
        (Some(a), Some(b)) => {
            let r = a.cmp(b);
            if r == Ordering::Equal {
                recursive_cmp(a_iter, b_iter)
            } else {
                r
            }
        }
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (None, None) => Ordering::Equal,
    }
}

pub fn part_1(input: &str) -> usize {
    let mut idx_sum = 0;
    for (idx, pair) in input.split("\n\n").enumerate() {
        let (a, b) = pair.split_once('\n').unwrap();
        let mut a_iter = a.chars().peekable();
        let mut b_iter = b.chars().peekable();
        let va = parse(&mut a_iter);
        let vb = parse(&mut b_iter);

        if va < vb {
            idx_sum += idx + 1;
        }
    }

    idx_sum
}

pub fn part_2(input: &str) -> usize {
    let mut packets: Vec<Val> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse(&mut l.chars().peekable()))
        .collect();

    let v2 = Val::L(vec![Val::L(vec![Val::V(2)])]);
    let v6 = Val::L(vec![Val::L(vec![Val::V(6)])]);

    packets.push(v2.clone());
    packets.push(v6.clone());
    packets.sort();

    let v2_pos = packets.iter().position(|v| *v == v2).unwrap() + 1;
    let v6_pos = packets.iter().position(|v| *v == v6).unwrap() + 1;

    v6_pos * v2_pos
}

#[cfg(test)]
pub mod tests {
    use crate::day_13::{parse, part_1, part_2, Val};

    const INPUTS: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    pub fn test_day_13_parse() {
        fn p(s: &str) -> Val {
            parse(&mut s.chars().peekable())
        }
        assert_eq!(
            p("[[1],[2,3,4]]"),
            Val::L(vec![
                Val::L(vec![Val::V(1),]),
                Val::L(vec![Val::V(2), Val::V(3), Val::V(4),])
            ])
        )
    }

    #[test]
    pub fn test_day_13_example_part1() {
        assert_eq!(part_1(INPUTS), 13);
    }

    #[test]
    pub fn test_day_13_part1() {
        assert_eq!(part_1(include_str!("input/day_13.txt")), 5390);
    }

    #[test]
    pub fn test_day_13_example_part2() {
        assert_eq!(part_2(INPUTS), 140);
    }

    #[test]
    pub fn test_day_13_part2() {
        assert_eq!(part_2(include_str!("input/day_13.txt")), 19261);
    }
}
