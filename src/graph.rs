use ordered_float::OrderedFloat;
use std::{collections::HashMap, hash::Hash};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: OrderedFloat<f64>,
}

impl Edge {
    pub fn new(from: usize, to: usize, weight: f64) -> Self {
        Edge {
            from,
            to,
            weight: weight.into(),
        }
    }
}

#[derive(Clone)]
pub struct Node {
    index: usize,
    edges: HashMap<usize, Edge>,
    pub(crate) label: Option<String>,
}

impl Node {
    pub fn new(index: usize, label: Option<String>) -> Self {
        Node {
            index,
            edges: Default::default(),
            label,
        }
    }
    pub fn num_edges(&self) -> usize {
        return self.edges.len();
    }
    pub fn get_edge(&self, index: usize) -> Option<Edge> {
        self.edges.get(&index).cloned()
    }
    pub fn add_edge(&mut self, neighbor: usize, weight: f64) {
        self.edges
            .insert(neighbor, Edge::new(self.index, neighbor, weight));
    }
    pub fn remove_edge(&mut self, neighbor: usize) {
        self.edges.remove(&neighbor);
    }
    pub fn get_edge_list(&self) -> Vec<&Edge> {
        self.edges.values().collect()
    }
    pub fn get_ordered_edge_list(&self) -> Vec<&Edge> {
        let mut edges: Vec<_> = self.edges.iter().collect();
        edges.sort_by(|(x, _), (y, _)| x.cmp(y));
        edges.into_iter().map(|(_, e)| e).collect()
    }
}

#[derive(Clone)]
pub struct GraphList {
    pub undirected: bool,
    pub nodes: Vec<Node>,
}

impl GraphList {
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
    fn valid_indices(&self, from: usize, to: usize) -> bool {
        ((0..self.num_nodes()).contains(&from) && (0..self.num_nodes()).contains(&to))
    }
    pub fn get_edge(&self, from: usize, to: usize) -> Result<Option<Edge>, String> {
        if !self.valid_indices(from, to) {
            Err(format!("Node out of range: from: {from}, to: {to}"))
        } else {
            Ok(self.nodes[from].get_edge(to))
        }
    }
    pub fn is_edge(&self, from: usize, to: usize) -> bool {
        self.get_edge(from, to).is_ok_and(|edge| edge.is_some())
    }
    pub fn make_edge_list(&self) -> Vec<&Edge> {
        let mut edges = vec![];
        for node in &self.nodes {
            edges.extend(node.get_edge_list())
        }
        return edges;
    }
    pub fn insert_edge(&mut self, from: usize, to: usize, weight: f64) -> Result<(), String> {
        if !self.valid_indices(from, to) {
            Err(format!("Node out of range: from: {from}, to: {to}"))
        } else {
            self.nodes[from].add_edge(to, weight);
            if self.undirected {
                self.nodes[to].add_edge(from, weight);
            }
            Ok(())
        }
    }
    pub fn remove_edge(&mut self, from: usize, to: usize) -> Result<(), String> {
        if !self.valid_indices(from, to) {
            Err(format!("Node out of range: from: {from}, to: {to}"))
        } else {
            self.nodes[from].remove_edge(to);
            if self.undirected {
                self.nodes[to].remove_edge(from);
            }
            Ok(())
        }
    }
    pub fn insert_node(&mut self, label: Option<String>) -> &Node {
        self.nodes.push(Node::new(self.num_nodes(), label));
        self.nodes.last().unwrap()
    }
}

pub struct GraphMatrix<const Nodes: usize> {
    undirected: bool,
    connections: [[OrderedFloat<f64>; Nodes]; Nodes],
}

