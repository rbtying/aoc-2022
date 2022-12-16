// See day_16_prompt.txt

use std::collections::{HashMap, VecDeque};
use std::sync::mpsc;
use std::thread::spawn;

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
fn max_pressure_bfs(
    aa_idx: u8,
    has_flows: &[u8],
    get_rate: impl Fn(u8) -> i64,
    paths: impl Fn((u8, u8)) -> usize,
    max_time: usize,
) -> i64 {
    let mut max = 0;

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
            } else {
                // Assume that we're done -- we're not necessarily terminated,
                // but there's no point re-exploring going on another path after
                // waiting a cycle, since if there were another path to go to,
                // we would want to go earlier.
                let final_pressure = pressure_released + current_rate * (max_time - minute) as i64;
                max = max.max(final_pressure);
            }
        }
    }

    max
}

/// I found it hard to optimize the original BFS solution originally, so I tried
/// straight dynamic programming. Unfortunately, it doesn't quite run fast
/// enough for part 2.
pub fn max_pressure_dp(
    aa_idx: u8,
    has_flows: &[u8],
    get_rate: impl Fn(u8) -> i64,
    paths: impl Fn((u8, u8)) -> usize,
    max_time: usize,
) -> i64 {
    let mut tbl: Vec<Vec<Vec<i64>>> =
        vec![vec![vec![i64::MIN; 1 << has_flows.len()]; has_flows.len()]; (max_time + 1) as usize];

    let mut max = 0;

    for (i, v) in has_flows.iter().enumerate() {
        let dist = paths((aa_idx, *v));
        tbl[dist + 1][i][1 << i] = 0;
    }

    // Try for max_time minutes
    for minute in 1..=max_time as usize {
        // With the following set of valves opened
        for opened in 0..(1 << has_flows.len()) {
            // Try each location that we can go to
            for current_node in 0..has_flows.len() {
                // compute the current flow if we don't go anywhere
                #[allow(clippy::redundant_closure)]
                let released_pressure: i64 =
                    apply_mask(has_flows, opened).map(|n| get_rate(n)).sum();

                let do_nothing: i64 = tbl[minute - 1][current_node][opened] + released_pressure;

                if do_nothing > tbl[minute][current_node][opened] {
                    tbl[minute][current_node][opened] = do_nothing;
                }
                max = max.max(tbl[minute][current_node][opened]);

                // If we haven't opened the current node, move on
                if (1 << current_node) & opened == 0 {
                    continue;
                }

                // Otherwise, consider all the possible downstream paths
                for next_node in 0..has_flows.len() {
                    // Don't consider already-opened nodes
                    if ((1 << next_node) & opened) != 0 {
                        continue;
                    }

                    // Skip ahead the number of moves necessary to get to the next node
                    let dist = paths((has_flows[current_node], has_flows[next_node]));

                    // If it takes too long to get there and then open it, ignore it
                    if minute + dist + 1 > max_time {
                        continue;
                    }

                    // Consider the benefit of going there and opening it
                    let v =
                        tbl[minute][current_node][opened] + released_pressure * (dist as i64 + 1);

                    if v > tbl[minute + dist + 1][next_node][opened | (1 << next_node)] {
                        tbl[minute + dist + 1][next_node][opened | (1 << next_node)] = v;
                    }
                }
            }
        }
    }

    max
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

    max_pressure_bfs(
        name_to_idx["AA"],
        &has_flows,
        |idx| valves[&valve_list[idx as usize]].flow_rate,
        |p| paths[&p] as usize,
        30,
    )
}

pub fn part_1_dp(input: &str) -> i64 {
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

    max_pressure_dp(
        name_to_idx["AA"],
        &has_flows,
        |idx| valves[&valve_list[idx as usize]].flow_rate,
        |p| paths[&p] as usize,
        30,
    )
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

fn apply_mask(has_flows: &[u8], mask: usize) -> impl Iterator<Item = u8> + '_ {
    assert!(has_flows.len() < usize::BITS as usize);

    has_flows
        .iter()
        .enumerate()
        .filter(move |(idx, _)| mask & (1 << idx) != 0)
        .map(move |(_, v)| *v)
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

    println!("searching for best combo... {:?}", has_flows);

    // This is trivially parallelizable, so let's spin up a few threads

    let num_threads = match std::thread::available_parallelism() {
        Ok(v) => v.get(),
        Err(_) => 1,
    };

    let mut producers: Vec<mpsc::Sender<Option<usize>>> = vec![];
    let mut handles = vec![];
    for _ in 0..num_threads {
        let (tx, rx) = mpsc::channel();
        producers.push(tx);
        let paths_ = paths.clone();
        let valve_list_ = valve_list.clone();
        let has_flows_ = has_flows.clone();
        let valves_ = valves.clone();
        let aa_idx = name_to_idx["AA"];
        handles.push(spawn(move || {
            let mut max = 0;
            while let Some(i) = rx.recv().unwrap() {
                let me = i;
                let elephant = !i & ((1 << has_flows_.len()) - 1);

                let me_nodes = apply_mask(&has_flows_, me).collect::<Vec<_>>();
                let me_max = max_pressure_bfs(
                    aa_idx,
                    &me_nodes,
                    |idx| valves_[&valve_list_[idx as usize]].flow_rate,
                    |p| paths_[&p] as usize,
                    26,
                );

                let elephant_nodes = apply_mask(&has_flows_, elephant).collect::<Vec<_>>();
                let elephant_max = max_pressure_bfs(
                    aa_idx,
                    &elephant_nodes,
                    |idx| valves_[&valve_list_[idx as usize]].flow_rate,
                    |p| paths_[&p] as usize,
                    26,
                );

                max = max.max(me_max + elephant_max);
            }

            max
        }));
    }

    // No point going beyond the 50% mark, since `me` and `elephant` are
    // symmetric.
    for i in 0..(1 << (has_flows.len() - 1)) {
        producers[i % num_threads].send(Some(i)).unwrap();
    }
    producers.iter_mut().for_each(|tx| tx.send(None).unwrap());

    handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .max()
        .unwrap()
}

#[cfg(test)]
pub mod tests {
    use crate::day_16::{part_1, part_1_dp, part_2};

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
        assert_eq!(part_1_dp(INPUTS), 1651);
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
