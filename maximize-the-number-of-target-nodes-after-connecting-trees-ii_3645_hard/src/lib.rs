// https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-ii/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        let adj_list1 = build_adj_list(&edges1, n);
        let adj_list2 = build_adj_list(&edges2, m);
        let mut color1 = vec![-1; n];
        let mut color2 = vec![-1; m];
        color1[0] = 0;
        color2[0] = 0;
        let mut even1 = 0;
        let mut odd1 = 0;
        let mut even2 = 0;
        let mut odd2 = 0;
        dfs(
            &adj_list1,
            0,
            usize::MAX,
            &mut color1,
            &mut even1,
            &mut odd1,
        );
        dfs(
            &adj_list2,
            0,
            usize::MAX,
            &mut color2,
            &mut even2,
            &mut odd2,
        );
        let mut result = vec![0; n];
        let max2 = even2.max(odd2);
        for i in 0..n {
            result[i] = if color1[i] == 0 {
                even1 + max2
            } else {
                odd1 + max2
            };
        }
        result
    }
}

fn dfs(
    adj_list1: &[Vec<usize>],
    u: usize,
    parent: usize,
    color: &mut [i32],
    even_count: &mut i32,
    odd_count: &mut i32,
) {
    if color[u] == 0 {
        *even_count += 1;
    } else {
        *odd_count += 1;
    }
    for neighbor in &adj_list1[u] {
        if *neighbor != parent {
            color[*neighbor] = if color[u] == 0 { 1 } else { 0 };
            dfs(adj_list1, *neighbor, u, color, even_count, odd_count);
        }
    }
}

fn build_adj_list(edges: &[Vec<i32>], n: usize) -> Vec<Vec<usize>> {
    let mut adj_list = vec![Vec::new(); n];
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj_list[u].push(v);
        adj_list[v].push(u);
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
        let output = vec![8, 7, 7, 8, 8];
        let result = Solution::max_target_nodes(edges1, edges2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let edges1 = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]];
        let edges2 = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let output = vec![3, 6, 6, 6, 6];
        let result = Solution::max_target_nodes(edges1, edges2);
        assert_eq!(result, output);
    }
}
