// https://leetcode.com/problems/most-profitable-path-in-a-tree/
#[allow(dead_code)]
struct Solution;

use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let n = amount.len();
        let mut adj_list = vec![Vec::new(); n];

        for e in edges {
            let (from, to) = (e[0] as usize, e[1] as usize);
            adj_list[from].push(to as i32);
            adj_list[to].push(from as i32);
        }

        // Bob's DFS
        // node -> time
        let mut bob_times: HashMap<i32, i32> = HashMap::new();
        dfs(bob, -1, 0, &mut bob_times, &adj_list);

        // Alice's BFS
        // (node, time, parent, total_profit)
        let mut q = VecDeque::from([(0, 0, -1, amount[0])]);
        let mut result = i32::MIN;

        while let Some((node, time, parent, profit)) = q.pop_front() {
            // If it's a leaf node (except the root)
            if node != 0 && adj_list[node as usize].len() == 1 {
                result = result.max(profit);
            }

            for &nei in &adj_list[node as usize] {
                if nei == parent {
                    continue;
                }
                let mut nei_profit = amount[nei as usize];
                let nei_time = time + 1;

                if let Some(&bob_time_on_this_node) = bob_times.get(&nei) {
                    if nei_time > bob_time_on_this_node {
                        nei_profit = 0;
                    } else if nei_time == bob_time_on_this_node {
                        nei_profit /= 2;
                    }
                }

                q.push_back((nei, nei_time, node, profit + nei_profit));
            }
        }

        result
    }
}

fn dfs(
    src: i32,
    prev: i32,
    time: i32,
    bob_times: &mut HashMap<i32, i32>,
    adj_list: &Vec<Vec<i32>>,
) -> bool {
    if src == 0 {
        bob_times.insert(src, time);
        return true;
    }

    for &nei in &adj_list[src as usize] {
        if nei == prev {
            continue;
        }
        if dfs(nei, src, time + 1, bob_times, adj_list) {
            bob_times.insert(src, time); // Fix: store Bob's time when backtracking
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]];
        let bob = 3;
        let amount = vec![-2, 4, 2, -4, 6];
        let output = 6;
        let result = Solution::most_profitable_path(edges, bob, amount);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let edges = vec![vec![0, 1]];
        let bob = 1;
        let amount = vec![-7280, 2350];
        let output = -7280;
        let result = Solution::most_profitable_path(edges, bob, amount);
        assert_eq!(result, output);
    }
}
