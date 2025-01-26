// https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting/
#[allow(dead_code)]
struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};
#[allow(dead_code)]
impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let n = favorite.len();
        let mut longest_cycle = 0;
        let mut visit = vec![false; n];
        let mut length_2_cycles = Vec::new();

        // find longest cycle
        for i in 0..n {
            if visit[i] {
                continue;
            }

            let (mut start, mut cur) = (i, i);
            let mut cur_set = HashSet::new();

            while !visit[cur] {
                visit[cur] = true;
                cur_set.insert(cur);
                cur = favorite[cur] as usize;
            }
            // new cycle
            if cur_set.contains(&cur) {
                let mut length = cur_set.len();
                while start != cur {
                    length -= 1;
                    start = favorite[start] as usize;
                }
                longest_cycle = longest_cycle.max(length);
                if length == 2 {
                    length_2_cycles.push((cur, favorite[cur]));
                }
            }
        }
        // find sum of longest non closed circles
        let mut inverted: HashMap<i32, Vec<_>> = HashMap::new();
        for (dst, &src) in favorite.iter().enumerate() {
            inverted.entry(src).or_insert(Vec::new()).push(dst);
        }

        fn bfs(src: usize, inverted: &mut HashMap<i32, Vec<usize>>, parent: usize) -> i32 {
            let mut q = VecDeque::new();
            q.push_back((src, 0));
            let mut max_length = 0;
            while let Some((node, length)) = q.pop_front() {
                if node == parent {
                    continue;
                }
                max_length = max_length.max(length);
                if let Some(neighbors) = inverted.get(&(node as i32)) {
                    for &neighbor in neighbors {
                        q.push_back((neighbor, length + 1));
                    }
                }
            }
            max_length
        }

        let mut chain_sum = 0;
        for (n1, n2) in length_2_cycles {
            chain_sum +=
                bfs(n1, &mut inverted, n2 as usize) + bfs(n2 as usize, &mut inverted, n1) + 2;
        }

        chain_sum.max(longest_cycle as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let favorite = vec![2, 2, 1, 2];
        let output = 3;
        let result = Solution::maximum_invitations(favorite);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let favorite = vec![1, 2, 0];
        let output = 3;
        let result = Solution::maximum_invitations(favorite);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let favorite = vec![3, 0, 1, 4, 1];
        let output = 4;
        let result = Solution::maximum_invitations(favorite);
        assert_eq!(result, output);
    }
}