impl<const Nodes: usize> GraphMatrix<Nodes> {
    pub fn new(undirected: bool) -> Self {
        GraphMatrix {
            undirected,
            connections: [[OrderedFloat(0.0); Nodes]; Nodes],
        }
    }
    pub fn get_edge(&self, from: usize, to: usize) -> Option<OrderedFloat<f64>> {
        self.connections.get(from)?.get(to).copied()
    }
    pub fn set_edge(
        &mut self,
        from: usize,
        to: usize,
        weight: OrderedFloat<f64>,
    ) -> Result<(), String> {
        let mut row = self
            .connections
            .get_mut(from)
            .ok_or(format!("From out of range: {from}"))?;
        let mut connection = row.get_mut(to).ok_or(format!("To out of range: {to}"))?;
        *connection = weight;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ordered_float::OrderedFloat;

    #[test]
    fn test_edge_creation() {
        let edge = Edge::new(1, 2, 3.5);
        assert_eq!(edge.from, 1);
        assert_eq!(edge.to, 2);
        assert_eq!(edge.weight, OrderedFloat(3.5));
    }

    #[test]
    fn test_node_creation() {
        let node = Node::new(0, Some("A".to_string()));
        assert_eq!(node.index, 0);
        assert_eq!(node.label, Some("A".to_string()));
        assert_eq!(node.num_edges(), 0);
    }

    #[test]
    fn test_node_edge_operations() {
        let mut node = Node::new(0, None);

        // Test adding edges
        node.add_edge(1, 1.0);
        node.add_edge(2, 2.0);
        assert_eq!(node.num_edges(), 2);

        // Test getting edges
        assert!(node.get_edge(1).is_some());
        assert!(node.get_edge(3).is_none());

        // Test edge list
        let edges = node.get_edge_list();
        assert_eq!(edges.len(), 2);

        // Test ordered edge list
        let ordered_edges = node.get_ordered_edge_list();
        assert_eq!(ordered_edges[0].to, 1);
        assert_eq!(ordered_edges[1].to, 2);

        // Test removing edge
        node.remove_edge(1);
        assert_eq!(node.num_edges(), 1);
    }

    #[test]
    fn test_graph_list_operations() {
        let mut graph = GraphList {
            undirected: false,
            nodes: vec![Node::new(0, None), Node::new(1, None)],
        };

        // Test inserting edge
        assert!(graph.insert_edge(0, 1, 1.0).is_ok());
        assert!(graph.is_edge(0, 1));
        assert!(!graph.is_edge(1, 0)); // Directed graph

        // Test invalid edge insertion
        assert!(graph.insert_edge(0, 2, 1.0).is_err());

        // Test removing edge
        assert!(graph.remove_edge(0, 1).is_ok());
        assert!(!graph.is_edge(0, 1));

        // Test inserting node
        let new_node = graph.insert_node(Some("C".to_string()));
        assert_eq!(new_node.index, 2);
        assert_eq!(new_node.label, Some("C".to_string()));
    }

    #[test]
    fn test_undirected_graph() {
        let mut graph = GraphList {
            undirected: true,
            nodes: vec![Node::new(0, None), Node::new(1, None)],
        };

        assert!(graph.insert_edge(0, 1, 1.0).is_ok());
        assert!(graph.is_edge(0, 1));
        assert!(graph.is_edge(1, 0)); // Undirected graph

        assert!(graph.remove_edge(0, 1).is_ok());
        assert!(!graph.is_edge(0, 1));
        assert!(!graph.is_edge(1, 0));
    }

    #[test]
    fn test_graph_matrix() {
        let mut matrix: GraphMatrix<3> = GraphMatrix::new(false);

        // Test setting and getting edges
        assert!(matrix.set_edge(0, 1, OrderedFloat(2.5)).is_ok());
        assert_eq!(matrix.get_edge(0, 1), Some(OrderedFloat(2.5)));
        assert_eq!(matrix.get_edge(1, 0), Some(OrderedFloat(0.0))); // Default value

        // Test invalid indices
        assert!(matrix.set_edge(3, 0, OrderedFloat(1.0)).is_err());
        assert_eq!(matrix.get_edge(3, 0), None);
    }
}
