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
        find_max_geodes(
            self,
            &State::new(minutes),
            false,
            false,
            false,
            &mut max_geodes,
        );
        max_geodes
    }
}

#[derive(Default)]
struct State {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    remaining_minutes: u32,
}

impl State {
    fn new(remaining_minutes: u32) -> State {
        State {
            ore_robots: 1,
            remaining_minutes,
            ..State::default()
        }
    }

    fn tick(&self) -> State {
        State {
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geodes: self.geodes + self.geode_robots,
            remaining_minutes: self.remaining_minutes - 1,
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
    could_make_ore_robot: bool,
    could_make_clay_robot: bool,
    could_make_obsidian_robot: bool,
    max_geodes: &mut u32,
) {
    if state.remaining_minutes == 0 {
        if state.geodes > *max_geodes {
            *max_geodes = state.geodes;
        }
        return;
    }

    // If there are N remaining minutes, we could, at most, produce N geode robots, which would
    // crack N-1 + N-2 + ... + 1 geodes. This is the same as the sum the N-1 first integers, which
    // is (N-1)*N/2
    let geode_upper_bound = state.geodes
        + state.geode_robots * state.remaining_minutes
        + (state.remaining_minutes - 1) * state.remaining_minutes / 2;
    if geode_upper_bound <= *max_geodes {
        return;
    }

    let can_make_geode_robot = state.ore >= blueprint.geode_robot_ore_cost
        && state.obsidian >= blueprint.geode_robot_obsidian_cost;
    if can_make_geode_robot {
        let mut new_state = state.tick();
        new_state.geode_robots += 1;
        new_state.ore -= blueprint.geode_robot_ore_cost;
        new_state.obsidian -= blueprint.geode_robot_obsidian_cost;
        find_max_geodes(blueprint, &new_state, false, false, false, max_geodes);

        // If we can make a geode robot this turn, don't evaluate other possibilities. Making geode
        // robots is always best :)
        return;
    }

    let can_make_obsidian_robot = state.ore >= blueprint.obsidian_robot_ore_cost
        && state.clay >= blueprint.obsidian_robot_clay_cost;
    if can_make_obsidian_robot && !could_make_obsidian_robot {
        let mut new_state = state.tick();
        new_state.obsidian_robots += 1;
        new_state.ore -= blueprint.obsidian_robot_ore_cost;
        new_state.clay -= blueprint.obsidian_robot_clay_cost;
        find_max_geodes(blueprint, &new_state, false, false, false, max_geodes);
    }

    let can_make_clay_robot = state.ore >= blueprint.clay_robot_ore_cost;
    if can_make_clay_robot && !could_make_clay_robot {
        let mut new_state = state.tick();
        new_state.clay_robots += 1;
        new_state.ore -= blueprint.clay_robot_ore_cost;
        find_max_geodes(blueprint, &new_state, false, false, false, max_geodes);
    }

    let can_make_ore_robot = state.ore >= blueprint.ore_robot_ore_cost;
    if can_make_ore_robot && !could_make_ore_robot {
        let mut new_state = state.tick();
        new_state.ore -= blueprint.ore_robot_ore_cost;
        new_state.ore_robots += 1;
        find_max_geodes(blueprint, &new_state, false, false, false, max_geodes);
    }

    find_max_geodes(
        blueprint,
        &state.tick(),
        can_make_ore_robot,
        can_make_clay_robot,
        can_make_obsidian_robot,
        max_geodes,
    );
}
