// https://leetcode.com/problems/divide-nodes-into-the-maximum-number-of-groups/

#[allow(dead_code)]
struct Solution;

use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj_list = vec![vec![]; n];

        for edge in edges {
            let from = edge[0] as usize - 1;
            let to = edge[1] as usize - 1;
            if from >= n || to >= n {
                continue;
            }
            adj_list[from].push(to);
            adj_list[to].push(from);
        }

        let mut visit = HashSet::new();
        let mut res = 0;

        for i in 0..n {
            if visit.contains(&i) {
                continue;
            }

            let component = get_connected_component(&adj_list, i, &mut visit);
            let mut max_count = 0;

            for &src in &component {
                let length = longest_path(&adj_list, src, component.len());
                if length == -1 {
                    return -1;
                }
                max_count = max_count.max(length);
            }
            res += max_count;
        }

        res
    }
}

fn longest_path(adj_list: &[Vec<usize>], src: usize, size: usize) -> i32 {
    let mut q = VecDeque::new();
    let mut dist = vec![-1; adj_list.len()];

    q.push_back((src, 1));
    dist[src] = 1;

    while let Some((node, length)) = q.pop_front() {
        for &neighbor in &adj_list[node] {
            if dist[neighbor] == -1 {
                dist[neighbor] = length + 1;
                q.push_back((neighbor, length + 1));
            } else if dist[neighbor] == length {
                return -1;
            }
        }
    }

    *dist.iter().max().unwrap()
}

fn get_connected_component(
    adj_list: &[Vec<usize>],
    src: usize,
    visit: &mut HashSet<usize>,
) -> Vec<usize> {
    let mut q = VecDeque::new();
    let mut component = Vec::new();

    q.push_back(src);
    visit.insert(src);

    while let Some(node) = q.pop_front() {
        component.push(node);
        for &neighbor in &adj_list[node] {
            if !visit.contains(&neighbor) {
                visit.insert(neighbor);
                q.push_back(neighbor);
            }
        }
    }

    component
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 6;
        let edges = vec![
            vec![1, 2],
            vec![1, 4],
            vec![1, 5],
            vec![2, 6],
            vec![2, 3],
            vec![4, 6],
        ];
        let output = 4;
        let result = Solution::magnificent_sets(n, edges);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 3;
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 1]];
        let output = -1;
        let result = Solution::magnificent_sets(n, edges);
        assert_eq!(result, output);
    }
}
