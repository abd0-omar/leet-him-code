pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
    // a dijkstra where each edge weight is the same, is just bfs
    let n = n as usize;
    let mut adj_list = vec![vec![]; n + 1];
    for edge in edges {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        adj_list[from].push(to);
        adj_list[to].push(from);
    }

    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back(1);

    let mut cur_time = 0;
    let mut result: Option<i32> = None;
    let mut visited = vec![vec![]; n + 1];

    while !q.is_empty() {
        let size = q.len();
        for _ in 0..size {
            let cur_node = q.pop_front().unwrap();
            if cur_node == n {
                if result.is_some() {
                    return cur_time;
                }
                result = Some(cur_time);
            }

            for &neighbor in adj_list[cur_node as usize].iter() {
                if visited[neighbor].len() == 0
                    || (visited[neighbor].len() == 1 && visited[neighbor][0] != cur_time)
                {
                    q.push_back(neighbor);
                    visited[neighbor].push(cur_time);
                }
            }
        }
        if (cur_time / change) % 2 == 1 {
            cur_time += change - (cur_time % change);
        }
        cur_time += time;
    }
    cur_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 5;
        let edges = vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![3, 4], vec![4, 5]];
        let time = 3;
        let change = 5;
        let output = 13;
        let result = second_minimum(n, edges, time, change);
        assert_eq!(result, output);
    }
}
