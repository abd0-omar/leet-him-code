// https://leetcode.com/problems/maximum-number-of-k-divisible-components/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let n = n as usize;
        let mut adj_graph = vec![vec![]; n];
        for edge in edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            adj_graph[from].push(to);
            adj_graph[to].push(from);
        }
        let mut result = 0;
        // any node can be the root node (undirected graph)
        dfs(&adj_graph, &values, 0, -1, &mut result, k);
        result
    }
}

fn dfs(
    adj_graph: &[Vec<usize>],
    values: &[i32],
    curr: usize,
    parent: i32,
    result: &mut i32,
    k: i32,
) -> i64 {
    let mut total = values[curr] as i64;

    for &child in adj_graph[curr].iter() {
        // avoid cycles
        if child as i32 != parent {
            total += dfs(adj_graph, values, child, curr as i32, result, k);
        }
    }

    if total % k as i64 == 0 {
        *result += 1;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 5;
        let edges = vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]];
        let values = vec![1, 8, 1, 4, 4];
        let k = 6;
        let output = 2;
        let result = Solution::max_k_divisible_components(n, edges, values, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 5],
            vec![2, 6],
        ];
        let values = vec![3, 0, 6, 1, 5, 2, 1];
        let k = 3;
        let output = 3;
        let result = Solution::max_k_divisible_components(n, edges, values, k);
        assert_eq!(result, output);
    }
}
