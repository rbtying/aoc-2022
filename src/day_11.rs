// The day 11 prompt's examples are too long, see day_11_prompt.txt
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Op {
    Immediate(u64),
    Old,
    Mul(Vec<Op>),
    Add(Vec<Op>),
}

impl Op {
    fn parse_operand(operand: &str) -> Op {
        if operand == "old" {
            Op::Old
        } else {
            Op::Immediate(operand.parse().unwrap())
        }
    }

    fn eval(&self, old: u64) -> u64 {
        match self {
            Op::Immediate(v) => *v,
            Op::Old => old,
            Op::Mul(operands) => operands.iter().map(|o| o.eval(old)).product(),
            Op::Add(operands) => operands.iter().map(|o| o.eval(old)).sum(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Test {
    test_divisor: u64,
    true_dest: usize,
    false_dest: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Op,
    test: Test,
    inspect_count: usize,
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|l| {
            let mut iter = l.lines();
            iter.next().unwrap();

            let items = {
                let items_list = iter.next().unwrap();
                items_list
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split(", ")
                    .flat_map(|it| it.parse())
                    .collect()
            };

            let op = {
                let op_str = iter.next().unwrap().split_once("new = ").unwrap().1;
                let mut op_iter = op_str.split_whitespace();
                let lhs = Op::parse_operand(op_iter.next().unwrap());
                let op_typ = op_iter.next().unwrap();
                let rhs = Op::parse_operand(op_iter.next().unwrap());

                match op_typ {
                    "*" => Op::Mul(vec![lhs, rhs]),
                    "+" => Op::Add(vec![lhs, rhs]),
                    _ => unreachable!(),
                }
            };

            let test = {
                let test_divisor = iter
                    .next()
                    .unwrap()
                    .split_once("divisible by ")
                    .unwrap()
                    .1
                    .parse()
                    .unwrap();
                let true_dest = iter
                    .next()
                    .unwrap()
                    .split_once("monkey ")
                    .unwrap()
                    .1
                    .parse()
                    .unwrap();
                let false_dest = iter
                    .next()
                    .unwrap()
                    .split_once("monkey ")
                    .unwrap()
                    .1
                    .parse()
                    .unwrap();

                Test {
                    test_divisor,
                    true_dest,
                    false_dest,
                }
            };

            Monkey {
                inspect_count: 0,
                items,
                test,
                operation: op,
            }
        })
        .collect()
}

pub fn part_1(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);

    for _round in 0..20 {
        for idx in 0..monkeys.len() {
            for item in std::mem::take(&mut monkeys[idx].items) {
                let new = monkeys[idx].operation.eval(item);
                monkeys[idx].inspect_count += 1;

                let item_ = new / 3;

                let dst = if item_ % monkeys[idx].test.test_divisor == 0 {
                    monkeys[idx].test.true_dest
                } else {
                    monkeys[idx].test.false_dest
                };
                monkeys[dst].items.push(item_);
            }
        }
    }

    let mut cts = monkeys.iter().map(|m| m.inspect_count).collect::<Vec<_>>();
    cts.sort();

    cts[cts.len() - 1] * cts[cts.len() - 2]
}

pub fn part_2(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);

    // All of the tests are modulus, so we can operate in the modular group
    // which contains all of the test moduluses. Math hax :(
    let modulus: u64 = monkeys
        .iter()
        .map(|m| m.test.test_divisor)
        .collect::<HashSet<_>>()
        .iter()
        .product();

    for _round in 0..10000 {
        for idx in 0..monkeys.len() {
            for item in std::mem::take(&mut monkeys[idx].items) {
                let new = monkeys[idx].operation.eval(item);
                monkeys[idx].inspect_count += 1;

                let item_ = new % modulus;

                let dst = if item_ % monkeys[idx].test.test_divisor == 0 {
                    monkeys[idx].test.true_dest
                } else {
                    monkeys[idx].test.false_dest
                };
                monkeys[dst].items.push(item_);
            }
        }
    }

    let mut cts = monkeys.iter().map(|m| m.inspect_count).collect::<Vec<_>>();
    cts.sort();

    cts[cts.len() - 1] * cts[cts.len() - 2]
}

#[cfg(test)]
pub mod tests {
    use crate::day_11::{part_1, part_2};

    const INPUTS: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    #[test]
    pub fn test_day_11_example_part1() {
        assert_eq!(part_1(INPUTS), 10605);
    }

    #[test]
    pub fn test_day_11_part1() {
        assert_eq!(part_1(include_str!("input/day_11.txt")), 50172);
    }

    #[test]
    pub fn test_day_11_example_part2() {
        assert_eq!(part_2(INPUTS), 2713310158);
    }

    #[test]
    pub fn test_day_11_part2() {
        assert_eq!(part_2(include_str!("input/day_11.txt")), 11614682178);
    }
}
