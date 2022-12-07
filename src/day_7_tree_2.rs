use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

// Option 2: ownership is hierarchical
#[derive(Debug)]
struct DirEnt {
    parent: Option<Weak<RefCell<DirEnt>>>,
    is_dir: bool,
    size: usize,
    children: HashMap<String, Rc<RefCell<DirEnt>>>,
}

fn proc(input: &str) -> Rc<RefCell<DirEnt>> {
    let mut lines = input.lines().peekable();
    let root = Rc::new(RefCell::new(DirEnt {
        parent: None,
        is_dir: true,
        size: 0,
        children: HashMap::new(),
    }));

    let mut cwd = Rc::clone(&root);

    loop {
        match lines.next() {
            Some(cmd) if cmd.starts_with('$') => {
                let mut cmd_iter = cmd.split_ascii_whitespace();
                assert_eq!(Some("$"), cmd_iter.next());

                match cmd_iter.next().unwrap() {
                    "cd" => {
                        cwd = match cmd_iter.next().unwrap() {
                            "/" => Rc::clone(&root),
                            ".." => cwd
                                .borrow()
                                .parent
                                .as_ref()
                                .map(|r| r.upgrade().unwrap())
                                .unwrap_or_else(|| Rc::clone(&root)),
                            p => {
                                if let Some(child) = cwd.borrow().children.get(p) {
                                    Rc::clone(child)
                                } else {
                                    let new_node = Rc::new(RefCell::new(DirEnt {
                                        parent: Some(Rc::downgrade(&cwd)),
                                        is_dir: true,
                                        size: 0,
                                        children: HashMap::new(),
                                    }));
                                    cwd.borrow_mut()
                                        .children
                                        .insert(p.to_string(), Rc::clone(&new_node));
                                    new_node
                                }
                            }
                        };
                    }
                    "ls" => {
                        while !lines.peek().map(|l| l.starts_with('$')).unwrap_or(true) {
                            let mut l = lines.next().unwrap().split_ascii_whitespace();
                            match l.next().unwrap() {
                                "dir" => {
                                    let new_node = Rc::new(RefCell::new(DirEnt {
                                        parent: Some(Rc::downgrade(&cwd)),
                                        is_dir: true,
                                        size: 0,
                                        children: HashMap::new(),
                                    }));
                                    cwd.borrow_mut().children.insert(
                                        l.next().unwrap().to_string(),
                                        Rc::clone(&new_node),
                                    );
                                }
                                v => {
                                    let new_node = Rc::new(RefCell::new(DirEnt {
                                        parent: Some(Rc::downgrade(&cwd)),
                                        is_dir: false,
                                        size: v.parse().unwrap(),
                                        children: HashMap::new(),
                                    }));
                                    cwd.borrow_mut().children.insert(
                                        l.next().unwrap().to_string(),
                                        Rc::clone(&new_node),
                                    );
                                }
                            };
                        }
                    }
                    _ => unimplemented!("unrecognized command {:?}", cmd),
                }
            }
            Some(s) if s.is_empty() => continue,
            None => break,
            _ => unreachable!(),
        }
    }

    let mut stk = vec![Rc::clone(&root)];

    while let Some(n) = stk.pop() {
        let fstat = n.borrow();
        stk.extend(fstat.children.values().map(Rc::clone));

        if !fstat.is_dir {
            let size = fstat.size;
            let mut parent = fstat.parent.as_ref().and_then(|v| v.upgrade());

            while let Some(p) = parent {
                let mut pp = p.borrow_mut();
                pp.size += size;
                parent = pp.parent.as_ref().and_then(|v| v.upgrade());
            }
        }
    }

    root
}

pub fn part_1(input: &str) -> usize {
    let root = proc(input);

    let mut stk = vec![Rc::clone(&root)];
    let mut sum = 0;

    while let Some(n) = stk.pop() {
        let fstat = n.borrow();
        stk.extend(fstat.children.values().map(Rc::clone));

        if !fstat.is_dir {
            let size = fstat.size;
            if size < 100000 {
                sum += size
            }
        }
    }

    sum
}

pub fn part_2(input: &str) -> usize {
    let root = proc(input);

    let used_space = root.borrow().size;
    let free_space = 70000000 - used_space;
    let needed_space = 30000000 - free_space;

    let mut options = vec![];
    let mut stk = vec![Rc::clone(&root)];

    while let Some(n) = stk.pop() {
        let fstat = n.borrow();
        stk.extend(fstat.children.values().map(Rc::clone));

        if !fstat.is_dir {
            let size = fstat.size;
            if size > needed_space {
                options.push(size);
            }
        }
    }

    options.sort();
    options[0]
}

#[cfg(test)]
pub mod tests {
    use crate::day_7_tree::{part_1, part_2};

    const INPUTS: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    pub fn test_day_7_tree_2_example_part1() {
        assert_eq!(part_1(INPUTS), 95437);
    }

    #[test]
    pub fn test_day_7_tree_2_part1() {
        assert_eq!(part_1(include_str!("input/day_7.txt")), 1642503);
    }

    #[test]
    pub fn test_day_7_tree_2_example_part2() {
        assert_eq!(part_2(INPUTS), 24933642);
    }

    #[test]
    pub fn test_day_7_tree_2_part2() {
        assert_eq!(part_2(include_str!("input/day_7.txt")), 6999588);
    }
}
