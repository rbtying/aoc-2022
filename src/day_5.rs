use std::collections::HashMap;

struct State {
    stacks: HashMap<usize, Vec<char>>,
    lookups: HashMap<char, usize>,
}

impl State {
    pub fn parse<'a, I: Iterator<Item = &'a str> + Sized>(input: &mut I) -> State {
        let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
        let mut revlabels: HashMap<char, usize> = HashMap::new();

        for line in input.by_ref() {
            if line.is_empty() {
                continue;
            }

            let mut opened = None;
            let mut parsed_number = false;
            let mut label = ' ';

            for (idx, char) in line.chars().enumerate() {
                match char {
                    '[' => opened = Some(idx + 1),
                    ']' if opened.is_some() => {
                        stacks.entry(opened.unwrap()).or_default().push(label);
                        opened = None;
                    }
                    _ if opened.is_some() => label = char,
                    a if a.is_numeric() => {
                        revlabels.insert(a, idx);
                        parsed_number = true;
                    }
                    _ => continue,
                }
            }

            if parsed_number {
                break;
            }
        }
        stacks.values_mut().for_each(|v| v.reverse());

        State {
            stacks,
            lookups: revlabels,
        }
    }

    fn get(&mut self, name: char) -> &'_ mut Vec<char> {
        self.stacks.get_mut(&self.lookups[&name]).unwrap()
    }

    fn top(&self) -> String {
        let mut keys = self.lookups.keys().collect::<Vec<_>>();
        keys.sort();

        keys.into_iter()
            .map(|label| *self.stacks[&self.lookups[label]].last().unwrap())
            .collect()
    }
}

fn parse_cmd(line: &str) -> (usize, char, char) {
    let mut cmd_str_iter = line.split_ascii_whitespace();
    assert_eq!(cmd_str_iter.next().unwrap(), "move");
    let num = cmd_str_iter.next().unwrap().parse::<usize>().unwrap();
    assert_eq!(cmd_str_iter.next().unwrap(), "from");
    let from = cmd_str_iter.next().unwrap().chars().next().unwrap();
    assert_eq!(cmd_str_iter.next().unwrap(), "to");
    let to = cmd_str_iter.next().unwrap().chars().next().unwrap();
    (num, from, to)
}

pub fn part_1(input: &str) -> String {
    let mut iter = input.lines();
    let mut s = State::parse(&mut iter);

    for line in iter {
        if line.is_empty() {
            continue;
        }

        let (num, from, to) = parse_cmd(line);

        for _ in 0..num {
            let v = s.get(from).pop().unwrap();
            s.get(to).push(v);
        }
    }

    s.top()
}

pub fn part_2(input: &str) -> String {
    let mut iter = input.lines();
    let mut s = State::parse(&mut iter);
    for line in iter {
        if line.is_empty() {
            continue;
        }

        let (num, from, to) = parse_cmd(line);

        let mut to_xfer = vec![];

        for _ in 0..num {
            to_xfer.push(s.get(from).pop().unwrap());
        }

        for v in to_xfer.into_iter().rev() {
            s.get(to).push(v);
        }
    }

    s.top()
}

#[cfg(test)]
pub mod tests {
    use crate::day_5::{part_1, part_2};

    const INPUTS: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    pub fn test_day_5_example_part1() {
        assert_eq!(part_1(INPUTS), "CMZ");
    }

    #[test]
    pub fn test_day_5_part1() {
        assert_eq!(part_1(include_str!("input/day_5.txt")), "MQSHJMWNH");
    }

    #[test]
    pub fn test_day_5_example_part2() {
        assert_eq!(part_2(INPUTS), "MCD");
    }

    #[test]
    pub fn test_day_5_part2() {
        assert_eq!(part_2(include_str!("input/day_5.txt")), "LLWJRBHVZ");
    }
}
