use crate::graph::{GraphList, Node};
use std::collections::VecDeque;

pub fn bfs(g: &GraphList, start: usize) -> Vec<isize> {
    let mut seen = vec![false; g.num_nodes()];
    let mut last = vec![-1_isize; g.num_nodes()];
    let mut pending = VecDeque::new();
    pending.push_back(start);
    seen[start] = true;
    while !pending.is_empty() {
        let next = pending.pop_front().unwrap();
        for e in g.nodes[next].get_edge_list() {
            let neighbour = e.to;
            if !seen[neighbour] {
                seen[neighbour] = true;
                last[neighbour] = next as isize;
                pending.push_back(neighbour)
            }
        }
    }
    return last;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    // Helper function to create a simple directed graph for testing
    fn create_test_graph() -> GraphList {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![
                Node::new(0, None),
                Node::new(1, None),
                Node::new(2, None),
                Node::new(3, None),
            ],
        };

        graph.insert_edge(0, 1, 1.0).unwrap();
        graph.insert_edge(0, 2, 1.0).unwrap();
        graph.insert_edge(1, 3, 1.0).unwrap();
        graph.insert_edge(2, 3, 1.0).unwrap();

        graph
    }

    // Helper function to create a simple undirected graph for testing
    fn create_undirected_test_graph() -> GraphList {
        let mut graph = GraphList {
            undirected: true,
            nodes: vec![Node::new(0, None), Node::new(1, None), Node::new(2, None)],
        };

        graph.insert_edge(0, 1, 1.0).unwrap();
        graph.insert_edge(1, 2, 1.0).unwrap();

        graph
    }

    #[test]
    fn test_bfs_simple_directed_graph() {
        let graph = create_test_graph();
        let result = bfs(&graph, 0);

        // Expected parent relationships:
        // 0 -> 1, 0 -> 2
        // 1 -> 3, 2 -> 3
        assert_eq!(result, vec![-1, 0, 0, 1]);
    }

    #[test]
    fn test_bfs_simple_undirected_graph() {
        let graph = create_undirected_test_graph();
        let result = bfs(&graph, 0);

        // Expected parent relationships:
        // 0 -> 1
        // 1 -> 2
        assert_eq!(result, vec![-1, 0, 1]);
    }

    #[test]
    fn test_bfs_disconnected_graph() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None), Node::new(1, None), Node::new(2, None)],
        };

        graph.insert_edge(0, 1, 1.0).unwrap();
        // Node 2 is disconnected

        let result = bfs(&graph, 0);
        assert_eq!(result, vec![-1, 0, -1]);
    }

    #[test]
    fn test_bfs_single_node() {
        let graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None)],
        };

        let result = bfs(&graph, 0);
        assert_eq!(result, vec![-1]);
    }

    #[test]
    fn test_bfs_cyclic_graph() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None), Node::new(1, None), Node::new(2, None)],
        };

        graph.insert_edge(0, 1, 1.0).unwrap();
        graph.insert_edge(1, 2, 1.0).unwrap();
        graph.insert_edge(2, 0, 1.0).unwrap();

        let result = bfs(&graph, 0);
        assert_eq!(result, vec![-1, 0, 1]);
    }

    #[test]
    fn test_bfs_multiple_starting_points() {
        let graph = create_test_graph();

        // Start from node 1
        let result = bfs(&graph, 1);
        assert_eq!(result, vec![-1, -1, -1, 1]);

        // Start from node 2
        let result = bfs(&graph, 2);
        assert_eq!(result, vec![-1, -1, -1, 2]);
    }

    #[test]
    fn test_bfs_complete_graph() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None), Node::new(1, None), Node::new(2, None)],
        };

        // Create a complete graph (every node connected to every other node)
        graph.insert_edge(0, 1, 1.0).unwrap();
        graph.insert_edge(0, 2, 1.0).unwrap();
        graph.insert_edge(1, 0, 1.0).unwrap();
        graph.insert_edge(1, 2, 1.0).unwrap();
        graph.insert_edge(2, 0, 1.0).unwrap();
        graph.insert_edge(2, 1, 1.0).unwrap();

        let result = bfs(&graph, 0);
        assert_eq!(result, vec![-1, 0, 0]);
    }
}
