// https://leetcode.com/problems/shortest-distance-after-road-addition-queries-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // this one will mark my 500 leetcode problem
        // 500 problems of unemployment
        // let's hope we get a job before reaching 1000
        // or have enough coins for the leetcode cap
        // and I can't say I would've done that without Dr.Mostafa Saad
        // and it's a fun one too
        // bfs
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for i in 1..n {
            let from = i - 1;
            let to = i;
            graph[from].push(to);
        }
        // dbg!(&graph);
        let mut result = Vec::with_capacity(queries.len());
        for q in queries {
            let from = q[0] as usize;
            let to = q[1] as usize;
            graph[from].push(to);

            let mut visited = vec![false; n];
            result.push(bfs(&graph, n - 1, &mut visited));
        }
        result
    }
}

fn bfs(graph: &[Vec<usize>], target: usize, visited: &mut [bool]) -> i32 {
    // bfs might not be the most efficient here,
    // but it should work within the problem constraints.
    // get it working first, and optimize later if necessary.

    let mut q = std::collections::VecDeque::new();
    q.push_back(0);
    let mut lvl = 0;

    while !q.is_empty() {
        let size = q.len();
        for _ in 0..size {
            let cur_node = q.pop_front().unwrap();
            visited[cur_node] = true;
            if cur_node == target {
                return lvl;
            }

            for &neighbor in &graph[cur_node] {
                if !visited[neighbor] {
                    q.push_back(neighbor);
                }
            }
        }
        lvl += 1;
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 5;
        let queries = vec![vec![2, 4], vec![0, 2], vec![0, 4]];
        let output = vec![3, 2, 1];
        let result = Solution::shortest_distance_after_queries(n, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 4;
        let queries = vec![vec![0, 3], vec![0, 2]];
        let output = vec![1, 1];
        let result = Solution::shortest_distance_after_queries(n, queries);
        assert_eq!(result, output);
    }
}
