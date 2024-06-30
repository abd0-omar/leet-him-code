pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut dsu_alice = DSU::new(n);
    let mut dsu_bob = DSU::new(n);
    let mut count_alice = 0;
    let mut count_bob = 0;
    let mut count_both = 0;

    for edge in &edges {
        let edge_type = edge[0];
        let from = edge[1] as usize;
        let to = edge[2] as usize;

        if edge_type == 3 {
            if dsu_alice.union_find(from, to) && dsu_bob.union_find(from, to) {
                count_both += 1;
            }
        }
    }

    for edge in &edges {
        let edge_type = edge[0];
        let from = edge[1] as usize;
        let to = edge[2] as usize;

        if edge_type == 1 {
            if dsu_alice.union_find(from, to) {
                count_alice += 1;
            }
        }
    }

    for edge in &edges {
        let edge_type = edge[0];
        let from = edge[1] as usize;
        let to = edge[2] as usize;

        if edge_type == 2 {
            if dsu_bob.union_find(from, to) {
                count_bob += 1;
            }
        }
    }

    let total_edges = edges.len() as i32;
    if dsu_alice.forsets == 1 && dsu_bob.forsets == 1 {
        total_edges - count_both - count_alice - count_bob
    } else {
        -1
    }
}

// my long attempt
pub fn _max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize + 1;
    // count if there could be a cycle
    // count two edges of type 3 and other type
    let mut dsu_alice = DSU::new(n);
    let mut dsu_bob = DSU::new(n);
    let mut count_alice1 = 0;
    let mut count_alice2 = 0;
    let mut count_bob1 = 0;
    let mut count_bob2 = 0;

    // from index 0 to tuple or struct type
    use std::collections::HashSet;
    let mut adj_list = vec![vec![]; n];
    let mut adj_list_unidirectional = vec![vec![]; n];
    for edge in edges {
        let edge_type = edge[0];
        let from = edge[1] as usize;
        let to = edge[2] as usize;

        if edge_type != 2 {
            if dsu_alice.union_find(from, to) {
                count_alice1 += 1;
            } else {
                // already connected counter
                count_alice2 += 1;
            }
        }

        if edge_type != 1 {
            if dsu_bob.union_find(from, to) {
                count_bob1 += 1;
            } else {
                count_bob2 += 1;
            }
        }

        println!("{:?}", edge);
        adj_list[from].push((to, edge_type));

        adj_list_unidirectional[from].push((to, edge_type));
        adj_list_unidirectional[to].push((from, edge_type));
    }

    // handle doulbe edges
    for node in 1..n {
        let mut hs_alice = HashSet::new();
        let mut hs_bob = HashSet::new();
        for &(neighbor, neighbor_type) in adj_list[node].iter() {
            //
            if neighbor_type != 2 {
                if !hs_alice.insert(neighbor) {
                    count_alice2 += 1;
                }
            }

            if neighbor_type != 1 {
                if !hs_bob.insert(neighbor) {
                    count_bob2 += 1;
                }
            }
        }
    }

    // check reachable for it_works2() test case
    // by using dfs
    // alice
    // let mut all_visited_alice: HashMap<usize, Vec<bool>> = HashMap::new();
    // let mut visited = vec![vec![false; n]; n];
    // for node in 1..n {
    //     dfs(&adj_list, &mut visited, node, 2, node);

    //     // for i in 1..n {
    //     //     if !visited[i] {
    //     //         if let Some(v) = all_visited_alice.get(&i) {
    //     //             if !v[node] {
    //     //                 return -1;
    //     //             }
    //     //         }
    //     //     }
    //     // }

    //     // return -1;
    //     // }

    //     // all_visited_alice.insert(node, visited);
    // }
    // let mut dont_return_neg_one = false;
    // dbg!(&visited);
    // 'outer: for v in visited.iter() {
    //     let mut all_true = true;
    //     for i in 1..n {
    //         if !v[i] {
    //             all_true = false;
    //             continue 'outer;
    //         }
    //     }
    //     if all_true {
    //         dont_return_neg_one = true;
    //         break;
    //     }

    //     // if v.iter().skip(1).any(|&f| f == false) {
    //     //     // dbg!(&node);
    //     //     dbg!(&visited);
    //     //     return -1;
    //     // }
    // }
    // if !dont_return_neg_one {
    //     return -1;
    // }
    // // dbg!(&all_visited_alice);

    // // 'outer: for visted in all_visited_alice.values() {
    // //     for i in 1..n {
    // //         if visted[i] {
    // //             continue 'outer;
    // //         }
    // //     }
    // //     return -1;
    // // }

    // // bob
    // // let mut all_visited_bob: HashMap<usize, Vec<bool>> = HashMap::new();
    // let mut visited = vec![vec![false; n]; n];
    // for node in 1..n {
    //     dfs(&adj_list, &mut visited, node, 1, node);

    //     // if visited.iter().skip(1).any(|&f| f == false) {
    //     //     return -1;
    //     // }
    //     // for i in 1..n {
    //     //     if !visited[i] {
    //     //         if let Some(v) = all_visited_bob.get(&i) {
    //     //             if !v[node] {
    //     //                 return -1;
    //     //             }
    //     //         }
    //     //     }
    //     // }

    //     // all_visited_bob.insert(node, visited);
    // }
    // let mut dont_return_neg_one = false;
    // dbg!(&visited);
    // 'outer: for v in visited.iter() {
    //     let mut all_true = true;
    //     for i in 1..n {
    //         if !v[i] {
    //             all_true = false;
    //             continue 'outer;
    //         }
    //     }
    //     if all_true {
    //         dont_return_neg_one = true;
    //         break;
    //     }

    //     // if v.iter().skip(1).any(|&f| f == false) {
    //     //     // dbg!(&node);
    //     //     dbg!(&visited);
    //     //     return -1;
    //     // }
    // }
    // if !dont_return_neg_one {
    //     return -1;
    // }
    // // dbg!(&all_visited_bob);

    // // 'outer: for visted in all_visited_bob.values() {
    // //     for i in 1..n {
    // //         if visted[i] {
    // //             continue 'outer;
    // //         }
    // //     }
    // //     return -1;
    // // }

    // why 2?
    // because we made dsu with n + 1
    // so zero is a forest on it's own
    // and the rest are in one forest

    dbg!(&adj_list);
    dbg!(&count_alice1);
    dbg!(&count_alice2);
    dbg!(&dsu_alice.forsets);
    dbg!(&count_bob1);
    dbg!(&count_bob2);
    dbg!(&dsu_bob.forsets);

    if n == 2 {
        if dsu_alice.forsets != 1 && dsu_bob.forsets != 1 {
            return -1;
        } else {
            if dsu_alice.forsets != 2 && dsu_bob.forsets != 2 {
                return -1;
            }
        }
    }

    println!("{:?}", adj_list);
    dbg!(adj_list);
    dbg!(count_alice1);
    dbg!(count_alice2);
    dbg!(dsu_alice.forsets);
    dbg!(count_bob1);
    dbg!(count_bob2);
    dbg!(dsu_bob.forsets);

    count_alice2 + count_bob2
}

