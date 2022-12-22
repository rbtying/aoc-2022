// --- Day 22: Monkey Map ---
//
// The monkeys take you on a surprisingly easy trail through the jungle. They're even going in
// roughly the right direction according to your handheld device's Grove Positioning System.

// As you walk, the monkeys explain that the grove is protected by a force field. To pass through
// the force field, you have to enter a password; doing so involves tracing a specific path on a
// strangely-shaped board.

// At least, you're pretty sure that's what you have to do; the elephants aren't exactly fluent in
// monkey.

// The monkeys give you notes that they took when they last saw the password entered (your puzzle
// input).

// For example:

//         ...#
//         .#..
//         #...
//         ....
// ...#.......#
// ........#...
// ..#....#....
// ..........#.
//         ...#....
//         .....#..
//         .#......
//         ......#.

// 10R5L5R10L4R5L5
//
// The first half of the monkeys' notes is a map of the board. It is comprised of a set of open
// tiles (on which you can move, drawn .) and solid walls (tiles which you cannot enter, drawn #).

// The second half is a description of the path you must follow. It consists of alternating numbers
// and letters:

// A number indicates the number of tiles to move in the direction you are facing. If you run into
// a wall, you stop moving forward and continue with the next instruction.
//
// A letter indicates whether to turn 90 degrees clockwise (R) or counterclockwise (L). Turning
// happens in-place; it does not change your current tile.
//
// So, a path like 10R5 means "go forward 10 tiles, then turn clockwise 90 degrees, then go forward
// 5 tiles".

// You begin the path in the leftmost open tile of the top row of tiles. Initially, you are facing
// to the right (from the perspective of how the map is drawn).

// If a movement instruction would take you off of the map, you wrap around to the other side of
// the board. In other words, if your next tile is off of the board, you should instead look in the
// direction opposite of your current facing as far as you can until you find the opposite edge of
// the board, then reappear there.

// For example, if you are at A and facing to the right, the tile in front of you is marked B; if
// you are at C and facing down, the tile in front of you is marked D:

//         ...#
//         .#..
//         #...
//         ....
// ...#.D.....#
// ........#...
// B.#....#...A
// .....C....#.
//         ...#....
//         .....#..
//         .#......
//         ......#.
//
// It is possible for the next tile (after wrapping around) to be a wall; this still counts as
// there being a wall in front of you, and so movement stops before you actually wrap to the other
// side of the board.

// By drawing the last facing you had with an arrow on each tile you visit, the full path taken by
// the above example looks like this:

//         >>v#
//         .#v.
//         #.v.
//         ..v.
// ...#...v..v#
// >>>v...>#.>>
// ..#v...#....
// ...>>>>v..#.
//         ...#....
//         .....#..
//         .#......
//         ......#.
//
// To finish providing the password to this strange input device, you need to determine numbers for
// your final row, column, and facing as your final position appears from the perspective of the
// original map. Rows start from 1 at the top and count downward; columns start from 1 at the left
// and count rightward. (In the above example, row 1, column 1 refers to the empty space with no
// tile on it in the top-left corner.) Facing is 0 for right (>), 1 for down (v), 2 for left (<),
// and 3 for up (^). The final password is the sum of 1000 times the row, 4 times the column, and
// the facing.

// In the above example, the final row is 6, the final column is 8, and the final facing is 0. So,
// the final password is 1000 * 6 + 4 * 8 + 0: 6032.

// Follow the path given in the monkeys' notes. What is the final password?

// --- Part Two ---
//
// As you reach the force field, you think you hear some Elves in the distance. Perhaps they've
// already arrived?

// You approach the strange input device, but it isn't quite what the monkeys drew in their notes.
// Instead, you are met with a large cube; each of its six faces is a square of 50x50 tiles.

// To be fair, the monkeys' map does have six 50x50 regions on it. If you were to carefully fold
// the map, you should be able to shape it into a cube!

// In the example above, the six (smaller, 4x4) faces of the cube are:

//         1111
//         1111
//         1111
//         1111
// 222233334444
// 222233334444
// 222233334444
// 222233334444
//         55556666
//         55556666
//         55556666
//         55556666
//
// You still start in the same position and with the same facing as before, but the wrapping rules
// are different. Now, if you would walk off the board, you instead proceed around the cube. From
// the perspective of the map, this can look a little strange. In the above example, if you are at
// A and move to the right, you would arrive at B facing down; if you are at C and move down, you
// would arrive at D facing up:

//         ...#
//         .#..
//         #...
//         ....
// ...#.......#
// ........#..A
// ..#....#....
// .D........#.
//         ...#..B.
//         .....#..
//         .#......
//         ..C...#.
//
// Walls still block your path, even if they are on a different face of the cube. If you are at E
// facing up, your movement is blocked by the wall marked by the arrow:

//         ...#
//         .#..
//      -->#...
//         ....
// ...#..E....#
// ........#...
// ..#....#....
// ..........#.
//         ...#....
//         .....#..
//         .#......
//         ......#.
//
// Using the same method of drawing the last facing you had with an arrow on each tile you visit,
// the full path taken by the above example now looks like this:

//         >>v#
//         .#v.
//         #.v.
//         ..v.
// ...#..^...v#
// .>>>>>^.#.>>
// .^#....#....
// .^........#.
//         ...#..v.
//         .....#v.
//         .#v<<<<.
//         ..v...#.
//
// The final password is still calculated from your final position and facing from the perspective
// of the map. In this example, the final row is 5, the final column is 7, and the final facing is
// 3, so the final password is 1000 * 5 + 4 * 7 + 3 = 5031.

