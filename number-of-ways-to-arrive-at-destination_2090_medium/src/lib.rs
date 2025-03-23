// https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj_list = vec![vec![]; n];
        let modulo = 1_000_000_007;

        // Construct adjacency list with (neighbor, weight)
        for road in &roads {
            let from = road[0] as usize;
            let to = road[1] as usize;
            let weight = road[2] as i64;
            adj_list[from].push((to, weight));
            adj_list[to].push((from, weight));
        }

        // dijkstra
        let mut distance = vec![None; n];
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        // (weight, node)
        let mut min_heap = BinaryHeap::from([(Reverse(0), 0)]);

        // new to dijkstra template (how many diff paths I can reach this node with the minimum cost)
        let mut path_count = vec![0; n];
        path_count[0] = 1;

        while let Some((Reverse(cur_weight), cur_node)) = min_heap.pop() {
            // If we've already found a shorter distance before, skip this node.
            // not necessary but get any optimization you can get
            if let Some(d) = distance[cur_node] {
                if cur_weight > d {
                    continue;
                }
            }

            for &(nei_node, nei_weight) in &adj_list[cur_node] {
                // allegedly new_distance
                // wannabe new_distance
                // dreamer new_distance
                let candidate_new_distance = cur_weight + nei_weight;

                if let Some(nei_min_dist_so_far) = distance[nei_node] {
                    if nei_min_dist_so_far > candidate_new_distance {
                        distance[nei_node] = Some(candidate_new_distance);
                        // new to dijkstra template
                        // found a smaller min_dist to node, so remove old path_count
                        // and update it with the new one
                        path_count[nei_node] = path_count[cur_node];
                        min_heap.push((Reverse(candidate_new_distance), nei_node));
                    } else if nei_min_dist_so_far == candidate_new_distance {
                        // new to dijkstra template
                        path_count[nei_node] =
                            (path_count[cur_node] + path_count[nei_node]) % modulo;
                    }
                } else {
                    distance[nei_node] = Some(candidate_new_distance);
                    // found a new node, initialize its path count
                    path_count[nei_node] = path_count[cur_node];
                    min_heap.push((Reverse(candidate_new_distance), nei_node));
                }
            }
        }
        path_count[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 7;
        let roads = vec![
            vec![0, 6, 7],
            vec![0, 1, 2],
            vec![1, 2, 3],
            vec![1, 3, 3],
            vec![6, 3, 3],
            vec![3, 5, 1],
            vec![6, 5, 1],
            vec![2, 5, 1],
            vec![0, 4, 5],
            vec![4, 6, 2],
        ];
        let output = 4;
        let result = Solution::count_paths(n, roads);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 2;
        let roads = vec![vec![1, 0, 10]];
        let output = 1;
        let result = Solution::count_paths(n, roads);
        assert_eq!(result, output);
    }
}
