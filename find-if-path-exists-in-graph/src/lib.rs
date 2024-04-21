//union find
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    forests: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let rank = vec![1; n];

        Self {
            parent,
            rank,
            forests: n,
        }
    }

    fn find_parent_and_link(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        self.parent[x] = self.find_parent_and_link(self.parent[x]);
        self.parent[x]
    }

    fn union_find_and_link(&mut self, x: usize, y: usize) -> bool {
        let x = self.find_parent_and_link(x);
        let y = self.find_parent_and_link(y);

        if x == y {
            // already same component
            return false;
        }

        //link
        if self.rank[x] > self.rank[y] {
            // smaller rank wins
            self.parent[x] = y;
        } else if self.rank[y] > self.rank[x] {
            self.parent[y] = x;
        } else {
            self.parent[y] = x;
            self.rank[y] += 1;
        }

        self.forests -= 1;

        true
    }
}

pub fn valid_path_union_find(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut union_find = UnionFind::new(n as usize);
    for edge in edges {
        union_find.union_find_and_link(edge[0] as usize, edge[1] as usize);
    }

    if union_find.find_parent_and_link(source as usize)
        == union_find.find_parent_and_link(destination as usize)
    {
        true
    } else {
        false
    }
}

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut graph = vec![vec![]; n as usize];
    let mut visited = vec![false; n as usize];
    for edge in edges {
        let from = edge[0];
        let to = edge[1];
        graph[from as usize].push(to);
        graph[to as usize].push(from);
    }
    dbg!(&graph);

    dfs(&graph, source, destination, &mut visited)
}

pub fn dfs(graph: &Vec<Vec<i32>>, source: i32, destination: i32, visited: &mut Vec<bool>) -> bool {
    if source == destination {
        return true;
    }

    visited[source as usize] = true;

    let mut result = false;

    for neighbor in &graph[source as usize] {
        if visited[*neighbor as usize] {
            continue;
        }
        result |= dfs(graph, *neighbor, destination, visited);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let source = 0;
        let destination = 2;
        let output = true;
        // Explanation: There are two paths from vertex 0 to vertex 2:
        // - 0 → 1 → 2
        // - 0 → 2
        let result = valid_path(n, edges, source, destination);
        assert_eq!(result, output);
    }
}
