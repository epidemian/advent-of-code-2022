use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;

use fxhash::FxHashMap as HashMap;

// Calculates the shortest path distance between a given start node and a goal using Dijkstra's
// algorithm. The goal is given as a predicate function instead of a node so that the caller can
// determine when it is reached.
pub fn shortest_path<T, PT, FT, IT>(start: &T, is_goal: PT, successors: FT) -> Option<usize>
where
    T: Eq + Hash + Clone,
    PT: Fn(&T) -> bool,
    FT: Fn(&T) -> IT,
    IT: IntoIterator<Item = T>,
{
    let mut unvisited = BinaryHeap::new();
    let mut distances = HashMap::default();

    distances.insert(start.clone(), 0);
    unvisited.push(Node {
        value: start.clone(),
        distance: 0,
    });

    while let Some(min_dist_node) = unvisited.pop() {
        let Node { value, distance } = min_dist_node;

        if is_goal(&value) {
            return Some(distance);
        }

        if distances[&value] < distance {
            continue;
        }

        for succ in successors(&value) {
            let path_dist = distance + 1;
            if path_dist < *distances.get(&succ).unwrap_or(&usize::MAX) {
                distances.insert(succ.clone(), path_dist);
                unvisited.push(Node {
                    value: succ,
                    distance: path_dist,
                });
            }
        }
    }
    None
}

#[derive(PartialEq, Eq)]
struct Node<T: Eq> {
    value: T,
    distance: usize,
}

impl<T: Eq> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare distances in the other way so the binary heap is min-sorted.
        other.distance.cmp(&self.distance)
    }
}

// Rust requires this for some reason too.
impl<T: Eq> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
