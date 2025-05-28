// https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        let adj_list1 = build_adj_list(edges1, n);
        let adj_list2 = build_adj_list(edges2, m);
        let mut result = vec![0; n];

        let mut max_tree2 = 0;
        for node in 0..m {
            max_tree2 = max_tree2.max(dfs(&adj_list2, k - 1, node, usize::MAX));
        }

        for node in 0..n {
            result[node] = dfs(&adj_list1, k, node, usize::MAX) + max_tree2;
        }
        result
    }
}

fn dfs(adj_list: &Vec<Vec<usize>>, k: i32, node: usize, parent: usize) -> i32 {
    if k < 0 {
        return 0;
    }
    let mut count = 1;
    for &neighbor in &adj_list[node] {
        if neighbor != parent {
            count += dfs(adj_list, k - 1, neighbor, node);
        }
    }
    count
}

fn build_adj_list(edges: Vec<Vec<i32>>, n: usize) -> Vec<Vec<usize>> {
    let mut adj_list = vec![vec![]; n];
    for edge in edges {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        adj_list[from].push(to);
        adj_list[to].push(from);
    }
    adj_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let edges1 = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]];
        let edges2 = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![2, 7],
            vec![1, 4],
            vec![4, 5],
            vec![4, 6],
        ];
        let k = 2;
        let output = vec![9, 7, 9, 8, 8];
        let result = Solution::max_target_nodes(edges1, edges2, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let edges1 = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]];
        let edges2 = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let k = 1;
        let output = vec![6, 3, 3, 3, 3];
        let result = Solution::max_target_nodes(edges1, edges2, k);
        assert_eq!(result, output);
    }
}
