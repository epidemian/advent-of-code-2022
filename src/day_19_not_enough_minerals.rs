use std::collections::HashMap;

pub fn run(input: &str) -> String {
    let blueprints: Vec<_> = input.lines().map(parse_blueprint).collect();

    let quality_levels = blueprints.iter().map(|blueprint| blueprint.quality_level());
    let total_quality_level: u32 = quality_levels.sum();

    let first_blueprints_max_geodes_product: u32 = blueprints
        .iter()
        .take(3)
        .map(|blueprint| blueprint.find_max_geodes_in(32))
        .product();

    format!("{total_quality_level} {first_blueprints_max_geodes_product}")
}

struct Blueprint {
    id: u32,
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}

impl Blueprint {
    fn quality_level(&self) -> u32 {
        self.id * self.find_max_geodes_in(24)
    }

    fn find_max_geodes_in(&self, minutes: u32) -> u32 {
        let mut max_geodes = 0;
        let mut state_cache = HashMap::new();
        find_max_geodes(
            self,
            &State::new(),
            minutes,
            &mut state_cache,
            &mut max_geodes,
        );
        println!(
            "max geodes for blueprint {} in {minutes} minutes: {max_geodes}, cache size={}",
            self.id,
            state_cache.len()
        );
        max_geodes
    }
}

#[derive(Default, Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct State {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl State {
    fn new() -> State {
        State {
            ore_robots: 1,
            ..State::default()
        }
    }

    fn tick(&self) -> State {
        State {
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geodes: self.geodes + self.geode_robots,
            ..*self
        }
    }
}

fn parse_blueprint(line: &str) -> Blueprint {
    let numbers: Vec<_> = line
        .split([' ', ':'])
        .filter_map(|word| word.parse().ok())
        .collect();

    Blueprint {
        id: numbers[0],
        ore_robot_ore_cost: numbers[1],
        clay_robot_ore_cost: numbers[2],
        obsidian_robot_ore_cost: numbers[3],
        obsidian_robot_clay_cost: numbers[4],
        geode_robot_ore_cost: numbers[5],
        geode_robot_obsidian_cost: numbers[6],
    }
}

fn find_max_geodes(
    blueprint: &Blueprint,
    state: &State,
    remaining_minutes: u32,
    state_cache: &mut HashMap<State, (u32, u32)>,
    max_geodes: &mut u32,
) {
    if let Some((cached_minutes, _cached_max_geodes)) = state_cache.get(state) {
        // We have seen this same state but with more remaining minutes, so it doesn't make sense to
        // evaluate this state+minutes combination.
        if remaining_minutes <= *cached_minutes {
            return;
        }
    }

    if remaining_minutes == 0 {
        if state.geodes > *max_geodes {
            *max_geodes = state.geodes;
        }
        return;
    }

    let can_make_ore_robot = state.ore >= blueprint.ore_robot_ore_cost;
    if can_make_ore_robot {
        let mut new_state = state.tick();
        new_state.ore -= blueprint.ore_robot_ore_cost;
        new_state.ore_robots += 1;
        find_max_geodes(
            blueprint,
            &new_state,
            remaining_minutes - 1,
            state_cache,
            max_geodes,
        );
    }

    let can_make_clay_robot = state.ore >= blueprint.clay_robot_ore_cost;
    if can_make_clay_robot {
        let mut new_state = state.tick();
        new_state.clay_robots += 1;
        new_state.ore -= blueprint.clay_robot_ore_cost;
        find_max_geodes(
            blueprint,
            &new_state,
            remaining_minutes - 1,
            state_cache,
            max_geodes,
        );
    }

    let can_make_obsidian_robot = state.ore >= blueprint.obsidian_robot_ore_cost
        && state.clay >= blueprint.obsidian_robot_clay_cost;
    if can_make_obsidian_robot {
        let mut new_state = state.tick();
        new_state.obsidian_robots += 1;
        new_state.ore -= blueprint.obsidian_robot_ore_cost;
        new_state.clay -= blueprint.obsidian_robot_clay_cost;
        find_max_geodes(
            blueprint,
            &new_state,
            remaining_minutes - 1,
            state_cache,
            max_geodes,
        );
    }

    let can_make_geode_robot = state.ore >= blueprint.geode_robot_ore_cost
        && state.obsidian >= blueprint.geode_robot_obsidian_cost;
    if can_make_geode_robot {
        let mut new_state = state.tick();
        new_state.geode_robots += 1;
        new_state.ore -= blueprint.geode_robot_ore_cost;
        new_state.obsidian -= blueprint.geode_robot_obsidian_cost;
        find_max_geodes(
            blueprint,
            &new_state,
            remaining_minutes - 1,
            state_cache,
            max_geodes,
        );
    }

    // let can_make_any_robot = can_make_ore_robot
    //     && can_make_clay_robot
    //     && can_make_obsidian_robot
    //     && can_make_geode_robot;
    // if can_make_any_robot {
    //     return;
    // }

    find_max_geodes(
        blueprint,
        &state.tick(),
        remaining_minutes - 1,
        state_cache,
        max_geodes,
    );

    state_cache.insert(*state, (remaining_minutes, *max_geodes));
}
