pub fn run(input: &str) -> String {
    let blueprints: Vec<_> = input.lines().map(parse_blueprint).collect();

    let total_quality_level: u32 = blueprints
        .iter()
        .map(|b| b.id * find_max_geodes(b, 24))
        .sum();

    let max_geodes_product: u32 = blueprints
        .iter()
        .take(3)
        .map(|b| find_max_geodes(b, 32))
        .product();

    format!("{total_quality_level} {max_geodes_product}")
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
        let mut next = State { ..*self };
        next.ore += self.ore_robots;
        next.clay += self.clay_robots;
        next.obsidian += self.obsidian_robots;
        next.geodes += self.geode_robots;
        next.remaining_minutes -= 1;
        next
    }
}

fn find_max_geodes(blueprint: &Blueprint, minutes: u32) -> u32 {
    let mut max_geodes = 0;
    let no_info = [false; 3];
    let mut stack = vec![(State::new(minutes), no_info)];
    while let Some((state, prev_minute_info)) = stack.pop() {
        if state.remaining_minutes == 0 {
            max_geodes = max_geodes.max(state.geodes);
            continue;
        }

        // If there are N remaining minutes, we could, at most, produce N geode robots, which would
        // crack N-1 + N-2 + ... + 1 geodes. This is the same as the sum the N-1 first integers,
        // which is (N-1)*N/2
        let geode_upper_bound = state.geodes
            + state.geode_robots * state.remaining_minutes
            + (state.remaining_minutes - 1) * state.remaining_minutes / 2;
        if geode_upper_bound <= max_geodes {
            continue;
        }

        let can_make_geode_robot = state.ore >= blueprint.geode_robot_ore_cost
            && state.obsidian >= blueprint.geode_robot_obsidian_cost;
        if can_make_geode_robot {
            let mut new_state = state.tick();
            new_state.geode_robots += 1;
            new_state.ore -= blueprint.geode_robot_ore_cost;
            new_state.obsidian -= blueprint.geode_robot_obsidian_cost;
            stack.push((new_state, no_info));

            // If we can make a geode robot this turn, don't evaluate other possibilities. Making
            // geode robots is always best :)
            continue;
        }

        let [could_make_ore_robot, could_make_clay_robot, could_make_obsidian_robot] =
            prev_minute_info;

        let can_make_obsidian_robot = state.ore >= blueprint.obsidian_robot_ore_cost
            && state.clay >= blueprint.obsidian_robot_clay_cost;
        if can_make_obsidian_robot && !could_make_obsidian_robot {
            let mut new_state = state.tick();
            new_state.obsidian_robots += 1;
            new_state.ore -= blueprint.obsidian_robot_ore_cost;
            new_state.clay -= blueprint.obsidian_robot_clay_cost;
            stack.push((new_state, no_info));
        }

        let can_make_clay_robot = state.ore >= blueprint.clay_robot_ore_cost;
        let enough_clay_robots = state.clay_robots >= blueprint.obsidian_robot_clay_cost;
        if can_make_clay_robot && !could_make_clay_robot && !enough_clay_robots {
            let mut new_state = state.tick();
            new_state.clay_robots += 1;
            new_state.ore -= blueprint.clay_robot_ore_cost;
            stack.push((new_state, no_info));
        }

        let can_make_ore_robot = state.ore >= blueprint.ore_robot_ore_cost;
        let enough_ore_robots = state.ore_robots >= blueprint.ore_robot_ore_cost
            && state.ore_robots >= blueprint.clay_robot_ore_cost
            && state.ore_robots >= blueprint.obsidian_robot_ore_cost
            && state.ore_robots >= blueprint.geode_robot_ore_cost;
        if can_make_ore_robot && !could_make_ore_robot && !enough_ore_robots {
            let mut new_state = state.tick();
            new_state.ore -= blueprint.ore_robot_ore_cost;
            new_state.ore_robots += 1;
            stack.push((new_state, no_info));
        }

        let minute_info = [
            can_make_ore_robot,
            can_make_clay_robot,
            can_make_obsidian_robot,
        ];
        stack.push((state.tick(), minute_info))
    }
    max_geodes
}
