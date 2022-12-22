#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(usize)]
pub enum Dir {
    R = 0,
    D = 1,
    L = 2,
    U = 3,
}

impl Dir {
    fn apply(self, pos: (usize, usize)) -> (isize, isize) {
        let dir = [(0, 1), (1, 0), (0, -1), (-1, 0)][self as usize];
        let pos = (pos.0 as isize, pos.1 as isize);

        (pos.0 + dir.0, pos.1 + dir.1)
    }

    fn sym(self) -> char {
        match self {
            Dir::R => '>',
            Dir::D => 'V',
            Dir::L => '<',
            Dir::U => '^',
        }
    }

    fn turn_right(self) -> Dir {
        (self as usize + 1).rem_euclid(4).into()
    }
    fn turn_left(self) -> Dir {
        (((self as usize) as isize - 1).rem_euclid(4) as usize).into()
    }
}

impl From<usize> for Dir {
    fn from(v: usize) -> Dir {
        use Dir::*;
        match v {
            0 => R,
            1 => D,
            2 => L,
            3 => U,
            _ => unreachable!("{} is an invalid dir", v),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(usize)]
pub enum FaceId {
    A = 0,
    B = 1,
    C = 2,
    X = 3,
    Y = 4,
    Z = 5,
}

impl From<usize> for FaceId {
    fn from(v: usize) -> FaceId {
        use FaceId::*;
        match v {
            0 => A,
            1 => B,
            2 => C,
            3 => X,
            4 => Y,
            5 => Z,
            _ => unreachable!("{} is an invalid faceid", v),
        }
    }
}

struct Face {
    top_left: (usize, usize),
    grid: Vec<Vec<bool>>,
}

struct State {
    face: FaceId,
    faces: Vec<Face>,
    cube_len: isize,
    pos: (usize, usize),
    dir: Dir,
    history: Vec<Vec<char>>,
}

impl State {
    fn orig_pos(&self) -> (usize, usize) {
        let d = self.faces[self.face as usize].top_left;
        (self.pos.0 + d.0, self.pos.1 + d.1)
    }

    fn blocked(&self, face: FaceId, pos: (usize, usize)) -> bool {
        self.faces[face as usize].grid[pos.0][pos.1]
    }

    fn answer(&self) -> usize {
        let pos = self.orig_pos();
        1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + self.dir as usize
    }

    fn update(&mut self, new_face: FaceId, new_pos: (usize, usize), new_dir: Dir) {
        self.face = new_face;
        self.dir = new_dir;
        self.pos = new_pos;

        let o = self.orig_pos();
        self.history[o.0][o.1] = new_dir.sym();
    }
}

pub fn part_1(input: &str, cube_len: usize) -> usize {
    let (i1, _) = input.split_once("\n\n").unwrap();
    let (faces, _, face_lookup) = extract_faces(i1, cube_len);

    part_2(input, cube_len, |face_id, dir| {
        let c = faces[face_id as usize].top_left;
        let p = |v, x, l| (v as isize + x as isize).rem_euclid(l as isize) as usize;
        let m = |v, x, l| (v as isize - x as isize).rem_euclid(l as isize) as usize;

        let next_face_id = match dir {
            Dir::R => (1..=6)
                .map(|x| face_lookup[c.0][p(c.1, x * cube_len, face_lookup[c.0].len())]).find(|v| *v != 9)
                .unwrap(),
            Dir::L => (1..=6)
                .map(|x| face_lookup[c.0][m(c.1, x * cube_len, face_lookup[c.0].len())]).find(|v| *v != 9)
                .unwrap(),
            Dir::U => (1..=6)
                .map(|x| face_lookup[m(c.0, x * cube_len, face_lookup.len())][c.1]).find(|v| *v != 9)
                .unwrap(),
            Dir::D => (1..=6)
                .map(|x| face_lookup[p(c.0, x * cube_len, face_lookup.len())][c.1]).find(|v| *v != 9)
                .unwrap(),
        };

        (next_face_id.into(), dir)
    })
}

fn extract_faces(i1: &str, cube_len: usize) -> (Vec<Face>, Vec<Vec<char>>, Vec<Vec<usize>>) {
    let max_cols = i1.lines().map(|l| l.len()).max().unwrap();
    let max_rows = i1.lines().count();

    let mut map = vec![vec![' '; max_cols]; max_rows];
    for (r, l) in i1.lines().enumerate() {
        for (c, x) in l.chars().enumerate() {
            map[r][c] = x;
        }
    }

    // Find all six faces of the cube
    let mut face_lookup = vec![vec![9usize; max_cols]; max_rows];
    let mut face_id = 0;

    let mut faces = vec![];
    let mut stk = vec![(0, map[0].iter().position(|c| *c == '.').unwrap())];

    while let Some(top_left) = stk.pop() {
        if map[top_left.0][top_left.1] != ' ' {
            let mut grid = vec![vec![false; cube_len]; cube_len];

            for r in top_left.0..top_left.0 + cube_len {
                for c in top_left.1..top_left.1 + cube_len {
                    face_lookup[r][c] = face_id;
                    grid[r - top_left.0][c - top_left.1] = map[r][c] == '#';
                }
            }
            faces.push(Face { top_left, grid });
            face_id += 1;
            if top_left.0 >= cube_len && face_lookup[top_left.0 - cube_len][top_left.1] == 9 {
                stk.push((top_left.0 - cube_len, top_left.1));
            }
            if top_left.0 + cube_len < map.len()
                && face_lookup[top_left.0 + cube_len][top_left.1] == 9
            {
                stk.push((top_left.0 + cube_len, top_left.1));
            }
            if top_left.1 >= cube_len && face_lookup[top_left.0][top_left.1 - cube_len] == 9 {
                stk.push((top_left.0, top_left.1 - cube_len));
            }
            if top_left.1 + cube_len < map[top_left.0].len()
                && face_lookup[top_left.0][top_left.1 + cube_len] == 9
            {
                stk.push((top_left.0, top_left.1 + cube_len));
            }
        }
    }

    assert_eq!(faces.len(), 6);

    (faces, map, face_lookup)
}

pub fn part_2(
    input: &str,
    cube_len: usize,
    traverse: impl Fn(FaceId, Dir) -> (FaceId, Dir),
) -> usize {
    let (i1, i2) = input.split_once("\n\n").unwrap();
    let (faces, map, face_lookup) = extract_faces(i1, cube_len);

    let mut state = State {
        faces,
        face: FaceId::A,
        pos: (0, 0),
        dir: Dir::R,
        history: map,
        cube_len: cube_len as isize,
    };

    let mut dist = 0;
    for c in i2.trim().chars() {
        let d = if let Some(v) = c.to_digit(10) {
            dist = dist * 10 + v;
            0
        } else if c == 'L' || c == 'R' {
            let v = dist;
            dist = 0;
            v
        } else {
            unreachable!("'{:?}'", c);
        };

        part_2_move(&mut state, &traverse, d);

        if c == 'L' {
            state.dir = state.dir.turn_left();
        } else if c == 'R' {
            state.dir = state.dir.turn_right();
        }
    }
    part_2_move(&mut state, &traverse, dist);

    println!();
    for l in state.history.iter() {
        for x in l.iter().copied() {
            eprint!(
                "{}",
                if x == '.' {
                    ' '
                } else if x == ' ' {
                    '.'
                } else {
                    x
                }
            );
        }
        println!();
    }
    println!();

    println!();
    for (_i, r) in face_lookup.iter().enumerate() {
        for (_j, x) in r.iter().enumerate() {
            if *x == 9 {
                eprint!(" ");
            } else {
                eprint!("{:?}", FaceId::from(*x));
            }
        }

        println!();
    }
    println!();

    state.answer()
}

fn part_2_move(state: &mut State, traverse: &impl Fn(FaceId, Dir) -> (FaceId, Dir), d: u32) {
    for _ in 0..d {
        let next_pos = state.dir.apply(state.pos);
        let (next_face, next_dir, next_pos) = if next_pos.0 < 0
            || next_pos.0 >= state.cube_len
            || next_pos.1 < 0
            || next_pos.1 >= state.cube_len
        {
            let (next_face, next_dir) = traverse(state.face, state.dir);
            let x = (
                next_pos.0.rem_euclid(state.cube_len),
                next_pos.1.rem_euclid(state.cube_len),
            );

            use Dir::*;

            let np = match (state.dir, next_dir) {
                // Not rotated, so we dont need to muck with the coordinate system
                (R, R) | (D, D) | (L, L) | (U, U) => x,
                // Rotated clockwise
                (R, D) | (D, L) | (L, U) | (U, R) => (x.1, state.cube_len - 1 - x.0),

                // Rotated counter-clockwise
                (D, R) | (L, D) | (U, L) | (R, U) => (state.cube_len - 1 - x.1, x.0),
                // Flipped 180
                (D, U) | (U, D) | (L, R) | (R, L) => {
                    (state.cube_len - 1 - x.0, state.cube_len - 1 - x.1)
                }
            };
            let np = (
                np.0.rem_euclid(state.cube_len),
                np.1.rem_euclid(state.cube_len),
            );
            (next_face, next_dir, np)
        } else {
            (state.face, state.dir, next_pos)
        };

        let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

        if !state.blocked(next_face, next_pos) {
            state.update(next_face, next_pos, next_dir);
        } else {
            break;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::day_22::{part_1, part_2};

    const INPUTS: &str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;
    #[test]
    pub fn test_day_22_example_part1() {
        assert_eq!(part_1(INPUTS, 4), 6032);
    }

    #[test]
    pub fn test_day_22_part1() {
        assert_eq!(part_1(include_str!("input/day_22.txt"), 50), 36518);
    }

    #[test]
    pub fn test_day_22_example_part2() {
        use super::Dir::*;
        use super::FaceId::*;

        let t = [
            [(Z, L), (B, D), (C, D), (X, D)],
            [(Z, D), (Y, D), (C, L), (A, U)],
            [(B, R), (Y, R), (X, L), (A, R)],
            [(C, R), (Y, U), (Z, U), (A, D)],
            [(Z, R), (X, U), (C, U), (B, U)],
            [(A, L), (X, R), (Y, L), (B, L)],
        ];

        assert_eq!(
            part_2(INPUTS, 4, |face_id, dir| t[face_id as usize][dir as usize]),
            5031
        );
    }

    #[test]
    pub fn test_day_22_part2() {
        use super::Dir::*;
        use super::FaceId::*;
        let t = [
            [(B, R), (C, D), (Y, R), (Z, R)],
            [(X, L), (C, L), (A, L), (Z, U)],
            [(B, U), (X, D), (Y, D), (A, U)],
            [(B, L), (Z, L), (Y, L), (C, U)],
            [(X, R), (Z, D), (A, R), (C, R)],
            [(X, U), (B, D), (A, D), (Y, U)],
        ];
        assert_eq!(
            part_2(include_str!("input/day_22.txt"), 50, |face_id, dir| t
                [face_id as usize][dir as usize]),
            143208
        );
    }
}
