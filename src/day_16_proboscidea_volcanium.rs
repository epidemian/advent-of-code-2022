use crate::dijkstra::shortest_path;
use fxhash::FxHashMap as HashMap;
use rayon::prelude::*;
use std::mem;

// Note: this solution was ~stolen from~ heavily inspired by
// https://old.reddit.com/r/adventofcode/comments/zn6k1l/2022_day_16_solutions/j2xhog7/
pub fn run(input: &str) -> String {
    let graph = parse_graph(input);
    let distances = graph
        .keys()
        .map(|id| {
            let dists = graph
                .iter()
                .filter(|&(other_id, valve)| other_id != id && valve.flow_rate > 0)
                .map(|(other_id, _v)| {
                    let successors = |id: &_| graph[id].connected_valves.iter().cloned();
                    let d = shortest_path(id, |id| id == other_id, successors).unwrap();
                    (*other_id, d)
                })
                .collect();
            (*id, dists)
        })
        .collect();

    let bitmask_indices = graph
        .iter()
        .filter(|(_id, valve)| valve.flow_rate > 0)
        .enumerate()
        .map(|(i, (id, _valve))| {
            assert!(i < BITMASK_BITS, "too many valves to fit on bitmask");
            (*id, 1 << i)
        })
        .collect();

    let ctx = Context {
        graph,
        distances,
        bitmask_indices,
    };

    let start_valve = (b'A', b'A');
    let mut part_1_max_pressures = HashMap::default();
    visit_all_paths(&ctx, start_valve, 30, 0, 0, &mut part_1_max_pressures);
    let part_1_ans = part_1_max_pressures.values().max().unwrap();

    let mut part_2_max_pressures = HashMap::default();
    visit_all_paths(&ctx, start_valve, 26, 0, 0, &mut part_2_max_pressures);
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

struct Valve {
    flow_rate: u64,
    connected_valves: Vec<ValveId>,
}

// Bunch up some structures to avoid passing too many parameters around.
struct Context {
    graph: HashMap<ValveId, Valve>,
    distances: HashMap<ValveId, HashMap<ValveId, usize>>,
    bitmask_indices: HashMap<ValveId, Bitmask>,
}

// Prefer 2-byte tuples for IDs instead of &str like "AA" to avoid adding lifetime annotations.
type ValveId = (u8, u8);
type Bitmask = u16;
const BITMASK_BITS: usize = mem::size_of::<Bitmask>() * 8;

fn visit_all_paths(
    ctx: &Context,
    current_valve: ValveId,
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

    for (other_valve, &dist) in ctx.distances[&current_valve].iter() {
        if dist as u64 + 1 > remaining_minutes
            || (ctx.bitmask_indices[other_valve] & open_valves_bitmask) != 0
        {
            continue;
        }
        let remaining_minutes = remaining_minutes - dist as u64 - 1;
        visit_all_paths(
            ctx,
            *other_valve,
            remaining_minutes,
            open_valves_bitmask | ctx.bitmask_indices[other_valve],
            released_pressure + remaining_minutes * ctx.graph[other_valve].flow_rate,
            max_released_pressure,
        )
    }
}

fn parse_graph(input: &str) -> HashMap<ValveId, Valve> {
    input
        .lines()
        .map(|line| {
            let words: Vec<_> = line
                .split([' ', ';', '=', ','])
                .filter(|s| !s.is_empty())
                .collect();
            let valve_id = parse_valve_id(words[1]);
            let flow_rate = words[5].parse().expect("invalid number");
            let connected_valves = words[10..].iter().map(|s| parse_valve_id(s)).collect();
            let valve = Valve {
                flow_rate,
                connected_valves,
            };
            (valve_id, valve)
        })
        .collect()
}

fn parse_valve_id(s: &str) -> ValveId {
    let bytes = s.as_bytes();
    (bytes[0], bytes[1])
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
