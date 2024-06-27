pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    // let mut hs = std::collections::HashSet::new();
    // for e in edges {
    //     let from = e[0];
    //     let to = e[1];
    //     if !hs.insert(from) {
    //         return from;
    //     }
    //     if !hs.insert(to) {
    //         return to;
    //     }
    // }
    // -1
    // what if we have more than one center?
    // topological sort
    let n = edges.len() + 1;
    let mut adj_list = vec![vec![]; n + 1];
    let mut in_degree = vec![0; n + 1];
    for e in edges {
        let from = e[0];
        let to = e[1];
        // undirected graph
        in_degree[to as usize] += 1;
        in_degree[from as usize] += 1;
        // adj_list to be used when decrease in_degree of neighbors
        adj_list[from as usize].push(to);
        adj_list[to as usize].push(from);
    }
    let mut q = std::collections::VecDeque::new();
    // push lowest in_degree first, any node with in_degree "one" push it
    // maybe the & will get an error
    // I could do iter_mut here
    for (node, degree) in in_degree.iter_mut().enumerate() {
        if *degree == 1 {
            q.push_back(node);
            // in_degree[node] -= 1;
            *degree -= 1;
        }
    }

    let mut ans = 0;
    // if there was more than one center, uncomment last_layer
    // let mut last_layer = Vec::new();
    while !q.is_empty() {
        // decrease the in_degree if all neighbors and if one of them have
        // in_degree of 1, push it to the q
        let size = q.len(); 
        // last_layer.clear();
        for _ in 0..size {
            let cur_node = q.pop_front().unwrap();
            // last_layer.push(cur_node as i32);
            ans = cur_node;
            for neighbor in &adj_list[cur_node] {
                in_degree[*neighbor as usize] -= 1;
                if in_degree[*neighbor as usize] == 1 {
                    q.push_back(*neighbor as usize);
                }
            }
        }
    }
    ans as _
    // last_layer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let edges = vec![vec![1, 2], vec![2, 3], vec![4, 2]];
        let output = 2;
        let result = find_center(edges);
        assert_eq!(result, output);
    }
}
