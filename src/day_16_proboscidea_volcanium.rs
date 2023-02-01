use crate::dijkstra::shortest_path_distances;
use std::collections::HashMap;

// Note: this solution was ~stolen from~ heavily inspired by
// https://old.reddit.com/r/adventofcode/comments/zn6k1l/2022_day_16_solutions/j2xhog7/
pub fn run(input: &str) -> String {
    let graph = parse_graph(input);
    let distances: HashMap<_, _> = graph
        .keys()
        .map(|id| {
            let mut dists =
                shortest_path_distances(id, |id| graph[id].connected_valves.iter().cloned());
            dists.retain(|other_id, _dist| other_id != id && graph[other_id].flow_rate > 0);
            (*id, dists)
        })
        .collect();

    let bitmask_indices: HashMap<&str, u16> = graph
        .iter()
        .filter(|(_id, valve)| valve.flow_rate > 0)
        .enumerate()
        .map(|(i, (id, _valve))| (*id, 1 << i))
        .collect();

    let mut part_1_max_pressures = HashMap::new();
    visit_all_paths(
        &graph,
        &distances,
        &bitmask_indices,
        "AA",
        30,
        0,
        0,
        &mut part_1_max_pressures,
    );
    let part_1_ans = part_1_max_pressures.values().max().unwrap();

    let mut part_2_max_pressures = HashMap::new();
    visit_all_paths(
        &graph,
        &distances,
        &bitmask_indices,
        "AA",
        26,
        0,
        0,
        &mut part_2_max_pressures,
    );
    let mut part_2_ans = 0;
    for (bitmask_1, pressure_1) in part_2_max_pressures.iter() {
        for (bitmask_2, pressure_2) in part_2_max_pressures.iter() {
            if (bitmask_1 & bitmask_2) == 0 {
                part_2_ans = part_2_ans.max(pressure_1 + pressure_2)
            }
        }
    }

    format!("{part_1_ans} {part_2_ans}")
}

type Bitmask = u16;

#[allow(clippy::too_many_arguments)]
fn visit_all_paths(
    graph: &HashMap<&str, Valve>,
    distances: &HashMap<&str, HashMap<&str, usize>>,
    bitmask_indices: &HashMap<&str, Bitmask>,
    current_valve: &str,
    remaining_minutes: u64,
    open_valves_bitmask: Bitmask,
    released_pressure: u64,
    max_released_pressure: &mut HashMap<Bitmask, u64>,
) {
    let max_val = max_released_pressure
        .entry(open_valves_bitmask)
        .or_insert(0);
    if released_pressure > *max_val {
        *max_val = released_pressure;
    }

    for (other_valve, &dist) in distances[current_valve].iter() {
        if dist as u64 + 1 > remaining_minutes
            || (bitmask_indices[other_valve] & open_valves_bitmask) != 0
        {
            continue;
        }
        let remaining_minutes = remaining_minutes - dist as u64 - 1;
        visit_all_paths(
            graph,
            distances,
            bitmask_indices,
            other_valve,
            remaining_minutes,
            open_valves_bitmask | bitmask_indices[other_valve],
            released_pressure + remaining_minutes * graph[other_valve].flow_rate,
            max_released_pressure,
        )
    }
}

struct Valve<'a> {
    flow_rate: u64,
    connected_valves: Vec<&'a str>,
}

fn parse_graph(input: &str) -> HashMap<&str, Valve> {
    input
        .lines()
        .map(|line| {
            let words: Vec<_> = line
                .split([' ', ';', '=', ','])
                .filter(|s| !s.is_empty())
                .collect();
            let valve_id = words[1];
            let flow_rate = words[5].parse().unwrap();
            let connected_valves = words[10..].to_vec();
            let valve = Valve {
                flow_rate,
                connected_valves,
            };
            (valve_id, valve)
        })
        .collect()
}
