// https://leetcode.com/problems/valid-arrangement-of-pairs/
#[allow(dead_code)]
struct Solution;

use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph: HashMap<i32, VecDeque<i32>> = HashMap::new();
        let mut out_degree: HashMap<i32, i32> = HashMap::new();
        let mut in_degree: HashMap<i32, i32> = HashMap::new();

        for pair in &pairs {
            let start = pair[0];
            let end = pair[1];
            graph.entry(start).or_default().push_back(end);
            *out_degree.entry(start).or_default() += 1;
            *in_degree.entry(end).or_default() += 1;
        }
        // start: out > in
        // eg. 1 -> [3, 5]

        // regular: in = ou

        // end: out < in
        // eg. 2 -> [4],  6 -> [4], and 4 has 1 in for example
        let mut start_node = pairs[0][0];
        for (&node, &out) in &out_degree {
            let in_count = in_degree.get(&node).unwrap_or(&0);
            if out == in_count + 1 {
                start_node = node;
                break;
            }
        }

        let mut result = Vec::new();

        dfs(start_node, &mut graph, &mut result);

        result.reverse();
        // dbg!(&result);

        result.windows(2).map(|w| vec![w[0], w[1]]).collect()
    }
}

fn dfs(node: i32, graph: &mut HashMap<i32, VecDeque<i32>>, result: &mut Vec<i32>) {
    while let Some(next_node) = graph.entry(node).or_default().pop_front() {
        dfs(next_node, graph, result);
    }
    result.push(node);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let pairs = vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]];
        let output = vec![vec![11, 9], vec![9, 4], vec![4, 5], vec![5, 1]];
        let result = Solution::valid_arrangement(pairs);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let pairs = vec![vec![1, 3], vec![3, 2], vec![2, 1]];
        let output = vec![vec![1, 3], vec![3, 2], vec![2, 1]];
        let result = Solution::valid_arrangement(pairs);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let pairs = vec![vec![1, 2], vec![1, 3], vec![2, 1]];
        let output = vec![vec![1, 2], vec![2, 1], vec![1, 3]];
        let result = Solution::valid_arrangement(pairs);
        assert_eq!(result, output);
    }
}
