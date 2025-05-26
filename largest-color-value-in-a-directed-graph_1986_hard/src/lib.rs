// https://leetcode.com/problems/largest-color-value-in-a-directed-graph/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let colors = colors.chars().collect::<Vec<char>>();
        let n = colors.len();
        let mut adj_list = vec![vec![]; n];
        let mut in_degree = vec![0; n];
        for edge in edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            in_degree[to] += 1;
            adj_list[from].push(to);
        }

        let mut color_count = vec![vec![0; 26]; n];
        let mut ready = std::collections::VecDeque::new();
        for i in 0..n {
            if in_degree[i] == 0 {
                ready.push_back(i);
            }
        }
        let mut visited = 0;
        let mut result = 0;
        while let Some(node) = ready.pop_front() {
            visited += 1;
            let color_idx = (colors[node] as u8 - b'a') as usize;
            color_count[node][color_idx] += 1;
            result = result.max(color_count[node][color_idx]);
            for &neighbor in &adj_list[node] {
                for c in 0..26 {
                    color_count[neighbor][c] = color_count[neighbor][c].max(color_count[node][c]);
                }
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 0 {
                    ready.push_back(neighbor);
                }
            }
        }
        if visited != n { -1 } else { result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let colors = "abaca".to_string();
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]];
        let output = 3;
        let result = Solution::largest_path_value(colors, edges);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let colors = "a".to_string();
        let edges = vec![vec![0, 0]];
        let output = -1;
        let result = Solution::largest_path_value(colors, edges);
        assert_eq!(result, output);
    }
}
