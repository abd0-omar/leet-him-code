// https://leetcode.com/problems/minimum-cost-walk-in-weighted-graph/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let mut dsu = DSU::new(n as usize);
        for edge in edges.iter() {
            dsu.union(edge[0], edge[1]);
        }

        for edge in edges {
            let root = dsu.find(edge[0]);
            dsu.bitwise_and_sum[root as usize] =
                if let Some(sum) = dsu.bitwise_and_sum[root as usize] {
                    Some(sum & edge[2])
                } else {
                    Some(edge[2])
                };
        }

        let mut result = Vec::with_capacity(query.len());
        for q in query.into_iter() {
            // if diff representative (i.e seperate forests) return -1
            // I found a good use for "i.e", it doesn't add much but I want to
            // use it
            if dsu.find(q[0]) != dsu.find(q[1]) {
                result.push(-1);
            } else {
                let common_representative = dsu.find(q[0]) as usize;
                result.push(dsu.bitwise_and_sum[common_representative].unwrap());
            }
        }
        result
    }
}

struct DSU {
    // representative
    parent: Vec<i32>,
    rank: Vec<i32>,
    bitwise_and_sum: Vec<Option<i32>>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let parent = (0..n).into_iter().map(|x| x as i32).collect::<Vec<i32>>();
        Self {
            parent,
            rank: vec![0; n],
            bitwise_and_sum: vec![None; n],
        }
    }

    fn find(&mut self, x: i32) -> i32 {
        let x_usize = x as usize;
        if self.parent[x_usize] == x {
            return x;
        }
        self.parent[x_usize] = self.find(self.parent[x_usize]);
        return self.parent[x_usize];
    }

    fn union(&mut self, x: i32, y: i32) {
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
            return;
        }

        let x_usize = x as usize;
        let y_usize = y as usize;

        if self.rank[x_usize] >= self.rank[y_usize] {
            self.parent[y_usize] = x;
        } else if self.rank[x_usize] < self.rank[y_usize] {
            self.parent[x_usize] = y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 5;
        let edges = vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]];
        let query = vec![vec![0, 3], vec![3, 4]];
        let output = vec![1, -1];
        let result = Solution::minimum_cost(n, edges, query);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 3;
        let edges = vec![vec![0, 2, 7], vec![0, 1, 15], vec![1, 2, 6], vec![1, 2, 1]];
        let query = vec![vec![1, 2]];
        let output = vec![0];
        let result = Solution::minimum_cost(n, edges, query);
        assert_eq!(result, output);
    }
}
