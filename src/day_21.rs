// --- Day 21: Monkey Math ---

// The monkeys are back! You're worried they're going to try to steal your stuff
// again, but it seems like they're just holding their ground and making various
// monkey noises at you.

// Eventually, one of the elephants realizes you don't speak monkey and comes
// over to interpret. As it turns out, they overheard you talking about trying
// to find the grove; they can show you a shortcut if you answer their riddle.

// Each monkey is given a job: either to yell a specific number or to yell the
// result of a math operation. All of the number-yelling monkeys know their
// number from the start; however, the math operation monkeys need to wait for
// two other monkeys to yell a number, and those two other monkeys might also be
// waiting on other monkeys.

// Your job is to work out the number the monkey named root will yell before the
// monkeys figure it out themselves.

// For example:

// root: pppw + sjmn
// dbpl: 5
// cczh: sllz + lgvd
// zczc: 2
// ptdq: humn - dvpt
// dvpt: 3
// lfqf: 4
// humn: 5
// ljgn: 2
// sjmn: drzm * dbpl
// sllz: 4
// pppw: cczh / lfqf
// lgvd: ljgn * ptdq
// drzm: hmdt - zczc
// hmdt: 32

// Each line contains the name of a monkey, a colon, and then the job of that monkey:

// A lone number means the monkey's job is simply to yell that number.

// A job like aaaa + bbbb means the monkey waits for monkeys aaaa and bbbb to
// yell each of their numbers; the monkey then yells the sum of those two
// numbers.

// aaaa - bbbb means the monkey yells aaaa's number minus bbbb's number.

// Job aaaa * bbbb will yell aaaa's number multiplied by bbbb's number.

// Job aaaa / bbbb will yell aaaa's number divided by bbbb's number.

// So, in the above example, monkey drzm has to wait for monkeys hmdt and zczc
// to yell their numbers. Fortunately, both hmdt and zczc have jobs that involve
// simply yelling a single number, so they do this immediately: 32 and 2. Monkey
// drzm can then yell its number by finding 32 minus 2: 30.

// Then, monkey sjmn has one of its numbers (30, from monkey drzm), and already
// has its other number, 5, from dbpl. This allows it to yell its own number by
// finding 30 multiplied by 5: 150.

// This process continues until root yells a number: 152.

// However, your actual situation involves considerably more monkeys. What
// number will the monkey named root yell?

// --- Part Two ---

// Due to some kind of monkey-elephant-human mistranslation, you seem to have
// misunderstood a few key details about the riddle.

// First, you got the wrong job for the monkey named root; specifically, you got
// the wrong math operation. The correct operation for monkey root should be =,
// which means that it still listens for two numbers (from the same two monkeys
// as before), but now checks that the two numbers match.

// Second, you got the wrong monkey for the job starting with humn:. It isn't a
// monkey - it's you. Actually, you got the job wrong, too: you need to figure
// out what number you need to yell so that root's equality check passes. (The
// number that appears after humn: in your input is now irrelevant.)

// In the above example, the number you need to yell to pass root's equality
// test is 301. (This causes root to get the same number, 150, from both of its
// monkeys.)

// What number do you yell to pass root's equality test?

