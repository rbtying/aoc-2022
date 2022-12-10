// The day 10 prompt's examples are too long, see day_10_prompt.txt

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Add(isize),
    Noop,
}

struct Cpu {
    x_history: Vec<isize>,
}

impl Cpu {
    fn new() -> Self {
        Cpu { x_history: vec![1] }
    }

    fn exec(&mut self, ins: Instruction) {
        let x = *self.x_history.last().unwrap();
        match ins {
            Instruction::Noop => self.x_history.push(x),
            Instruction::Add(v) => {
                self.x_history.push(x);
                self.x_history.push(x + v);
            }
        }
    }
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().filter(|l| !l.is_empty()).map(|l| {
        let mut iter = l.split_ascii_whitespace();
        match iter.next().unwrap() {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Add(iter.next().unwrap().parse().unwrap()),
            _ => unreachable!(),
        }
    })
}

pub fn part_1(input: &str) -> isize {
    let mut cpu = Cpu::new();

    for instruction in parse_instructions(input) {
        cpu.exec(instruction);
    }

    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|idx| cpu.x_history[idx - 1] * idx as isize)
        .sum()
}

pub fn part_2(input: &str) -> String {
    let mut cpu = Cpu::new();

    for instruction in parse_instructions(input) {
        cpu.exec(instruction);
    }

    let mut s = String::new();
    s.push('\n');

    for row in 0..6 {
        let start_idx = row * 40;
        for offset in 0..40 {
            let x = cpu.x_history[start_idx + offset];
            if [x - 1, x, x + 1].contains(&(offset as isize)) {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }
    s
}

#[cfg(test)]
pub mod tests {
    use crate::day_10::{part_1, part_2};

    const INPUTS: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    pub fn test_day_10_example_part1() {
        assert_eq!(part_1(INPUTS), 13140);
    }

    #[test]
    pub fn test_day_10_part1() {
        assert_eq!(part_1(include_str!("input/day_10.txt")), 15680);
    }

    #[test]
    pub fn test_day_10_example_part2() {
        assert_eq!(
            part_2(INPUTS),
            r#"
##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     
"#
        );
    }

    #[test]
    pub fn test_day_10_part2() {
        assert_eq!(
            part_2(include_str!("input/day_10.txt")),
            r#"
#### #### ###  #### #  #  ##  #  # ###  
   # #    #  # #    #  # #  # #  # #  # 
  #  ###  ###  ###  #### #    #  # #  # 
 #   #    #  # #    #  # # ## #  # ###  
#    #    #  # #    #  # #  # #  # #    
#### #    ###  #    #  #  ###  ##  #    
"#
        );
    }
}
