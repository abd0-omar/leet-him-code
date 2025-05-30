// https://leetcode.com/problems/find-closest-node-to-given-two-nodes/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let mut dist1 = vec![-1; edges.len()];
        let mut dist2 = vec![-1; edges.len()];
        dfs_get_dist(&edges, node1 as usize, &mut dist1, 0);
        dfs_get_dist(&edges, node2 as usize, &mut dist2, 0);
        let mut min_dist = i32::MAX;
        let mut result = -1;
        for node in 0..edges.len() {
            if dist1[node] != -1 && dist2[node] != -1 {
                let max_dist = dist1[node].max(dist2[node]);
                if max_dist < min_dist {
                    min_dist = max_dist;
                    result = node as i32;
                }
            }
        }
        result
    }
}

fn dfs_get_dist(edges: &[i32], node: usize, dist: &mut [i32], cur_dist: i32) {
    if dist[node] != -1 {
        return;
    }
    dist[node] = cur_dist;
    if edges[node] != -1 {
        dfs_get_dist(edges, edges[node] as usize, dist, cur_dist + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let edges = vec![2, 2, 3, -1];
        let node1 = 0;
        let node2 = 1;
        let output = 2;
        let result = Solution::closest_meeting_node(edges, node1, node2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let edges = vec![1, 2, -1];
        let node1 = 0;
        let node2 = 2;
        let output = 2;
        let result = Solution::closest_meeting_node(edges, node1, node2);
        assert_eq!(result, output);
    }
}
