use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

// Calculates the shortest path distances starting from a given node using
// Dijkstra's algorithm.
pub fn shortest_path_distances<T, FT, IT>(start: &T, successors: FT) -> HashMap<T, usize>
where
    T: Eq + Hash + Clone,
    FT: Fn(&T) -> IT,
    IT: IntoIterator<Item = T>,
{
    let mut unvisited = BinaryHeap::new();
    let mut distances = HashMap::new();

    distances.insert(start.clone(), 0);
    unvisited.push(Node {
        value: start.clone(),
        distance: 0,
    });

    while let Some(min_dist_node) = unvisited.pop() {
        let Node { value, distance } = min_dist_node;

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

    distances
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
