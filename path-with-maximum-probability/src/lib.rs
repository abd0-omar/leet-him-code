use std::collections::BinaryHeap;

use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
struct ProbWrapper(f64);

impl PartialEq for ProbWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ProbWrapper {}

impl PartialOrd for ProbWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for ProbWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

pub fn max_probability(
    n: i32,
    edges: Vec<Vec<i32>>,
    succ_prob: Vec<f64>,
    start_node: i32,
    end_node: i32,
) -> f64 {
    let mut adj_list = vec![vec![]; n as usize];
    let mut succ_prob_iter = succ_prob.into_iter();
    for edge in edges {
        let from = edge[0];
        let to = edge[1];
        let weight = succ_prob_iter.next().unwrap();

        adj_list[from as usize].push((weight, to));
        adj_list[to as usize].push((weight, from));
    }

    let mut pq = BinaryHeap::new();
    pq.push((ProbWrapper(1.0), start_node));

    let mut probabilities = vec![0.0; n as usize];
    probabilities[start_node as usize] = 1.0;

    while let Some((ProbWrapper(cur_prob), cur_node)) = pq.pop() {
        if cur_node == end_node {
            return cur_prob;
        }

        for &(neighbor_prob, neighbor_node) in &adj_list[cur_node as usize] {
            let new_prob = cur_prob * neighbor_prob;

            if new_prob > probabilities[neighbor_node as usize] {
                probabilities[neighbor_node as usize] = new_prob;
                pq.push((ProbWrapper(new_prob), neighbor_node));
            }
        }
    }

    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
        let succ_prob = vec![0.5, 0.5, 0.3];
        let start_node = 0;
        let end_node = 2;
        let output = 0.30000;
        let result = max_probability(n, edges, succ_prob, start_node, end_node);
        assert!((result - output).abs() < 1e-5);
    }
}