fn dfs(
    adj_list: &Vec<Vec<(usize, i32)>>,
    visited: &mut Vec<Vec<bool>>,
    node: usize,
    not_allowed_type: i32,
    coming_from: usize,
) {
    visited[coming_from][node] = true;
    for &(neighbor, neighbor_type) in adj_list[node].iter() {
        if neighbor_type != not_allowed_type {
            dfs(adj_list, visited, neighbor, not_allowed_type, coming_from);
        }
    }
}

// union find template
// disjoint-set union
#[derive(Debug)]
struct DSU {
    parent: Vec<usize>,
    rank: Vec<usize>,
    forsets: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        let parent = (0..=n).collect();
        let rank = vec![1; n + 1];
        Self {
            parent,
            rank,
            forsets: n,
        }
    }

    fn find_parent_and_link(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find_parent_and_link(self.parent[x]);
        }
        self.parent[x]
    }

    fn union_find(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find_parent_and_link(x);
        let root_y = self.find_parent_and_link(y);
        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }

        self.forsets -= 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 4;
        let edges = vec![
            vec![3, 1, 2],
            vec![3, 2, 3],
            vec![1, 1, 3],
            vec![1, 2, 4],
            vec![1, 1, 2],
            vec![2, 3, 4],
        ];
        let output = 2;
        let result = max_num_edges_to_remove(n, edges);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 4;
        let edges = vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]];
        let output = 0;
        let result = max_num_edges_to_remove(n, edges);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let n = 4;
        let edges = vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]];
        let output = -1;
        let result = max_num_edges_to_remove(n, edges);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let n = 4;
        let edges = vec![vec![3, 1, 2], vec![3, 3, 4], vec![1, 1, 3], vec![2, 2, 4]];
        let output = 0;
        let result = max_num_edges_to_remove(n, edges);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works4() {
        let n = 2;
        let edges = vec![vec![1, 1, 2], vec![2, 1, 2], vec![3, 1, 2]];
        let output = 2;
        let result = max_num_edges_to_remove(n, edges);
        assert_eq!(result, output);
    }
}
