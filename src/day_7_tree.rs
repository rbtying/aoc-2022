use std::collections::HashMap;

// Option 1: ownership is with the overall filesystem. For simplicity, don't support removing files
#[derive(Debug)]
struct FS {
    // note: root is inode 0.
    files: Vec<FSDirEnt>,
    cwd: usize,
}

impl FS {
    fn new() -> Self {
        FS {
            cwd: 0,
            files: vec![FSDirEnt {
                parent_inode: 0,
                is_dir: true,
                size: 0,
                children: HashMap::new(),
            }],
        }
    }

    fn cd(&mut self, path_segment: &str) {
        self.cwd = match path_segment {
            ".." => self.files[self.cwd].parent_inode,
            "/" => 0,
            _ => match self.files[self.cwd].children.get(path_segment) {
                Some(existing_inode) => *existing_inode,
                None => self.add_item(path_segment, true, 0),
            },
        }
    }

    fn fstat(&self) -> &'_ FSDirEnt {
        &self.files[self.cwd]
    }

    fn add_item(&mut self, path_segment: &str, is_dir: bool, size: usize) -> usize {
        if let Some(inode) = self.files[self.cwd].children.get(path_segment) {
            return *inode;
        }
        let inode = self.files.len();
        self.files.push(FSDirEnt {
            parent_inode: self.cwd,
            is_dir,
            size,
            children: HashMap::new(),
        });
        self.files[self.cwd]
            .children
            .insert(path_segment.to_string(), inode);
        inode
    }

    fn compute_dir_sizes(&mut self) {
        let mut stk = vec![0];

        while let Some(inode) = stk.pop() {
            stk.extend(self.files[inode].children.values().copied());

            if !self.files[inode].is_dir {
                let mut update_ptr = self.files[inode].parent_inode;
                'inner: loop {
                    self.files[update_ptr].size += self.files[inode].size;

                    if update_ptr == 0 {
                        break 'inner;
                    }

                    update_ptr = self.files[update_ptr].parent_inode
                }
            }
        }
    }

    fn iter_root(&self) -> impl Iterator<Item = &'_ FSDirEnt> + '_ {
        FSIter {
            stk: vec![0],
            fs: self,
        }
    }
}

struct FSIter<'a> {
    stk: Vec<usize>,
    fs: &'a FS,
}

impl<'a> Iterator for FSIter<'a> {
    type Item = &'a FSDirEnt;

    fn next(&mut self) -> Option<&'a FSDirEnt> {
        self.stk.pop().map(|inode| {
            self.stk
                .extend(self.fs.files[inode].children.values().copied());
            &self.fs.files[inode]
        })
    }
}

#[derive(Debug)]
struct FSDirEnt {
    parent_inode: usize,
    is_dir: bool,
    size: usize,
    children: HashMap<String, usize>,
}

fn proc(input: &str) -> FS {
    let mut lines = input.lines().peekable();
    let mut fs = FS::new();

    loop {
        match lines.next() {
            Some(cmd) if cmd.starts_with('$') => {
                let mut cmd_iter = cmd.split_ascii_whitespace();
                assert_eq!(Some("$"), cmd_iter.next());

                match cmd_iter.next().unwrap() {
                    "cd" => fs.cd(cmd_iter.next().unwrap()),
                    "ls" => {
                        while !lines.peek().map(|l| l.starts_with('$')).unwrap_or(true) {
                            let mut l = lines.next().unwrap().split_ascii_whitespace();
                            match l.next().unwrap() {
                                "dir" => {
                                    fs.add_item(l.next().unwrap(), true, 0);
                                }
                                v => {
                                    fs.add_item(l.next().unwrap(), false, v.parse().unwrap());
                                }
                            }
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

    fs.compute_dir_sizes();

    fs
}

pub fn part_1(input: &str) -> usize {
    let fs = proc(input);
    fs.iter_root()
        .filter(|dirent| dirent.is_dir && dirent.size < 100000)
        .map(|dirent| dirent.size)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let mut fs = proc(input);
    fs.cd("/");

    let used_space = fs.fstat().size;
    let free_space = 70000000 - used_space;
    let needed_space = 30000000 - free_space;

    let mut options = fs
        .iter_root()
        .filter(|dirent| dirent.is_dir && dirent.size >= needed_space)
        .map(|dirent| dirent.size)
        .collect::<Vec<_>>();
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
    pub fn test_day_7_tree_example_part1() {
        assert_eq!(part_1(INPUTS), 95437);
    }

    #[test]
    pub fn test_day_7_tree_part1() {
        assert_eq!(part_1(include_str!("input/day_7.txt")), 1642503);
    }

    #[test]
    pub fn test_day_7_tree_example_part2() {
        assert_eq!(part_2(INPUTS), 24933642);
    }

    #[test]
    pub fn test_day_7_tree_part2() {
        assert_eq!(part_2(include_str!("input/day_7.txt")), 6999588);
    }
}