use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Expr {
    Immediate(i64),
    Add([u8; 4], [u8; 4]),
    Sub([u8; 4], [u8; 4]),
    Mul([u8; 4], [u8; 4]),
    Div([u8; 4], [u8; 4]),
    Eq([u8; 4], [u8; 4]),
    Var,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Monkey {
    name: [u8; 4],
    job: Expr,
}

fn name(s: &str) -> [u8; 4] {
    let mut n = [0; 4];
    assert_eq!(s.len(), 4);

    n.copy_from_slice(s.as_bytes());

    n
}

fn compute(monkeys: &HashMap<[u8; 4], Monkey>, name: &[u8; 4]) -> Option<i64> {
    let monkey = monkeys[name];

    match monkey.job {
        Expr::Immediate(n) => Some(n),
        Expr::Add(a, b) => Some(compute(monkeys, &a)? + compute(monkeys, &b)?),
        Expr::Sub(a, b) => Some(compute(monkeys, &a)? - compute(monkeys, &b)?),
        Expr::Mul(a, b) => Some(compute(monkeys, &a)? * compute(monkeys, &b)?),
        Expr::Div(a, b) => Some(compute(monkeys, &a)? / compute(monkeys, &b)?),
        Expr::Eq(a, b) => Some((compute(monkeys, &a)? == compute(monkeys, &b)?) as i64),
        Expr::Var => None,
    }
}

fn extract(expr_a: Expr, expr_b: Expr) -> (Expr, i64, bool) {
    match (expr_a, expr_b) {
        (Expr::Immediate(v), _) => (expr_b, v, true),
        (_, Expr::Immediate(v)) => (expr_a, v, false),
        _ => unreachable!("Example input only has x in one location"),
    }
}

pub fn part_1(input: &str) -> i64 {
    let monkeys = parse(input);
    compute(&monkeys, b"root").unwrap()
}

pub fn part_2(input: &str) -> i64 {
    let mut monkeys = parse(input);
    let root = monkeys.get_mut(b"root").unwrap();
    root.job = match root.job {
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => Expr::Eq(a, b),
        _ => unreachable!(),
    };
    let humn = monkeys.get_mut(b"humn").unwrap();
    humn.job = Expr::Var;

    // Start by simplifying everything which doesn't reference `humn`.
    let names = monkeys.keys().copied().collect::<Vec<_>>();
    for n in &names {
        if let Expr::Immediate(_) = monkeys[n].job {
            continue;
        }
        if let Some(v) = compute(&monkeys, n) {
            monkeys.get_mut(n).unwrap().job = Expr::Immediate(v);
        }
    }

    // Now, actually try to solve the root equation
    let (a, b) = if let Expr::Eq(a, b) = monkeys[b"root"].job {
        (a, b)
    } else {
        unreachable!()
    };

    // In practice, since we only have one monkey which depends on the human, we
    // know that one of the two values is an immediate, and the other is an
    // expression.
    let (mut expr, mut v, _) = extract(monkeys[&a].job, monkeys[&b].job);

    loop {
        // This is true recursively, so just unroll the expression until we get
        // the answer.
        match expr {
            Expr::Add(a, b) => {
                let (expr_, v_, _) = extract(monkeys[&a].job, monkeys[&b].job);
                expr = expr_;

                // a + b = v
                //     a = v - b
                v -= v_;
            }
            Expr::Mul(a, b) => {
                let (expr_, v_, _) = extract(monkeys[&a].job, monkeys[&b].job);
                expr = expr_;

                // a * b = v
                //     a = v / b
                v /= v_;
            }
            Expr::Sub(a, b) => {
                let (expr_, v_, flipped) = extract(monkeys[&a].job, monkeys[&b].job);
                expr = expr_;

                if flipped {
                    // b - a = v => a - b = -v
                    v = -v;
                }
                // a - b = v
                //     a = v + b
                v += v_;
            }
            Expr::Div(a, b) => {
                let (expr_, v_, flipped) = extract(monkeys[&a].job, monkeys[&b].job);
                expr = expr_;

                if flipped {
                    // b / a = v
                    //     b = v * a
                    //     a = b / v
                    v = v_ / v;
                } else {
                    // a / b = v
                    //     a = v * b
                    v *= v_;
                }
            }
            Expr::Var => return v,
            Expr::Immediate(_) | Expr::Eq(_, _) => unreachable!(),
        }
    }
}

fn parse(input: &str) -> HashMap<[u8; 4], Monkey> {
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let (monkey_name, op) = line.split_once(": ").unwrap();
        let job = match op.parse::<i64>() {
            Ok(v) => Expr::Immediate(v),
            Err(_) => {
                let mut iter = op.split_whitespace();
                let n1 = name(iter.next().unwrap());
                let o = iter.next().unwrap();
                let n2 = name(iter.next().unwrap());

                match o {
                    "+" => Expr::Add(n1, n2),
                    "-" => Expr::Sub(n1, n2),
                    "/" => Expr::Div(n1, n2),
                    "*" => Expr::Mul(n1, n2),
                    _ => unreachable!("Unexpected {:?}", o),
                }
            }
        };

        let n = name(monkey_name);

        monkeys.insert(n, Monkey { name: n, job });
    }

    monkeys
}

#[cfg(test)]
pub mod tests {
    use crate::day_21::{part_1, part_2};

    const INPUTS: &str = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;
    #[test]
    pub fn test_day_21_example_part1() {
        assert_eq!(part_1(INPUTS), 152);
    }

    #[test]
    pub fn test_day_21_part1() {
        assert_eq!(part_1(include_str!("input/day_21.txt")), 83056452926300);
    }

    #[test]
    pub fn test_day_21_example_part2() {
        assert_eq!(part_2(INPUTS), 301);
    }

    #[test]
    pub fn test_day_21_part2() {
        assert_eq!(part_2(include_str!("input/day_21.txt")), 3469704905529);
    }
}
