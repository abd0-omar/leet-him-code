// https://leetcode.com/problems/find-champion-ii/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        // topo-sort without the sort

        // build graph
        let n = n as usize;
        let mut in_degree = vec![0; n];
        for edge in edges {
            let _from = edge[0] as usize;
            let to = edge[1] as usize;
            in_degree[to] += 1;
        }

        let mut champion: Option<i32> = None;
        for (node, &degree) in in_degree.iter().enumerate() {
            if degree == 0 {
                if champion.is_some() {
                    return -1;
                } else {
                    // first time champ
                    champion = Some(node as i32);
                }
            }
        }

        champion.unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let output = 0;
        let result = Solution::find_champion(n, edges);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 4;
        let edges = vec![vec![0, 2], vec![1, 3], vec![1, 2]];
        let output = -1;
        let result = Solution::find_champion(n, edges);
        assert_eq!(result, output);
    }
}
