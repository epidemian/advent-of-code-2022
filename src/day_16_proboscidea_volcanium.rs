use crate::dijkstra::shortest_path;
use fxhash::FxHashMap as HashMap;
use rayon::prelude::*;
use std::mem;

// Note: this solution was ~stolen from~ heavily inspired by
// https://old.reddit.com/r/adventofcode/comments/zn6k1l/2022_day_16_solutions/j2xhog7/
pub fn run(input: &str) -> String {
    let (valves, start_valve_id) = parse_valves(input);
    let distances = valves
        .iter()
        .map(|valve| {
            valves
                .iter()
                .filter(|&other_valve| other_valve.id != valve.id && other_valve.flow_rate > 0)
                .map(|other_valve| {
                    let successors = |&id: &ValveId| valves[id].connected_valves.iter().cloned();
                    let d =
                        shortest_path(&valve.id, |&id| id == other_valve.id, successors).unwrap();
                    (other_valve.id, d)
                })
                .collect()
        })
        .collect();

    let mut bit_index = 0;
    let valve_bitmasks = valves
        .iter()
        .map(|valve| {
            if valve.flow_rate == 0 {
                return 0;
            }
            assert!(bit_index < BITMASK_BITS, "too many valves for bitmask size");
            bit_index += 1;
            1 << (bit_index - 1)
        })
        .collect();

    let part_1_max_pressures =
        visit_all_paths(start_valve_id, 30, &valves, &distances, &valve_bitmasks);
    let part_1_ans = part_1_max_pressures.values().max().unwrap();

    let part_2_max_pressures =
        visit_all_paths(start_valve_id, 26, &valves, &distances, &valve_bitmasks);
    let part_2_ans = part_2_max_pressures
        .par_iter()
        .flat_map(|(bitmask_1, pressure_1)| {
            part_2_max_pressures
                .par_iter()
                .filter(move |&(bitmask_2, _pressure_2)| (bitmask_1 & bitmask_2) == 0)
                .map(move |(_bitmask_2, pressure_2)| pressure_1 + pressure_2)
        })
        .max()
        .unwrap_or(0);

    format!("{part_1_ans} {part_2_ans}")
}

type ValveId = usize;
type Bitmask = u16;
const BITMASK_BITS: usize = mem::size_of::<Bitmask>() * 8;

struct Valve {
    id: ValveId,
    flow_rate: u64,
    connected_valves: Vec<ValveId>,
}

fn visit_all_paths(
    initial_valve: ValveId,
    total_minutes: u64,
    valves: &Vec<Valve>,
    distances: &Vec<Vec<(ValveId, usize)>>,
    valve_bitmasks: &Vec<Bitmask>,
) -> HashMap<Bitmask, u64> {
    let mut max_pressures = HashMap::default();
    let mut stack = vec![(initial_valve, total_minutes, 0, 0)];

    while let Some(state) = stack.pop() {
        let (current_valve, remaining_minutes, open_valves_bitmask, released_pressure) = state;

        let max_val = max_pressures.entry(open_valves_bitmask).or_insert(0);
        if released_pressure > *max_val {
            *max_val = released_pressure;
        }

        for &(other_valve, dist) in distances[current_valve].iter() {
            let not_enough_time = dist as u64 + 1 > remaining_minutes;
            let already_open = valve_bitmasks[other_valve] & open_valves_bitmask != 0;
            if not_enough_time || already_open {
                continue;
            }
            let new_remaining_minutes = remaining_minutes - dist as u64 - 1;
            stack.push((
                other_valve,
                new_remaining_minutes,
                open_valves_bitmask | valve_bitmasks[other_valve],
                released_pressure + new_remaining_minutes * valves[other_valve].flow_rate,
            ));
        }
    }

    max_pressures
}

fn parse_valves(input: &str) -> (Vec<Valve>, ValveId) {
    let words_by_line: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.split([' ', ';', '=', ','])
                .filter(|s| !s.is_empty())
                .collect()
        })
        .collect();
    let valve_ids: HashMap<_, _> = words_by_line
        .iter()
        .enumerate()
        .map(|(i, words)| (words[1], i))
        .collect();
    let valves = words_by_line
        .iter()
        .enumerate()
        .map(|(id, words)| Valve {
            id,
            flow_rate: words[5].parse().expect("invalid number"),
            connected_valves: words[10..].iter().map(|s| valve_ids[s]).collect(),
        })
        .collect();
    (valves, valve_ids["AA"])
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        assert_eq!(super::run(SAMPLE), "1651 1707")
    }
    const SAMPLE: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";
}