// Fold the map into a cube, then follow the path given in the monkeys' notes. What is the final
// password?

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
            Dir::R => '→',
            Dir::D => '↓',
            Dir::L => '←',
            Dir::U => '↑',
        }
    }

    /// Rotate self and begin until begin == end
    fn match_rot(self, mut begin: Dir, end: Dir) -> Dir {
        let mut res = self;
        while begin != end {
            res = res.turn_left();
            begin = begin.turn_left();
        }
        res
    }

    fn reverse(self) -> Dir {
        (self as usize + 2).rem_euclid(4).into()
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

#[derive(Clone)]
struct Face {
    top_left: (usize, usize),
    grid: Vec<Vec<bool>>,
}

struct State<'a, 'b> {
    face: FaceId,
    faces: &'a [Face],
    cube_len: isize,
    pos: (usize, usize),
    dir: Dir,
    history: &'b mut Vec<Vec<char>>,
}

impl<'a, 'b> State<'a, 'b> {
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

fn solve(
    i2: &str,
    faces: &[Face],
    map: &mut Vec<Vec<char>>,
    face_lookup: &[Vec<usize>],
    cube_len: usize,
    traverse: impl Fn(FaceId, Dir) -> (FaceId, Dir),
) -> usize {
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

        mv(&mut state, &traverse, d);

        if c == 'L' {
            state.dir = state.dir.turn_left();
        } else if c == 'R' {
            state.dir = state.dir.turn_right();
        }
    }
    mv(&mut state, &traverse, dist);

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

pub fn part_1(input: &str, cube_len: usize) -> usize {
    let (i1, i2) = input.split_once("\n\n").unwrap();
    let (faces, mut map, face_lookup) = extract_faces(i1, cube_len);
    let t = |face_id, dir| {
        let c = faces[face_id as usize].top_left;
        let p = |v, x, l| (v as isize + x as isize).rem_euclid(l as isize) as usize;
        let m = |v, x, l| (v as isize - x as isize).rem_euclid(l as isize) as usize;

        let next_face_id = match dir {
            Dir::R => (1..=6)
                .map(|x| face_lookup[c.0][p(c.1, x * cube_len, face_lookup[c.0].len())])
                .find(|v| *v != 9)
                .unwrap(),
            Dir::L => (1..=6)
                .map(|x| face_lookup[c.0][m(c.1, x * cube_len, face_lookup[c.0].len())])
                .find(|v| *v != 9)
                .unwrap(),
            Dir::U => (1..=6)
                .map(|x| face_lookup[m(c.0, x * cube_len, face_lookup.len())][c.1])
                .find(|v| *v != 9)
                .unwrap(),
            Dir::D => (1..=6)
                .map(|x| face_lookup[p(c.0, x * cube_len, face_lookup.len())][c.1])
                .find(|v| *v != 9)
                .unwrap(),
        };

        (next_face_id.into(), dir)
    };

    solve(i2, &faces, &mut map, &face_lookup, cube_len, t)
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

fn compute_traversals(face_lookup: &[Vec<usize>]) -> [[(FaceId, Dir); 4]; 6] {
    let mut tab = [[None; 4]; 6];
    use Dir::*;
    use FaceId::*;

    for (r, row) in face_lookup.iter().enumerate().skip(1) {
        for (c, f) in row.iter().copied().enumerate().skip(1) {
            let p_c = face_lookup[r][c - 1];
            let p_r = face_lookup[r - 1][c];
            if p_c != f && p_c != 9 && f != 9 {
                tab[p_c][R as usize] = Some((FaceId::from(f), R));
                tab[f][L as usize] = Some((FaceId::from(p_c), L));
            }
            if p_r != f && p_r != 9 && f != 9 {
                tab[p_r][D as usize] = Some((FaceId::from(f), D));
                tab[f][U as usize] = Some((FaceId::from(p_r), U));
            }
        }
    }

    while tab.iter().any(|l| l.iter().any(|v| v.is_none())) {
        for a in [A, B, C, X, Y, Z] {
            for dir in [R, D, L, U] {
                if let Some((b, bd)) = tab[a as usize][dir as usize] {
                    let dir2 = dir.turn_right();
                    if let Some((c, cd)) = tab[a as usize][dir2 as usize] {
                        let b_out = dir2.match_rot(dir, bd);
                        let c_out = dir.match_rot(dir2, cd);
                        tab[b as usize][b_out as usize] = Some((c, c_out.reverse()));
                        tab[c as usize][c_out as usize] = Some((b, b_out.reverse()));
                    }
                }
            }
        }
    }

    let mut tab2 = [[(A, D); 4]; 6];

    for (idx, l) in tab.into_iter().enumerate() {
        for (idx2, d) in l.into_iter().enumerate() {
            tab2[idx][idx2] = d.unwrap();
        }
    }

    tab2
}

pub fn part_2(input: &str, cube_len: usize) -> usize {
    let (i1, i2) = input.split_once("\n\n").unwrap();
    let (faces, mut map, face_lookup) = extract_faces(i1, cube_len);

    let t = compute_traversals(&face_lookup);
    let t2 = |face_id, dir| t[face_id as usize][dir as usize];

    solve(i2, &faces, &mut map, &face_lookup, cube_len, t2)
}

fn mv<'a, 'b>(state: &mut State<'a, 'b>, traverse: &impl Fn(FaceId, Dir) -> (FaceId, Dir), d: u32) {
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
        assert_eq!(part_2(INPUTS, 4), 5031);
    }

    #[test]
    pub fn test_day_22_part2() {
        assert_eq!(part_2(include_str!("input/day_22.txt"), 50), 143208);
    }
}
