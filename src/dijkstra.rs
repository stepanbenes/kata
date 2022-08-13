use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u16,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

// Each node is represented as an `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: u16,
}

fn longest_path(adj_list: &Vec<Vec<Edge>>, start: usize) -> u16 {
    // dist[node] = current largest distance from `start` to `node`
    let mut dist: Vec<_> = vec![0; adj_list.len()];

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with larger cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Important as we may have already found a better way
        if cost < dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a larger cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost > dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    let mut index_of_largest = 0;
    for i in 0..dist.len() {
        if dist[index_of_largest] < dist[i] {
            index_of_largest = i;
        }
    }

    dist[index_of_largest]
}

pub fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    let mut graph = vec![vec![Edge {
        node: 1,
        cost: pyramid[0][0],
    }]];
    let mut node_index = 1;
    for row in 0..pyramid.len() {
        for column in 0..pyramid[row].len() {
            let mut edges = Vec::<Edge>::new();
            if row < pyramid.len() - 1 {
                let left_child = node_index + pyramid[row].len();
                let right_child = node_index + pyramid[row].len() + 1;
                edges.push(Edge {
                    node: left_child,
                    cost: pyramid[row + 1][column],
                });
                edges.push(Edge {
                    node: right_child,
                    cost: pyramid[row + 1][column + 1],
                });
            }
            graph.push(edges);
            node_index += 1;
        }
    }

    longest_path(&graph, 0)
}
