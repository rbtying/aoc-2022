// --- Day 19: Not Enough Minerals ---

// Your scans show that the lava did indeed form obsidian!

// The wind has changed direction enough to stop sending lava droplets toward
// you, so you and the elephants exit the cave. As you do, you notice a
// collection of geodes around the pond. Perhaps you could use the obsidian to
// create some geode-cracking robots and break them open?

// To collect the obsidian from the bottom of the pond, you'll need waterproof
// obsidian-collecting robots. Fortunately, there is an abundant amount of clay
// nearby that you can use to make them waterproof.

// In order to harvest the clay, you'll need special-purpose clay-collecting
// robots. To make any type of robot, you'll need ore, which is also plentiful
// but in the opposite direction from the clay.

// Collecting ore requires ore-collecting robots with big drills. Fortunately,
// you have exactly one ore-collecting robot in your pack that you can use to
// kickstart the whole operation.

// Each robot can collect 1 of its resource type per minute. It also takes one
// minute for the robot factory (also conveniently from your pack) to construct
// any type of robot, although it consumes the necessary resources available
// when construction begins.

// The robot factory has many blueprints (your puzzle input) you can choose
// from, but once you've configured it with a blueprint, you can't change it.
// You'll need to work out which blueprint is best.

// For example:

// Blueprint 1:
//   Each ore robot costs 4 ore.
//   Each clay robot costs 2 ore.
//   Each obsidian robot costs 3 ore and 14 clay.
//   Each geode robot costs 2 ore and 7 obsidian.

// Blueprint 2:
//   Each ore robot costs 2 ore.
//   Each clay robot costs 3 ore.
//   Each obsidian robot costs 3 ore and 8 clay.
//   Each geode robot costs 3 ore and 12 obsidian.

// (Blueprints have been line-wrapped here for legibility. The robot factory's
// actual assortment of blueprints are provided one blueprint per line.)

// The elephants are starting to look hungry, so you shouldn't take too long;
// you need to figure out which blueprint would maximize the number of opened
// geodes after 24 minutes by figuring out which robots to build and when to
// build them.

// Using blueprint 1 in the example above, the largest number of geodes you
// could open in 24 minutes is 9. One way to achieve that is:

// == Minute 1 ==
// 1 ore-collecting robot collects 1 ore; you now have 1 ore.

// [...]

// == Minute 24 ==
// 1 ore-collecting robot collects 1 ore; you now have 6 ore.
// 4 clay-collecting robots collect 4 clay; you now have 41 clay.
// 2 obsidian-collecting robots collect 2 obsidian; you now have 8 obsidian.
// 2 geode-cracking robots crack 2 geodes; you now have 9 open geodes.

// However, by using blueprint 2 in the example above, you could do even better:
// the largest number of geodes you could open in 24 minutes is 12.

// Determine the quality level of each blueprint by multiplying that blueprint's
// ID number with the largest number of geodes that can be opened in 24 minutes
// using that blueprint. In this example, the first blueprint has ID 1 and can
// open 9 geodes, so its quality level is 9. The second blueprint has ID 2 and
// can open 12 geodes, so its quality level is 24. Finally, if you add up the
// quality levels of all of the blueprints in the list, you get 33.

// Determine the quality level of each blueprint using the largest number of
// geodes it could produce in 24 minutes. What do you get if you add up the
// quality level of all of the blueprints in your list?

// --- Part Two ---

// While you were choosing the best blueprint, the elephants found some food on
// their own, so you're not in as much of a hurry; you figure you probably have
// 32 minutes before the wind changes direction again and you'll need to get out
// of range of the erupting volcano.

// Unfortunately, one of the elephants ate most of your blueprint list! Now,
// only the first three blueprints in your list are intact.

// In 32 minutes, the largest number of geodes blueprint 1 (from the example
// above) can open is 56. One way to achieve that is:

// However, blueprint 2 from the example above is still better; using it, the
// largest number of geodes you could open in 32 minutes is 62.

// example [...]

// You no longer have enough blueprints to worry about quality levels. Instead,
// for each of the first three blueprints, determine the largest number of
// geodes you could open; then, multiply these three values together.

// Don't worry about quality levels; instead, just determine the largest number
// of geodes you could open using each of the first three blueprints. What do
// you get if you multiply these numbers together?

use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
}

impl Resources {
    fn can_build(self, cost: Resources) -> bool {
        self.ore >= cost.ore
            && self.clay >= cost.clay
            && self.obsidian >= cost.obsidian
            && self.geodes >= cost.geodes
    }

