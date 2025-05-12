use ordered_float::OrderedFloat;

use crate::graph::{Edge, GraphList, Node};

pub fn floyd_warshall(g: &GraphList) -> Vec<Vec<isize>> {
    let n = g.num_nodes();
    let mut cost: Vec<Vec<OrderedFloat<f64>>> = vec![vec![f64::INFINITY.into(); n]; n];
    let mut last: Vec<Vec<isize>> = vec![vec![-1; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                cost[i][j] = 0.into();
            } else {
                let e = g.get_edge(i, j).unwrap();
                if let Some(Edge { weight, .. }) = e {
                    cost[i][j] = weight;
                    last[i][j] = i as isize;
                }
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if cost[i][j] > cost[i][k] + cost[k][j] {
                    cost[i][j] = cost[i][k] + cost[k][j];
                    last[i][j] = last[k][j];
                }
            }
        }
    }
    return last;
}
