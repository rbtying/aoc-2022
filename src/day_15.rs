// --- Day 15: Beacon Exclusion Zone ---

// You feel the ground rumble again as the distress signal leads you to a large
// network of subterranean tunnels. You don't have time to search them all, but
// you don't need to: your pack contains a set of deployable sensors that you
// imagine were originally built to locate lost Elves.

// The sensors aren't very powerful, but that's okay; your handheld device
// indicates that you're close enough to the source of the distress signal to
// use them. You pull the emergency sensor system out of your pack, hit the big
// button on top, and the sensors zoom off down the tunnels.

// Once a sensor finds a spot it thinks will give it a good reading, it attaches
// itself to a hard surface and begins monitoring for the nearest signal source
// beacon. Sensors and beacons always exist at integer coordinates. Each sensor
// knows its own position and can determine the position of a beacon precisely;
// however, sensors can only lock on to the one beacon closest to the sensor as
// measured by the Manhattan distance. (There is never a tie where two beacons
// are the same distance to a sensor.)

// It doesn't take long for the sensors to report back their positions and
// closest beacons (your puzzle input). For example:

// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
// Sensor at x=9, y=16: closest beacon is at x=10, y=16
// Sensor at x=13, y=2: closest beacon is at x=15, y=3
// Sensor at x=12, y=14: closest beacon is at x=10, y=16
// Sensor at x=10, y=20: closest beacon is at x=10, y=16
// Sensor at x=14, y=17: closest beacon is at x=10, y=16
// Sensor at x=8, y=7: closest beacon is at x=2, y=10
// Sensor at x=2, y=0: closest beacon is at x=2, y=10
// Sensor at x=0, y=11: closest beacon is at x=2, y=10
// Sensor at x=20, y=14: closest beacon is at x=25, y=17
// Sensor at x=17, y=20: closest beacon is at x=21, y=22
// Sensor at x=16, y=7: closest beacon is at x=15, y=3
// Sensor at x=14, y=3: closest beacon is at x=15, y=3
// Sensor at x=20, y=1: closest beacon is at x=15, y=3

// So, consider the sensor at 2,18; the closest beacon to it is at -2,15. For
// the sensor at 9,16, the closest beacon to it is at 10,16.

// Drawing sensors as S and beacons as B, the above arrangement of sensors and
// beacons looks like this:

//                1    1    2    2
//      0    5    0    5    0    5
//  0 ....S.......................
//  1 ......................S.....
//  2 ...............S............
//  3 ................SB..........
//  4 ............................
//  5 ............................
//  6 ............................
//  7 ..........S.......S.........
//  8 ............................
//  9 ............................
// 10 ....B.......................
// 11 ..S.........................
// 12 ............................
// 13 ............................
// 14 ..............S.......S.....
// 15 B...........................
// 16 ...........SB...............
// 17 ................S..........B
// 18 ....S.......................
// 19 ............................
// 20 ............S......S........
// 21 ............................
// 22 .......................B....

// This isn't necessarily a comprehensive map of all beacons in the area,
// though. Because each sensor only identifies its closest beacon, if a sensor
// detects a beacon, you know there are no other beacons that close or closer to
// that sensor. There could still be beacons that just happen to not be the
// closest beacon to any sensor. Consider the sensor at 8,7:

//                1    1    2    2
//      0    5    0    5    0    5
// -2 ..........#.................
// -1 .........###................
//  0 ....S...#####...............
//  1 .......#######........S.....
//  2 ......#########S............
//  3 .....###########SB..........
//  4 ....#############...........
//  5 ...###############..........
//  6 ..#################.........
//  7 .#########S#######S#........
//  8 ..#################.........
//  9 ...###############..........
// 10 ....B############...........
// 11 ..S..###########............
// 12 ......#########.............
// 13 .......#######..............
// 14 ........#####.S.......S.....
// 15 B........###................
// 16 ..........#SB...............
// 17 ................S..........B
// 18 ....S.......................
// 19 ............................
// 20 ............S......S........
// 21 ............................
// 22 .......................B....

// This sensor's closest beacon is at 2,10, and so you know there are no beacons
// that close or closer (in any positions marked #).

