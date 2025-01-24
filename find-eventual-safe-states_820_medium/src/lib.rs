// https://leetcode.com/problems/find-eventual-safe-states/
#[allow(dead_code)]
struct Solution;

#[derive(Clone, PartialEq)]
enum Status {
    Unvisited,
    // Undecided
    Visited,
    Processed,
}

#[allow(dead_code)]
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut visited = vec![Status::Unvisited; graph.len()];
        let mut result = Vec::new();
        for node in 0..graph.len() {
            if visited[node] == Status::Unvisited {
                if !has_cycle_dfs(&graph, &mut visited, node, &mut result) {}
            }
        }
        result.sort_unstable();
        result
    }
}

fn has_cycle_dfs(
    graph: &[Vec<i32>],
    visited: &mut [Status],
    node: usize,
    result: &mut Vec<i32>,
) -> bool {
    match visited[node] {
        Status::Unvisited => {
            visited[node] = Status::Visited;
            for &neighbor in &graph[node] {
                if has_cycle_dfs(graph, visited, neighbor as usize, result) {
                    return true;
                }
            }
            result.push(node as i32);
            visited[node] = Status::Processed;
            false
        }
        Status::Visited => return true,
        Status::Processed => return false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let graph = vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![],
        ];
        let output = vec![2, 4, 5, 6];
        let result = Solution::eventual_safe_nodes(graph);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
        let output = vec![4];
        let result = Solution::eventual_safe_nodes(graph);
        assert_eq!(result, output);
    }
}
