use crate::dijkstra::shortest_path_distances;
use std::collections::HashMap;

pub fn run(input: &str) -> String {
    let graph = parse_graph(input);
    let distances: HashMap<_, _> = graph
        .iter()
        // .filter(|(_id, valve)| valve.flow_rate > 0)
        .map(|(id, _valve)| {
            let dists =
                shortest_path_distances(id, |id| graph[id].connected_valves.iter().cloned());
            let dists: HashMap<_, _> = dists
                .into_iter()
                .filter(|(other_id, _dist)| other_id != id && graph[other_id].flow_rate > 0)
                .collect();
            (*id, dists)
        })
        .collect();

    let mut remaining_minutes: u64 = 30;
    let mut current_valve = "AA";
    let mut open_valves = vec![];
    let mut total_pressure_released = 0;
    loop {
        let next_valve = pick_next_valve(
            &distances,
            current_valve,
            &open_valves,
            remaining_minutes,
            &graph,
        );
        let Some(next_valve_id) = next_valve else {
            break
        };
        let next_valve_dist = distances[current_valve][next_valve_id];
        println!("next valve {next_valve_id} (distance={next_valve_dist}), remaining minutes {remaining_minutes}");
        current_valve = next_valve_id;
        remaining_minutes -= next_valve_dist as u64 + 1;
        open_valves.push(next_valve_id);
        total_pressure_released += remaining_minutes * graph[next_valve_id].flow_rate;
        if remaining_minutes <= 0 {
            break;
        }
    }

    format!("{total_pressure_released}")
}

const PLAN_AHEAD_MINUTES: i32 = 10;

fn pick_next_valve<'a>(
    distances: &'a HashMap<&str, HashMap<&str, usize>>,
    starting_valve: &str,
    open_valves: &Vec<&str>,
    remaining_minutes: u64,
    graph: &HashMap<&str, Valve>,
) -> Option<&'a str> {
    let possible_paths = get_all_possible_paths(
        graph,
        distances,
        starting_valve,
        open_valves,
        remaining_minutes.min(PLAN_AHEAD_MINUTES as u64) as i32,
    );
    // println!(
    //     "possible paths starting at {starting_valve} {:#?}",
    //     possible_paths
    // );

    let best_path = possible_paths.iter().max_by_key(|path| {
        let mut released_pressure = 0;
        let mut current_valve = starting_valve;
        let mut remaining_minutes = remaining_minutes.min(PLAN_AHEAD_MINUTES as u64);
        for valve_id in path.iter() {
            let dist = distances[current_valve][valve_id];
            if dist as u64 + 1 > remaining_minutes {
                return 0;
            }
            remaining_minutes -= dist as u64 + 1;
            released_pressure += remaining_minutes * graph[valve_id].flow_rate;
            current_valve = valve_id;
        }
        // println!(
        //     "going from {starting_valve} to {} will yield {released_pressure}",
        //     path[0]
        // );
        released_pressure
    })?;
    Some(best_path[0])
}

fn get_all_possible_paths<'a>(
    graph: &HashMap<&str, Valve>,
    distances: &'a HashMap<&str, HashMap<&str, usize>>,
    starting_valve: &str,
    open_valves: &[&str],
    max_path_minutes: i32,
) -> Vec<Vec<ValveId<'a>>> {
    assert!(max_path_minutes >= 0);
    if max_path_minutes == 0 {
        return vec![];
    }
    let res = distances[starting_valve]
        // TODO: don't use non-deterministic HashMap iteration
        .iter()
        .filter(|(id, _dist)| !open_valves.contains(id))
        .filter(|(_id, dist)| **dist as i32 + 1 <= max_path_minutes)
        .flat_map(|(id, dist)| {
            let mut open_valves = open_valves.to_vec();
            open_valves.push(id);

            let mut paths = get_all_possible_paths(
                graph,
                distances,
                *id,
                &open_valves,
                max_path_minutes - *dist as i32 - 1,
            );
            if paths.is_empty() {
                return vec![vec![*id]];
            }
            for path in paths.iter_mut() {
                path.insert(0, id);
            }
            paths
        })
        .collect();
    // println!(
    //     "possible paths starting at {starting_valve} with minutes {max_path_minutes} {:#?}",
    //     res
    // );
    res
}

type ValveId<'a> = &'a str;

#[derive(Debug)]
struct Valve<'a> {
    flow_rate: u64,
    connected_valves: Vec<ValveId<'a>>,
}

fn parse_graph(input: &str) -> HashMap<ValveId, Valve> {
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
