pub fn run(input: &str) -> String {
    let blueprints: Vec<_> = input.lines().map(Blueprint::parse).collect();

    let total_quality_level: u32 = blueprints.iter().map(|b| b.quality_level()).sum();

    let max_geodes_product: u32 = blueprints
        .iter()
        .take(3)
        .map(|b| b.find_max_geodes_in(32))
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

impl Blueprint {
    fn parse(line: &str) -> Blueprint {
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

    fn quality_level(&self) -> u32 {
        self.id * self.find_max_geodes_in(24)
    }

    fn find_max_geodes_in(&self, minutes: u32) -> u32 {
        let mut max_geodes = 0;
        State::new(minutes).find_max_geodes(self, false, false, false, &mut max_geodes);
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

    fn geode_upper_bound(&self) -> u32 {
        // If there are N remaining minutes, we could, at most, produce N geode robots, which would
        // crack N-1 + N-2 + ... + 1 geodes. This is the same as the sum the N-1 first integers,
        // which is (N-1)*N/2
        self.geodes
            + self.geode_robots * self.remaining_minutes
            + (self.remaining_minutes - 1) * self.remaining_minutes / 2
    }

    fn find_max_geodes(
        &self,
        blueprint: &Blueprint,
        could_make_ore_robot: bool,
        could_make_clay_robot: bool,
        could_make_obsidian_robot: bool,
        max_geodes: &mut u32,
    ) {
        if self.remaining_minutes == 0 {
            if self.geodes > *max_geodes {
                *max_geodes = self.geodes;
            }
            return;
        }

        if self.geode_upper_bound() <= *max_geodes {
            return;
        }

        let can_make_geode_robot = self.ore >= blueprint.geode_robot_ore_cost
            && self.obsidian >= blueprint.geode_robot_obsidian_cost;
        if can_make_geode_robot {
            let mut new_state = self.tick();
            new_state.geode_robots += 1;
            new_state.ore -= blueprint.geode_robot_ore_cost;
            new_state.obsidian -= blueprint.geode_robot_obsidian_cost;
            new_state.find_max_geodes(blueprint, false, false, false, max_geodes);

            // If we can make a geode robot this turn, don't evaluate other possibilities. Making
            // geode robots is always best :)
            return;
        }

        let can_make_obsidian_robot = self.ore >= blueprint.obsidian_robot_ore_cost
            && self.clay >= blueprint.obsidian_robot_clay_cost;
        if can_make_obsidian_robot && !could_make_obsidian_robot {
            let mut new_state = self.tick();
            new_state.obsidian_robots += 1;
            new_state.ore -= blueprint.obsidian_robot_ore_cost;
            new_state.clay -= blueprint.obsidian_robot_clay_cost;
            new_state.find_max_geodes(blueprint, false, false, false, max_geodes);
        }

        let can_make_clay_robot = self.ore >= blueprint.clay_robot_ore_cost;
        if can_make_clay_robot && !could_make_clay_robot {
            let mut new_state = self.tick();
            new_state.clay_robots += 1;
            new_state.ore -= blueprint.clay_robot_ore_cost;
            new_state.find_max_geodes(blueprint, false, false, false, max_geodes);
        }

        let can_make_ore_robot = self.ore >= blueprint.ore_robot_ore_cost;
        if can_make_ore_robot && !could_make_ore_robot {
            let mut new_state = self.tick();
            new_state.ore -= blueprint.ore_robot_ore_cost;
            new_state.ore_robots += 1;
            new_state.find_max_geodes(blueprint, false, false, false, max_geodes);
        }

        self.tick().find_max_geodes(
            blueprint,
            can_make_ore_robot,
            can_make_clay_robot,
            can_make_obsidian_robot,
            max_geodes,
        );
    }
}