// None of the detected beacons seem to be producing the distress signal, so
// you'll need to work out where the distress beacon is by working out where it
// isn't. For now, keep things simple by counting the positions where a beacon
// cannot possibly be along just a single row.

// So, suppose you have an arrangement of beacons and sensors like in the
// example above and, just in the row where y=10, you'd like to count the number
// of positions a beacon cannot possibly exist. The coverage from all sensors
// near that row looks like this:

//                  1    1    2    2
//        0    5    0    5    0    5
//  9 ...#########################...
// 10 ..####B######################..
// 11 .###S#############.###########.

// In this example, in the row where y=10, there are 26 positions where a beacon
// cannot be present.

// Consult the report from the sensors you just deployed. In the row where
// y=2000000, how many positions cannot contain a beacon?

// --- Part Two ---

// Your handheld device indicates that the distress signal is coming from a
// beacon nearby. The distress beacon is not detected by any sensor, but the
// distress beacon must have x and y coordinates each no lower than 0 and no
// larger than 4000000.

// To isolate the distress beacon's signal, you need to determine its tuning
// frequency, which can be found by multiplying its x coordinate by 4000000 and
// then adding its y coordinate.

// In the example above, the search space is smaller: instead, the x and y
// coordinates can each be at most 20. With this reduced search area, there is
// only a single position that could have a beacon: x=14, y=11. The tuning
// frequency for this distress beacon is 56000011.

// Find the only possible position for the distress beacon. What is its tuning
// frequency?

use std::collections::{HashMap, HashSet};

fn parse_coord(coord: &str) -> (i64, i64) {
    let (x, y) = coord.trim().split_once(", ").unwrap();
    (
        x.strip_prefix("x=").unwrap().parse().unwrap(),
        y.strip_prefix("y=").unwrap().parse().unwrap(),
    )
}

fn dist(c1: (i64, i64), c2: (i64, i64)) -> i64 {
    let horiz_dist = (c2.0 - c1.0).abs();
    let vert_dist = (c2.1 - c1.1).abs();
    horiz_dist + vert_dist
}

fn tuning_frequency(c: (i64, i64)) -> i64 {
    c.0 * 4000000 + c.1
}

type SensorWithDist = HashSet<((i64, i64), i64)>;

fn parse(input: &str) -> (SensorWithDist, HashSet<(i64, i64)>) {
    let mut sensor_locs = HashSet::new();
    let mut beacon_locs = HashSet::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let a = line.strip_prefix("Sensor at ").unwrap();
        let (coord_1, coord_2) = a.split_once(": closest beacon is at ").unwrap();

        let c1 = parse_coord(coord_1);
        let c2 = parse_coord(coord_2);

        let manhattan_dist = dist(c2, c1);

        sensor_locs.insert((c1, manhattan_dist));
        beacon_locs.insert(c2);
    }
    (sensor_locs, beacon_locs)
}

pub fn part_1(input: &str, y: i64) -> usize {
    let (sensor_locs, beacon_locs) = parse(input);

    let mut cannot_have_beacon = HashSet::new();

    for ((sensor_x, sensor_y), d) in &sensor_locs {
        for x in (sensor_x - *d - 1)..=(sensor_x + *d + 1) {
            if dist((*sensor_x, *sensor_y), (x, y)) <= *d && !beacon_locs.contains(&(x, y)) {
                cannot_have_beacon.insert((x, y));
            }
        }
    }

    cannot_have_beacon.len()
}

fn merge_overlapping_intervals(arr: &mut Vec<(i64, i64)>) {
    let mut result = vec![arr[0]];

    for current in arr.iter().skip(1) {
        let j = result.len() - 1;

        if current.0 >= result[j].0 && current.0 <= result[j].1 {
            result[j].1 = current.1.max(result[j].1);
        } else {
            result.push(*current);
        }
    }

    *arr = result;
}

