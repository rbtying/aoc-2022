// See day_16_prompt.txt

use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
pub struct Valve {
    flow_rate: i64,
    tunnels: Vec<String>,
}

fn parse(input: &str) -> HashMap<String, Valve> {
    let mut m = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let line = line.strip_prefix("Valve ").unwrap();
        let (name, line) = line.split_once(" has flow rate=").unwrap();
        let (rate, line) = match line.split_once("; tunnels lead to valves ") {
            Some(v) => v,
            None => line.split_once("; tunnel leads to valve ").unwrap(),
        };
        let tunnels = line.split(", ").map(|t| t.to_string()).collect();

        m.insert(
            name.to_string(),
            Valve {
                flow_rate: rate.parse().unwrap(),
                tunnels,
            },
        );
    }
    m
}

/// Straightforward, not-particularly-memoized BFS approach.
fn best_ending_pressures(
    aa_idx: u8,
    has_flows: &[u8],
    get_rate: impl Fn(u8) -> i64,
    paths: impl Fn((u8, u8)) -> usize,
    max_time: usize,
) -> HashMap<usize, i64> {
    let mut maxes = HashMap::new();

    struct State {
        minute: usize,
        opened: usize,
        current_node: u8,
        current_rate: i64,
        pressure_released: i64,
    }

    let mut q = VecDeque::new();

    q.push_back(State {
        minute: 0,
        opened: 0,
        current_node: aa_idx,
        current_rate: get_rate(aa_idx),
        pressure_released: 0,
    });

    while let Some(State {
        minute,
        opened,
        current_node,
        current_rate,
        pressure_released,
    }) = q.pop_front()
    {
        // Assume that we're done -- we're not necessarily terminated,
        // but there's no point re-exploring going on another path after
        // waiting a cycle, since if there were another path to go to,
        // we would want to go earlier.
        let final_pressure = pressure_released + current_rate * (max_time - minute) as i64;

        let current_max = maxes.get(&opened).copied().unwrap_or(0);
        maxes.insert(opened, current_max.max(final_pressure));

        for (idx, next_node) in has_flows.iter().enumerate() {
            if *next_node == current_node {
                continue;
            }

            let travel_time = paths((current_node, *next_node)) + 1;

            // If we can open this valve, in time, go open it
            if (1 << idx) & opened == 0 && minute + travel_time <= max_time {
                q.push_back(State {
                    minute: minute + travel_time,
                    opened: opened | 1 << idx,
                    current_node: *next_node,
                    current_rate: current_rate + get_rate(*next_node),
                    pressure_released: pressure_released + travel_time as i64 * current_rate,
                });
            }
        }
    }

    maxes
}

fn floyd_warshall(
    nodes: impl Iterator<Item = u8> + Clone,
    adjacent_pairs: impl Iterator<Item = (u8, u8)>,
) -> HashMap<(u8, u8), i64> {
    let mut paths = HashMap::new();

    // Floyd-Warshall init
    for p in adjacent_pairs {
        paths.insert(p, 1);
    }
    for i in nodes.clone() {
        paths.insert((i, i), 0);
    }

    // Compute Floyd-Warshall
    for i in nodes.clone() {
        for j in nodes.clone() {
            for k in nodes.clone() {
                let djk = paths.get(&(j, k)).copied().unwrap_or(i64::MAX);
                let dji = paths.get(&(j, i)).copied().unwrap_or(i64::MAX);
                let dik = paths.get(&(i, k)).copied().unwrap_or(i64::MAX);
                let djiik = dji.saturating_add(dik);

                if djk > djiik {
                    paths.insert((j, k), djiik);
                }
            }
        }
    }

    paths
}

pub fn part_1(input: &str) -> i64 {
    let valves = parse(input);
    let mut valve_list = valves.keys().cloned().collect::<Vec<_>>();
    valve_list.sort();

    let mut name_to_idx = HashMap::new();
    let mut has_flows = vec![];

    for (idx, name) in valve_list.iter().enumerate() {
        name_to_idx.insert(name.to_string(), idx as u8);
        if valves[name].flow_rate > 0 || name == "AA" {
            has_flows.push(idx as u8);
        }
    }

    let paths = floyd_warshall(
        name_to_idx.values().copied(),
        name_to_idx
            .iter()
            .flat_map(|(n, i)| valves[n].tunnels.iter().map(|jn| (*i, name_to_idx[jn]))),
    );

    *best_ending_pressures(
        name_to_idx["AA"],
        &has_flows,
        |idx| valves[&valve_list[idx as usize]].flow_rate,
        |p| paths[&p] as usize,
        30,
    )
    .values()
    .max()
    .unwrap()
}

pub fn part_2(input: &str) -> i64 {
    let valves = parse(input);
    let mut valve_list = valves.keys().cloned().collect::<Vec<_>>();
    valve_list.sort();

    let mut name_to_idx = HashMap::new();
    let mut has_flows = vec![];

    for (idx, name) in valve_list.iter().enumerate() {
        name_to_idx.insert(name.to_string(), idx as u8);
        if valves[name].flow_rate > 0 {
            has_flows.push(idx as u8);
        }
    }

    let paths = floyd_warshall(
        name_to_idx.values().copied(),
        name_to_idx
            .iter()
            .flat_map(|(n, i)| valves[n].tunnels.iter().map(|jn| (*i, name_to_idx[jn]))),
    );

    let ending_pressures = best_ending_pressures(
        name_to_idx["AA"],
        &has_flows,
        |idx| valves[&valve_list[idx as usize]].flow_rate,
        |p| paths[&p] as usize,
        26,
    );

    let mut max = 0;

    for (me, f_me) in &ending_pressures {
        for (elephant, f_elephant) in &ending_pressures {
            if me & elephant == 0 {
                max = max.max(*f_me + *f_elephant);
            }
        }
    }

    max
}

#[cfg(test)]
pub mod tests {
    use crate::day_16::{part_1, part_2};

    const INPUTS: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

    #[test]
    pub fn test_day_16_example_part1() {
        assert_eq!(part_1(INPUTS), 1651);
    }

    #[test]
    pub fn test_day_16_part1() {
        assert_eq!(part_1(include_str!("input/day_16.txt")), 2080);
    }

    #[test]
    pub fn test_day_16_example_part2() {
        assert_eq!(part_2(INPUTS), 1707);
    }

    #[test]
    pub fn test_day_16_part2() {
        assert_eq!(part_2(include_str!("input/day_16.txt")), 2752);
    }
}
