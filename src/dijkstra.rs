use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

// Calculates the shortest path between two nodes using Dijkstra's algorithm.
pub fn shortest_path<T, FT, IT>(start: &T, end: &T, successors: FT) -> Option<usize>
where
    T: Eq + Hash + Clone,
    FT: Fn(&T) -> IT,
    IT: IntoIterator<Item = (T, usize)>,
{
    let mut unvisited = BinaryHeap::new();
    let mut distances = HashMap::new();

    distances.insert(start.clone(), 0);
    unvisited.push(Node {
        value: start.clone(),
        cost: 0,
    });

    while let Some(min_cost_node) = unvisited.pop() {
        let Node { value, cost } = min_cost_node;

        if &value == end {
            return Some(cost);
        }

        if distances[&value] < cost {
            continue;
        }

        for (succ, succ_cost) in successors(&value) {
            let path_cost = cost + succ_cost;
            if path_cost < *distances.get(&succ).unwrap_or(&usize::MAX) {
                distances.insert(succ.clone(), path_cost);
                unvisited.push(Node {
                    value: succ,
                    cost: path_cost,
                });
            }
        }
    }
    None
}

#[derive(PartialEq, Eq)]
struct Node<T: Eq> {
    value: T,
    cost: usize,
}

impl<T: Eq> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare cost in the other way so the binary heap is min-sorted.
        other.cost.cmp(&self.cost)
    }
}

// Rust requires this for some reason too.
impl<T: Eq> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