pub fn part_2_slow(input: &str, max_range: i64) -> i64 {
    let (sensor_locs, _) = parse(input);

    let mut bad_x_ranges: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();

    for ((sensor_x, sensor_y), d) in &sensor_locs {
        for dd in 0..=*d {
            let r = (sensor_x - (d - dd), sensor_x + (d - dd));
            if sensor_y + dd <= max_range {
                let e1 = bad_x_ranges.entry(sensor_y + dd).or_default();
                if let Err(pos) = e1.binary_search(&r) {
                    e1.insert(pos, r);
                    merge_overlapping_intervals(e1);
                }
            }
            if sensor_y + dd >= 0 {
                let e2 = bad_x_ranges.entry(sensor_y - dd).or_default();
                if let Err(pos) = e2.binary_search(&r) {
                    e2.insert(pos, r);
                    merge_overlapping_intervals(e2);
                }
            }
        }
    }

    for y in 0..=max_range {
        // if there's only one valid point, there's no way there's a range s.t. there's no y value
        let x_ranges = &bad_x_ranges[&y];
        if x_ranges.len() > 1 {
            return tuning_frequency((x_ranges[0].1 + 1, y));
        }
    }

    unreachable!()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Rectangle {
    x_range: (i64, i64),
    y_range: (i64, i64),
}

impl Rectangle {
    fn possibly_viable(self, sensor_loc: (i64, i64), radius: i64) -> bool {
        let corners = [
            (self.x_range.0, self.y_range.0),
            (self.x_range.0, self.y_range.1),
            (self.x_range.1, self.y_range.0),
            (self.x_range.1, self.y_range.1),
        ];

        corners
            .into_iter()
            .map(|c| dist(c, sensor_loc))
            .max()
            .unwrap()
            > radius
    }

    fn c1(self) -> (i64, i64) {
        (self.x_range.0, self.y_range.0)
    }
    fn c2(self) -> (i64, i64) {
        (self.x_range.1, self.y_range.1)
    }

    fn split(self) -> [Rectangle; 4] {
        let mid_x = (self.x_range.1 - self.x_range.0) / 2;
        let mid_y = (self.y_range.1 - self.y_range.0) / 2;

        [
            Rectangle {
                x_range: (self.x_range.0, self.x_range.0 + mid_x),
                y_range: (self.y_range.0, self.y_range.0 + mid_y),
            },
            Rectangle {
                x_range: (self.x_range.0 + mid_x, self.x_range.1),
                y_range: (self.y_range.0, self.y_range.0 + mid_y),
            },
            Rectangle {
                x_range: (self.x_range.0 + mid_x, self.x_range.1),
                y_range: (self.y_range.0 + mid_y, self.y_range.1),
            },
            Rectangle {
                x_range: (self.x_range.0, self.x_range.0 + mid_x),
                y_range: (self.y_range.0 + mid_y, self.y_range.1),
            },
        ]
    }
}

pub fn part_2(input: &str, max_range: i64) -> i64 {
    let (sensor_locs, _) = parse(input);

    let mut stk = vec![Rectangle {
        x_range: (0, max_range),
        y_range: (0, max_range),
    }];

    while let Some(rect) = stk.pop() {
        if rect.c1() == rect.c2() {
            if sensor_locs.iter().all(|(s, d)| dist(*s, rect.c1()) > *d) {
                return tuning_frequency(rect.c1());
            }
        } else {
            let rects = rect.split();

            for r in rects {
                if r == rect {
                    continue;
                }
                if sensor_locs.iter().all(|(s, d)| r.possibly_viable(*s, *d)) {
                    stk.push(r);
                }
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
pub mod tests {
    use crate::day_15::{part_1, part_2, part_2_slow};

    const INPUTS: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

    #[test]
    pub fn test_day_15_example_part1() {
        assert_eq!(part_1(INPUTS, 10), 26);
    }

    #[test]
    pub fn test_day_15_part1() {
        assert_eq!(part_1(include_str!("input/day_15.txt"), 2000000), 5367037);
    }

    #[test]
    pub fn test_day_15_example_part2() {
        assert_eq!(part_2(INPUTS, 20), 56000011);
        assert_eq!(part_2_slow(INPUTS, 20), 56000011);
    }

    #[test]
    pub fn test_day_15_part2() {
        assert_eq!(
            part_2(include_str!("input/day_15.txt"), 4000000),
            11914583249288
        );
    }
}
