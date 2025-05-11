use crate::graph::{GraphList, Node};
use core::f64;
use ordered_float::OrderedFloat;
use std::cmp::{self, Ordering, Reverse};
use std::collections::BinaryHeap;

pub fn bellman_ford(g: &GraphList, start: usize) -> Option<Vec<OrderedFloat<f64>>> {
    let mut cost = vec![OrderedFloat(f64::INFINITY); g.num_nodes()];
    let mut last = vec![-1; g.num_nodes()];
    let all_edges = g.make_edge_list();
    cost[start] = 0.0_f64.into();
    for _ in 0..g.num_nodes() - 1 {
        for e in &all_edges {
            let cost_through_node = cost[e.from] + e.weight;
            if cost_through_node < cost[e.to] {
                cost[e.to] = cost_through_node;
                last[e.to] = e.from as isize;
            }
        }
    }
    for e in all_edges {
        if cost[e.to] > cost[e.from] + e.weight {
            return None;
        }
    }
    return Some(cost);
}
#[cfg(test)]
mod bellman_ford_tests {
    use super::*;
    use ordered_float::OrderedFloat;

    // Helper function to create a basic test graph
    fn create_basic_graph() -> GraphList {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![
                Node::new(0, None),
                Node::new(1, None),
                Node::new(2, None),
                Node::new(3, None),
            ],
        };

        graph.insert_edge(0, 1, 4.0).unwrap();
        graph.insert_edge(0, 2, 1.0).unwrap();
        graph.insert_edge(1, 3, 1.0).unwrap();
        graph.insert_edge(2, 1, 2.0).unwrap();
        graph.insert_edge(2, 3, 5.0).unwrap();
        graph
    }

    #[test]
    fn test_basic_shortest_paths() {
        let graph = create_basic_graph();
        let result = bellman_ford(&graph, 0).unwrap();

        assert_eq!(
            result,
            vec![
                OrderedFloat(0.0), // Start node
                OrderedFloat(3.0), // 0->2->1 (1+2)
                OrderedFloat(1.0), // 0->2
                OrderedFloat(4.0)  // 0->2->1->3 (1+2+1)
            ]
        );
    }

    #[test]
    fn test_negative_weights_no_cycle() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None), Node::new(1, None), Node::new(2, None)],
        };

        graph.insert_edge(0, 1, 4.0).unwrap();
        graph.insert_edge(0, 2, 5.0).unwrap();
        graph.insert_edge(2, 1, -2.0).unwrap();

        let result = bellman_ford(&graph, 0).unwrap();
        assert_eq!(
            result,
            vec![
                OrderedFloat(0.0),
                OrderedFloat(3.0), // 0->2->1 (5-2)
                OrderedFloat(5.0)
            ]
        );
    }

    #[test]
    fn test_negative_cycle_detection() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None), Node::new(1, None), Node::new(2, None)],
        };

        // Total cycle weight: 1 + 1 - 3 = -1
        graph.insert_edge(0, 1, 1.0).unwrap();
        graph.insert_edge(1, 2, 1.0).unwrap();
        graph.insert_edge(2, 0, -3.0).unwrap();

        assert!(bellman_ford(&graph, 0).is_none());
    }

    #[test]
    fn test_disconnected_nodes() {
        let mut graph = create_basic_graph();
        graph.insert_node(None); // Node 4

        let result = bellman_ford(&graph, 0).unwrap();
        assert_eq!(result[4], OrderedFloat(f64::INFINITY));
    }

    #[test]
    fn test_single_node_graph() {
        let graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None)],
        };

        let result = bellman_ford(&graph, 0).unwrap();
        assert_eq!(result, vec![OrderedFloat(0.0)]);
    }

    #[test]
    fn test_start_node_in_negative_cycle() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None), Node::new(1, None)],
        };

        // Negative cycle: 0->1->0 with total weight -2
        graph.insert_edge(0, 1, -1.0).unwrap();
        graph.insert_edge(1, 0, -1.0).unwrap();

        assert!(bellman_ford(&graph, 0).is_none());
    }

    #[test]
    fn test_multiple_negative_edges() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None), Node::new(1, None), Node::new(2, None)],
        };

        graph.insert_edge(0, 1, -2.0).unwrap();
        graph.insert_edge(1, 2, -3.0).unwrap();

        let result = bellman_ford(&graph, 0).unwrap();
        assert_eq!(
            result,
            vec![OrderedFloat(0.0), OrderedFloat(-2.0), OrderedFloat(-5.0)]
        );
    }

    #[test]
    fn test_zero_weight_cycle() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None), Node::new(1, None), Node::new(2, None)],
        };

        // Total cycle weight: 2 + 3 - 5 = 0
        graph.insert_edge(0, 1, 2.0).unwrap();
        graph.insert_edge(1, 2, 3.0).unwrap();
        graph.insert_edge(2, 0, -5.0).unwrap();

        let result = bellman_ford(&graph, 0).unwrap();
        assert_eq!(result[2], OrderedFloat(5.0)); // 0->1->2 (2+3)
    }

    #[test]
    fn test_start_node_unreachable() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None), Node::new(1, None)],
        };

        graph.insert_edge(1, 0, 1.0).unwrap(); // Edge from 1 to 0

        let result = bellman_ford(&graph, 0).unwrap();
        assert_eq!(result, vec![OrderedFloat(0.0), OrderedFloat(f64::INFINITY)]);
    }
}
