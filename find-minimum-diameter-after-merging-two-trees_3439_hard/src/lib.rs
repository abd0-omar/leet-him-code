// https://leetcode.com/problems/find-minimum-diameter-after-merging-two-trees/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        // find root node of each tree using topo-sort
        let n1 = edges1.len() + 1;
        let n2 = edges2.len() + 1;
        let mut in_degree1 = vec![0; n1];
        let mut in_degree2 = vec![0; n2];
        let mut adj_list1 = vec![vec![]; n1];
        let mut adj_list2 = vec![vec![]; n2];
        build_adj_list(edges1, &mut in_degree1, &mut adj_list1);
        build_adj_list(edges2, &mut in_degree2, &mut adj_list2);

        // get diameters for both trees
        let d1 = get_diameter(&mut in_degree1, &adj_list1, n1);
        let d2 = get_diameter(&mut in_degree2, &adj_list2, n2);

        // ceil for odd numbers
        let all_diameter = ((d1 as f64 / 2.0).ceil() + (d2 as f64 / 2.0).ceil()) as i32 + 1;
        all_diameter.max(d1).max(d2)
    }
}

fn get_diameter(in_degree: &mut Vec<i32>, adj_list: &[Vec<usize>], n: usize) -> i32 {
    let mut q = std::collections::VecDeque::new();
    // get the nodes with in_degree 1
    for i in 0..n {
        if in_degree[i] == 1 {
            q.push_back(i);
        }
    }
    let mut nodes_count = n;
    let mut lvl = 0;
    while nodes_count > 2 && !q.is_empty() {
        let size = q.len();
        nodes_count -= size;
        lvl += 1;
        for _ in 0..size {
            let cur_node = q.pop_front().unwrap();

            for &neighbor in adj_list[cur_node].iter() {
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 1 {
                    q.push_back(neighbor);
                }
            }
        }
    }
    if nodes_count == 2 {
        lvl * 2 + 1
    } else {
        lvl * 2
    }
}

fn build_adj_list(edges: Vec<Vec<i32>>, in_degree: &mut [i32], adj_list: &mut Vec<Vec<usize>>) {
    for edge in edges {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        in_degree[from] += 1;
        in_degree[to] += 1;
        adj_list[from].push(to);
        adj_list[to].push(from);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let edges1 = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let edges2 = vec![vec![0, 1]];
        let output = 3;
        let result = Solution::minimum_diameter_after_merge(edges1, edges2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let edges1 = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![2, 4],
            vec![2, 5],
            vec![3, 6],
            vec![2, 7],
        ];
        let edges2 = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![2, 4],
            vec![2, 5],
            vec![3, 6],
            vec![2, 7],
        ];
        let output = 5;
        let result = Solution::minimum_diameter_after_merge(edges1, edges2);
        assert_eq!(result, output);
    }
}
