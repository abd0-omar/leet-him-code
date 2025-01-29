// https://leetcode.com/problems/redundant-connection/
#[allow(dead_code)]
struct Solution;

struct Dsu {
    parent: Vec<i32>,
    component_size: Vec<i32>,
    // no_components, we don't need that field
}

impl Dsu {
    fn new(n: usize) -> Self {
        // add redundant `zeroth` node, to avoid node + 1 stuff
        let parent = (0..=n as i32).collect::<Vec<i32>>();
        let component_size = vec![1; n + 1];
        Self {
            parent,
            component_size,
        }
    }

    // find representative
    fn find(&mut self, x: i32) -> i32 {
        if self.parent[x as usize] == x {
            return x;
        }
        // path compression
        self.parent[x as usize] = self.find(self.parent[x as usize]);
        return self.parent[x as usize];
    }

    fn union(&mut self, x: i32, y: i32) -> bool {
        // they have same representative, we can't merge/union them
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return false;
        }

        if self.component_size[x as usize] >= self.component_size[y as usize] {
            self.parent[y as usize] = x;

            self.component_size[x as usize] += self.component_size[y as usize];
        } else {
            self.parent[x as usize] = y;

            self.component_size[y as usize] += self.component_size[x as usize];
        }

        true
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut dsu = Dsu::new(n);
        for edge in edges {
            // it's undirected, but will call it from and to anyway
            let from = edge[0];
            let to = edge[1];
            if !dsu.union(from, to) {
                return edge;
            }
        }
        unreachable!("the question said that there is always an additional edge");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        let output = vec![2, 3];
        let result = Solution::find_redundant_connection(edges);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]];
        let output = vec![1, 4];
        let result = Solution::find_redundant_connection(edges);
        assert_eq!(result, output);
    }
}
