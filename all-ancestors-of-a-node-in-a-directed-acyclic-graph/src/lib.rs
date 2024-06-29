pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut result: Vec<Vec<i32>> = vec![vec![]; n];
    // topo sort
    let mut adj_list = vec![vec![]; n];
    let mut in_degree = vec![0; n];
    for edge in edges {
        let from = edge[0];
        let to = edge[1];
        in_degree[to as usize] += 1;
        adj_list[from as usize].push(to);
    }
    dbg!(&in_degree);
    dbg!(&adj_list);

    // put nodes with degree 0 in the queue
    let mut ready_queue = std::collections::VecDeque::new();
    for (node, &degree) in in_degree.iter().enumerate() {
        if degree == 0 {
            ready_queue.push_back(node);
        }
    }

    dbg!(&ready_queue);

    // iterate on neighbors of nodes from the ready_queue and whenever
    // a node is "ready" put in the queue

    let mut sorted_result = vec![std::collections::BTreeSet::new(); n];

    while !ready_queue.is_empty() {
        let size = ready_queue.len();

        for _ in 0..size {
            let cur_node = ready_queue.pop_front().unwrap();

            for &neighbor in adj_list[cur_node].iter() {
                let neighbor = neighbor as usize;
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 0 {
                    ready_queue.push_back(neighbor);
                }
                let mut b_set = std::collections::BTreeSet::new();
                b_set.insert(cur_node);
                // result[neighbor].push(cur_node as i32);
                sorted_result[neighbor].insert(cur_node as i32);
                for &ancestor in sorted_result[cur_node].clone().iter() {
                    // result[neighbor].push(ancestor);
                    sorted_result[neighbor].insert(ancestor);
                }
            }
        }
    }

    dbg!(&sorted_result);
    let mut result = vec![vec![]; n];
    for (i, res) in sorted_result.iter_mut().enumerate() {
        let f = res.iter().copied().collect::<Vec<i32>>();
        for x in f {
            result[i].push(x);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 8;
        let edges = vec![
            vec![0, 3],
            vec![0, 4],
            vec![1, 3],
            vec![2, 4],
            vec![2, 7],
            vec![3, 5],
            vec![3, 6],
            vec![3, 7],
            vec![4, 6],
        ];
        let output = vec![
            vec![],
            vec![],
            vec![],
            vec![0, 1],
            vec![0, 2],
            vec![0, 1, 3],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3],
        ];
        let result = get_ancestors(n, edges);
        assert_eq!(result, output);
    }
}
