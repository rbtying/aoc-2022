// --- Day 18: Boiling Boulders ---

// You and the elephants finally reach fresh air. You've emerged near the base
// of a large volcano that seems to be actively erupting! Fortunately, the lava
// seems to be flowing away from you and toward the ocean.

// Bits of lava are still being ejected toward you, so you're sheltering in the
// cavern exit a little longer. Outside the cave, you can see the lava landing
// in a pond and hear it loudly hissing as it solidifies.

// Depending on the specific compounds in the lava and speed at which it cools,
// it might be forming obsidian! The cooling rate should be based on the surface
// area of the lava droplets, so you take a quick scan of a droplet as it flies
// past you (your puzzle input).

// Because of how quickly the lava is moving, the scan isn't very good; its
// resolution is quite low and, as a result, it approximates the shape of the
// lava droplet with 1x1x1 cubes on a 3D grid, each given as its x,y,z position.

// To approximate the surface area, count the number of sides of each cube that
// are not immediately connected to another cube. So, if your scan were only two
// adjacent cubes like 1,1,1 and 2,1,1, each cube would have a single side
// covered and five sides exposed, a total surface area of 10 sides.

// Here's a larger example:

// 2,2,2
// 1,2,2
// 3,2,2
// 2,1,2
// 2,3,2
// 2,2,1
// 2,2,3
// 2,2,4
// 2,2,6
// 1,2,5
// 3,2,5
// 2,1,5
// 2,3,5

// In the above example, after counting up all the sides that aren't connected
// to another cube, the total surface area is 64.

// What is the surface area of your scanned lava droplet?

// --- Part Two ---

// Something seems off about your calculation. The cooling rate depends on
// exterior surface area, but your calculation also included the surface area of
// air pockets trapped in the lava droplet.

// Instead, consider only cube sides that could be reached by the water and
// steam as the lava droplet tumbles into the pond. The steam will expand to
// reach as much as possible, completely displacing any air on the outside of
// the lava droplet but never expanding diagonally.

// In the larger example above, exactly one cube of air is trapped within the
// lava droplet (at 2,2,5), so the exterior surface area of the lava droplet is
// 58.

// What is the exterior surface area of your scanned lava droplet?

use std::collections::HashSet;

fn parse(input: &str) -> HashSet<(i64, i64, i64)> {
    let mut pts: HashSet<(i64, i64, i64)> = HashSet::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut pt = line.split(',').map(|v| v.parse().unwrap());
        pts.insert((pt.next().unwrap(), pt.next().unwrap(), pt.next().unwrap()));
    }

    pts
}

pub fn part_1(input: &str) -> usize {
    let pts = parse(input);

    pts.iter()
        .copied()
        .flat_map(adjacents)
        .filter(|p| !pts.contains(p))
        .count()
}

fn adjacents((x, y, z): (i64, i64, i64)) -> [(i64, i64, i64); 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

pub fn part_2(input: &str) -> usize {
    let pts = parse(input);

    let max_x = pts.iter().map(|p| p.0).max().unwrap() + 2;
    let max_y = pts.iter().map(|p| p.1).max().unwrap() + 2;
    let max_z = pts.iter().map(|p| p.2).max().unwrap() + 2;

    // Flood-fill from (-1, -1, -1), which we know is not actually in the set.
    let mut visited = HashSet::new();
    let mut q = vec![(-1, -1, -1)];

    let reachable = |v: (i64, i64, i64)| {
        let in_max_bounds = v.0 < max_x && v.1 < max_y && v.2 < max_z;
        let in_min_bounds = v.0 > -2 && v.1 > -2 && v.2 > -2;
        let occupied = pts.contains(&v);
        in_max_bounds && in_min_bounds && !occupied
    };

    while let Some(v) = q.pop() {
        visited.insert(v);

        for vv in adjacents(v) {
            if reachable(vv) && !visited.contains(&vv) {
                q.push(vv);
            }
        }
    }

    pts.into_iter()
        .flat_map(adjacents)
        .filter(|p| visited.contains(p))
        .count()
}

#[cfg(test)]
pub mod tests {
    use crate::day_18::{part_1, part_2};

    const INPUTS: &str = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;

    #[test]
    pub fn test_day_18_example_part1() {
        assert_eq!(part_1(INPUTS), 64);
    }

    #[test]
    pub fn test_day_18_part1() {
        assert_eq!(part_1(include_str!("input/day_18.txt")), 3412);
    }

    #[test]
    pub fn test_day_18_example_part2() {
        assert_eq!(part_2(INPUTS), 58);
    }

    #[test]
    pub fn test_day_18_part2() {
        assert_eq!(part_2(include_str!("input/day_18.txt")), 2018);
    }
}
