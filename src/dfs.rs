use crate::graph::{GraphList, GraphMatrix, Node};

pub fn dfs_recursive(g: &GraphList, ind: usize, seen: &mut Vec<bool>) {
    seen[ind] = true;
    let current = &g.nodes[ind];

    for e in current.get_edge_list() {
        let neighbour = e.to;
        if !seen[neighbour] {
            dfs_recursive(g, neighbour, seen)
        }
    }
}
pub fn dfs(g: &GraphList, start: usize) {
    let mut seen = vec![false; g.num_nodes()];
    dfs_recursive(g, start, &mut seen);
}
pub fn dfs_all(g: &GraphList) {
    let mut seen = vec![false; g.num_nodes()];
    for i in 0..g.num_nodes() {
        if !seen[i] {
            dfs_recursive(g, i, &mut seen)
        }
    }
}

pub fn dfs_stack(g: &GraphList, start: usize) -> Vec<i64> {
    let mut seen = vec![false; g.num_nodes()];
    let mut last = vec![-1_i64; g.num_nodes()];
    // This is the 'stack'
    let mut to_explore = Vec::new();
    to_explore.push(start);
    while !to_explore.is_empty() {
        let ind = to_explore.pop().unwrap();
        if !seen[ind] {
            let current = &g.nodes[ind];
            seen[ind] = true;
            let mut edges = current.get_ordered_edge_list();
            edges.reverse();
            for e in edges {
                let neighbor = e.to;
                if !seen[neighbor] {
                    last[neighbor] = ind as i64;
                    to_explore.push(neighbor)
                }
            }
        }
    }
    return last;
}

pub fn dfs_recursive_connected_componentes(
    g: &GraphList,
    ind: usize,
    component: &mut Vec<isize>,
    curr_comp: isize,
) {
    component[ind] = curr_comp;
    for e in g.nodes[ind].get_edge_list() {
        let neighbor = e.to;
        if component[neighbor] == -1 {
            dfs_recursive_connected_componentes(g, neighbor, component, curr_comp)
        }
    }
}

pub fn dfs_connected_componentes(g: &GraphList) -> Vec<isize> {
    let mut component = vec![-1; g.num_nodes()];
    let mut curr_comp = 0;
    for ind in 0..g.num_nodes() {
        if component[ind] == -1 {
            dfs_recursive_connected_componentes(g, ind, &mut component, curr_comp);
            curr_comp += 1;
        }
    }
    return component;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dfs_functions() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None), Node::new(1, None), Node::new(2, None)],
        };
        graph.insert_edge(0, 1, 1.0).unwrap();
        graph.insert_edge(1, 2, 1.0).unwrap();

        // Test DFS
        let mut seen = vec![false; graph.num_nodes()];
        dfs_recursive(&graph, 0, &mut seen);
        assert!(seen[0]);
        assert!(seen[1]);
        assert!(seen[2]);

        // Test DFS stack
        let last = dfs_stack(&graph, 0);
        assert_eq!(last, vec![-1, 0, 1]);

        // Test connected components
        let components = dfs_connected_componentes(&graph);
        assert_eq!(components, vec![0, 0, 0]);

        // Add a disconnected node
        graph.insert_node(None);
        let components = dfs_connected_componentes(&graph);
        assert_eq!(components, vec![0, 0, 0, 1]);
    }

    #[test]
    fn test_empty_graph() {
        let graph = GraphList {
            undirected: false,
            nodes: vec![],
        };

        assert_eq!(graph.num_nodes(), 0);
        assert!(graph.get_edge(0, 1).is_err());

        let components = dfs_connected_componentes(&graph);
        assert!(components.is_empty());
    }

    #[test]
    fn test_single_node_graph() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, Some("A".to_string()))],
        };

        assert_eq!(graph.num_nodes(), 1);
        assert!(!graph.is_edge(0, 0));

        graph.insert_edge(0, 0, 1.0).unwrap();
        assert!(graph.is_edge(0, 0));

        let components = dfs_connected_componentes(&graph);
        assert_eq!(components, vec![0]);
    }
}
