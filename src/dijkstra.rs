use crate::graph::{GraphList, Node};
use ordered_float::OrderedFloat;
use std::cmp::{self, Ordering, Reverse};
use std::collections::BinaryHeap;
// Boilerplate for state + min heap taken from here:
// https://doc.rust-lang.org/nightly/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: OrderedFloat<f64>,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
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
        Some(self.cmp(other))
    }
}
pub fn dijkstra(g: &GraphList, start: usize) -> Vec<OrderedFloat<f64>> {
    let mut costs = vec![OrderedFloat(f64::INFINITY); g.num_nodes()];
    costs[start] = 0_f64.into();
    let mut queue = BinaryHeap::new();
    let start = State {
        cost: 0_f64.into(),
        position: start,
    };
    queue.push(start);
    for i in 0..g.num_nodes() {
        if i != start.position {
            queue.push(State {
                cost: f64::INFINITY.into(),
                position: i,
            })
        }
    }
    while let Some(State { position, .. }) = queue.pop() {
        let node = &g.nodes[position];
        for e in node.get_edge_list() {
            let neighbor = e.to;
            let node_not_visited = queue
                .iter()
                .find(|State { position, .. }| *position == neighbor)
                .is_some();
            dbg!(e, node_not_visited);
            if node_not_visited {
                let new_cost = costs[position] + e.weight;
                dbg!(&costs);
                dbg!(new_cost);
                if new_cost < costs[neighbor] {
                    let state = State {
                        cost: new_cost,
                        position: neighbor,
                    };
                    costs[neighbor] = new_cost;
                    queue.push(state);
                }
            }
        }
    }
    return costs;
}
#[cfg(test)]
mod dijkstra_tests {
    use super::*;
    use ordered_float::OrderedFloat;

    fn create_weighted_graph() -> GraphList {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![
                Node::new(0, None),
                Node::new(1, None),
                Node::new(2, None),
                Node::new(3, None),
                Node::new(4, None),
            ],
        };

        // Directed weighted edges
        graph.insert_edge(0, 1, 4.0).unwrap();
        graph.insert_edge(0, 2, 1.0).unwrap();
        graph.insert_edge(1, 3, 1.0).unwrap();
        graph.insert_edge(2, 1, 2.0).unwrap();
        graph.insert_edge(2, 3, 5.0).unwrap();
        graph.insert_edge(3, 4, 3.0).unwrap();
        graph
    }

    #[test]
    fn test_shortest_paths_from_source() {
        let graph = create_weighted_graph();
        let distances = dijkstra(&graph, 0);

        assert_eq!(
            distances,
            vec![
                OrderedFloat(0.0), // 0 to 0
                OrderedFloat(3.0), // 0->2->1 (1+2)
                OrderedFloat(1.0), // 0->2
                OrderedFloat(4.0), // 0->2->1->3 (1+2+1)
                OrderedFloat(7.0)  // 0->2->1->3->4 (1+2+1+3)
            ]
        );
    }

    #[test]
    fn test_disconnected_nodes() {
        let mut graph = create_weighted_graph();
        // Add disconnected node
        graph.insert_node(None);

        let distances = dijkstra(&graph, 0);
        assert_eq!(distances[5], OrderedFloat(f64::INFINITY));
    }

    #[test]
    fn test_all_nodes_unreachable() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None), Node::new(1, None)],
        };
        // No edges added

        let distances = dijkstra(&graph, 0);
        assert_eq!(
            distances,
            vec![OrderedFloat(0.0), OrderedFloat(f64::INFINITY)]
        );
    }

    #[test]
    fn test_multiple_shortest_paths() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![
                Node::new(0, None),
                Node::new(1, None),
                Node::new(2, None),
                Node::new(3, None),
            ],
        };

        graph.insert_edge(0, 1, 3.0).unwrap();
        graph.insert_edge(0, 2, 2.0).unwrap();
        graph.insert_edge(1, 3, 1.0).unwrap();
        graph.insert_edge(2, 3, 2.0).unwrap();

        let distances = dijkstra(&graph, 0);
        assert_eq!(distances[3], OrderedFloat(4.0)); // Both paths equal weight
    }

    #[test]
    fn test_single_node_graph() {
        let graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None)],
        };

        let distances = dijkstra(&graph, 0);
        assert_eq!(distances, vec![OrderedFloat(0.0)]);
    }

    #[test]
    fn test_cycle_handling() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None), Node::new(1, None), Node::new(2, None)],
        };

        graph.insert_edge(0, 1, 1.0).unwrap();
        graph.insert_edge(1, 2, 1.0).unwrap();
        graph.insert_edge(2, 0, 1.0).unwrap();

        let distances = dijkstra(&graph, 0);
        assert_eq!(
            distances,
            vec![
                OrderedFloat(0.0),
                OrderedFloat(1.0),
                OrderedFloat(2.0) // 0->1->2
            ]
        );
    }

    #[test]
    fn test_duplicate_edges() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None), Node::new(1, None)],
        };

        graph.insert_edge(0, 1, 5.0).unwrap();
        graph.insert_edge(0, 1, 2.0).unwrap(); // Lower weight

        let distances = dijkstra(&graph, 0);
        assert_eq!(distances[1], OrderedFloat(2.0));
    }
}
