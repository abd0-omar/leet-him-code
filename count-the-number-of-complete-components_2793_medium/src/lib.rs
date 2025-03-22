// https://leetcode.com/problems/count-the-number-of-complete-components/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut dsu = DSU::new(n);

        for edge in edges.iter() {
            dsu.union_find(edge[0] as usize, edge[1] as usize);
        }
        // dbg!(&dsu.count_edges);
        // formula => (n (n - 1) ) / 2 == edges
        let mut edge_count = vec![0; n];
        for edge in edges.iter() {
            let root = dsu.find(edge[0] as usize);
            edge_count[root] += 1;
        }

        // dbg!(&edge_count);

        let mut result = 0;
        for node in 0..n {
            if dsu.find(node) == node {
                let nodes_count = dsu.size[node];
                let expected_edges = (nodes_count * (nodes_count - 1)) / 2;
                // dbg!(&node);
                // dbg!(&expected_edges);
                // dbg!(&nodes_count);
                if expected_edges == edge_count[node] {
                    result += 1;
                }
            }
        }
        result
    }
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        self.parent[x] = self.find(self.parent[x]);
        return self.parent[x];
    }

    fn union_find(&mut self, x: usize, y: usize) {
        let (x, y) = (self.find(x), self.find(y));

        if x == y {
            return;
        }

        if self.size[x] >= self.size[y] {
            self.parent[y] = x;
            self.size[x] += self.size[y];
        } else {
            self.parent[x] = y;
            self.size[y] += self.size[x];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]];
        let output = 3;
        let result = Solution::count_complete_components(n, edges);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]];
        let output = 1;
        let result = Solution::count_complete_components(n, edges);
        assert_eq!(result, output);
    }
}
