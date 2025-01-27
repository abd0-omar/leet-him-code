// https://leetcode.com/problems/course-schedule-iv/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let num_courses = num_courses as usize;
        let mut adj_list = vec![vec![]; num_courses];

        // make a reverse graph
        // to show the dependant
        // and when traversing in DFS
        // cache/save the results in HashMap or something
        //
        // after solving, you don't need to build the graph in reverse

        let mut in_degree = vec![0; num_courses];
        let mut max_node = 0;
        for pre in prerequisites {
            let from = pre[0] as usize;
            let to = pre[1] as usize;
            in_degree[to] += 1;
            if in_degree[to] > in_degree[max_node] {
                max_node = to;
            }
            adj_list[to].push(from);
        }

        // is_reachable[0][1]
        let mut is_reachable = vec![vec![false; num_courses]; num_courses];
        for i in 0..num_courses {
            is_reachable[i][i] = true;
        }

        // dbg!(&adj_list);

        for node in 0..num_courses {
            dfs(&adj_list, node, &mut is_reachable, node);
        }

        // dbg!(&is_reachable);

        let mut result = Vec::with_capacity(queries.len());
        for query in queries {
            let from = query[0] as usize;
            let to = query[1] as usize;
            result.push(is_reachable[to][from]);
        }

        result
    }
}

fn dfs(
    adj_list: &[Vec<usize>],
    node: usize,
    is_reachable: &mut [Vec<bool>],
    master_node: usize,
) -> () {
    for &neighbor in &adj_list[node] {
        // imp condition
        // otherwise it got me tle
        if !is_reachable[master_node][neighbor] {
            is_reachable[master_node][neighbor] = true;
            dfs(adj_list, neighbor, is_reachable, master_node)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let queries = vec![vec![0, 1], vec![1, 0]];
        let output = vec![false, true];
        let result = Solution::check_if_prerequisite(num_courses, prerequisites, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let num_courses = 2;
        let prerequisites = vec![];
        let queries = vec![vec![1, 0], vec![0, 1]];
        let output = vec![false, false];
        let result = Solution::check_if_prerequisite(num_courses, prerequisites, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let num_courses = 3;
        let prerequisites = vec![vec![1, 2], vec![1, 0], vec![2, 0]];
        let queries = vec![vec![1, 0], vec![1, 2]];
        let output = vec![true, true];
        let result = Solution::check_if_prerequisite(num_courses, prerequisites, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let num_courses = 5;
        let prerequisites = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        let queries = vec![vec![0, 4], vec![4, 0], vec![1, 3], vec![3, 0]];
        let output = vec![true, false, true, false];
        let result = Solution::check_if_prerequisite(num_courses, prerequisites, queries);
        assert_eq!(result, output);
    }
}