    fn ore_unit() -> Self {
        Resources {
            ore: 1,
            ..Default::default()
        }
    }
    fn clay_unit() -> Self {
        Resources {
            clay: 1,
            ..Default::default()
        }
    }
    fn obsidian_unit() -> Self {
        Resources {
            obsidian: 1,
            ..Default::default()
        }
    }
    fn geodes_unit() -> Self {
        Resources {
            geodes: 1,
            ..Default::default()
        }
    }
}

impl std::ops::Add for Resources {
    type Output = Resources;
    fn add(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geodes: self.geodes + rhs.geodes,
        }
    }
}

impl std::ops::Sub for Resources {
    type Output = Resources;
    fn sub(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geodes: self.geodes - rhs.geodes,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
struct Blueprint {
    id: i32,
    ore_bot_cost: Resources,
    clay_bot_cost: Resources,
    obsidian_bot_cost: Resources,
    geode_bot_cost: Resources,
}

impl Blueprint {
    fn all_costs(self) -> [Resources; 4] {
        [
            self.ore_bot_cost,
            self.clay_bot_cost,
            self.obsidian_bot_cost,
            self.geode_bot_cost,
        ]
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    let mut blueprints = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let line = line.strip_prefix("Blueprint ").unwrap();
        let (num, line) = line.split_once(':').unwrap();
        let mut blueprint = Blueprint {
            id: num.parse().unwrap(),
            ..Default::default()
        };

        for bot_line in line.split('.').map(|l| l.trim()) {
            if bot_line.is_empty() {
                continue;
            }
            let bot_line = bot_line.split_once("Each ").unwrap().1;
            let (typ, cost) = bot_line.split_once(" robot costs ").unwrap();

            let mut c = Resources::default();
            let costs = cost.split(" and ");

            for cost_part in costs {
                let (amt, typ__) = cost_part.split_once(' ').unwrap();

                match typ__ {
                    "ore" => c.ore = amt.parse().unwrap(),
                    "clay" => c.clay = amt.parse().unwrap(),
                    "obsidian" => c.obsidian = amt.parse().unwrap(),
                    _ => unreachable!(),
                }
            }

            match typ {
                "ore" => blueprint.ore_bot_cost = c,
                "clay" => blueprint.clay_bot_cost = c,
                "obsidian" => blueprint.obsidian_bot_cost = c,
                "geode" => blueprint.geode_bot_cost = c,
                _ => unreachable!(),
            }
        }
        blueprints.push(blueprint);
    }

    blueprints
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct State {
    active_bots: Resources,
    resources: Resources,
    no_build_build_mask: Resources,
    minute: u8,
}

impl State {
    fn build_bot(self, new_bot: Resources, cost: Resources, max_costs: Resources) -> Self {
        let mut new_rsc = self.resources - cost + self.active_bots;
        let new_bots = self.active_bots + new_bot;

        if new_bots.ore == max_costs.ore {
            new_rsc.ore = new_rsc.ore.min(max_costs.ore);
        }
        if new_bots.clay == max_costs.clay {
            new_rsc.clay = new_rsc.clay.min(max_costs.clay);
        }
        if new_bots.obsidian == max_costs.obsidian {
            new_rsc.obsidian = new_rsc.obsidian.min(max_costs.obsidian);
        }

        State {
            active_bots: new_bots,
            resources: new_rsc,
            minute: self.minute + 1,
            no_build_build_mask: Resources::default(),
        }
    }

    fn wait(self, max_costs: Resources, build_mask: Resources) -> Self {
        let mut new_rsc = self.resources + self.active_bots;

        if self.active_bots.ore == max_costs.ore {
            new_rsc.ore = new_rsc.ore.min(max_costs.ore);
        }
        if self.active_bots.clay == max_costs.clay {
            new_rsc.clay = new_rsc.clay.min(max_costs.clay);
        }
        if self.active_bots.obsidian == max_costs.obsidian {
            new_rsc.obsidian = new_rsc.obsidian.min(max_costs.obsidian);
        }

        State {
            active_bots: self.active_bots,
            resources: new_rsc,
            minute: self.minute + 1,
            no_build_build_mask: build_mask,
        }
    }
}

fn maximum_geodes(
    blueprint: Blueprint,
    active_bots: Resources,
    resources: Resources,
    time: u8,
) -> i32 {
    let max_costs = Resources {
        ore: blueprint.all_costs().iter().map(|c| c.ore).max().unwrap(),
        clay: blueprint.all_costs().iter().map(|c| c.clay).max().unwrap(),
        obsidian: blueprint
            .all_costs()
            .iter()
            .map(|c| c.obsidian)
            .max()
            .unwrap(),
        geodes: i32::MAX,
    };

    let mut max_geodes = 0;

    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(State {
        active_bots,
        resources,
        minute: 1,
        no_build_build_mask: Resources::default(),
    });

    while let Some(
        state @ State {
            active_bots,
            resources,
            minute,
            no_build_build_mask,
        },
    ) = q.pop_front()
    {
        // We visit in BFS order, which means we never observe minutes out-of-order. So we don't
        // need to include it in the visited set.
        if visited.contains(&(state.active_bots, state.resources)) {
            continue;
        }
        visited.insert((state.active_bots, state.resources));

        max_geodes = max_geodes.max((resources + active_bots).geodes);

        if minute == time {
            continue;
        }

        // If we managed to make a geode machine every cycle, this is the maximum number of geodes
        // that can be produced:
        // - new geodes machines: (1+2+3+...+(remaining time))
        // - existing geodes: remaining time * active geode bots
        let remaining_time = time as i32 - minute as i32;
        let max_potential_geodes_remaining =
            (remaining_time - 1) * remaining_time / 2 + active_bots.geodes * remaining_time;

        // If this branch can't produce enough geodes to beat our current max, give up.
        if max_potential_geodes_remaining + resources.geodes < max_geodes {
            continue;
        }

        // If we waited a cycle when we _could_ have built something, don't try to build it this
        // cycle -- we also explored that branch one minute ago, and that branch is strictly better
        // in all cases.
        let build_mask = Resources {
            geodes: resources.can_build(blueprint.geode_bot_cost).into(),
            obsidian: resources.can_build(blueprint.obsidian_bot_cost).into(),
            clay: resources.can_build(blueprint.clay_bot_cost).into(),
            ore: resources.can_build(blueprint.ore_bot_cost).into(),
        } - no_build_build_mask;

        // Always prefer to build geode bots, if possible.
        if build_mask.geodes > 0 {
            q.push_back(state.build_bot(
                Resources::geodes_unit(),
                blueprint.geode_bot_cost,
                max_costs,
            ));
            continue;
        }

        // Or obsidian bots, if we can, and we haven't maxed out the value
        if active_bots.obsidian < max_costs.obsidian && build_mask.obsidian > 0 {
            q.push_back(state.build_bot(
                Resources::obsidian_unit(),
                blueprint.obsidian_bot_cost,
                max_costs,
            ));
        }
        // Or clay bots, if we can, and we haven't maxed out the value
        if active_bots.clay < max_costs.clay && build_mask.clay > 0 {
            q.push_back(state.build_bot(
                Resources::clay_unit(),
                blueprint.clay_bot_cost,
                max_costs,
            ));
        }
        // Or ore bots, if we can, and we haven't maxed out the value
        if active_bots.ore < max_costs.ore && build_mask.ore > 0 {
            q.push_back(state.build_bot(Resources::ore_unit(), blueprint.ore_bot_cost, max_costs));
        }

        // If we can already build every type of bot, no need to wait for resources ever.
        if build_mask.obsidian > 0 && build_mask.clay > 0 && build_mask.ore > 0 {
            continue;
        }

        q.push_back(state.wait(max_costs, build_mask));
    }

    max_geodes
}

pub fn part_1(input: &str) -> i32 {
    let blueprints = parse(input);

    let mut quality_sum = 0;
    for blueprint in blueprints {
        let resources = Resources::default();
        let active_bots = Resources {
            ore: 1,
            ..Default::default()
        };
        let max_geodes = maximum_geodes(blueprint, active_bots, resources, 24);

        quality_sum += blueprint.id * max_geodes;
    }

    quality_sum
}

pub fn part_2(input: &str) -> i32 {
    let blueprints = parse(input);

    let mut p = 1;
    for blueprint in blueprints.iter().take(3) {
        let resources = Resources::default();
        let active_bots = Resources {
            ore: 1,
            ..Default::default()
        };
        let max_geodes = maximum_geodes(*blueprint, active_bots, resources, 32);
        p *= max_geodes;
    }
    p
}

#[cfg(test)]
pub mod tests {
    use crate::day_19::{part_1, part_2};

    const INPUTS: &str = r#"Blueprint 1: Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore.  Each clay robot costs 3 ore.  Each obsidian robot costs 3 ore and 8 clay.  Each geode robot costs 3 ore and 12 obsidian."#;

    #[test]
    pub fn test_day_19_example_part1() {
        assert_eq!(part_1(INPUTS), 33);
    }

    #[test]
    pub fn test_day_19_part1() {
        assert_eq!(part_1(include_str!("input/day_19.txt")), 1349);
    }

    #[test]
    pub fn test_day_19_example_part2() {
        assert_eq!(part_2(INPUTS), 62 * 56);
    }

    #[test]
    pub fn test_day_19_part2() {
        assert_eq!(part_2(include_str!("input/day_19.txt")), 21840);
    }
}
