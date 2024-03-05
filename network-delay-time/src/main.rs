fn main() {
    println!("Hello, world!");
    let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
    let n = 4;
    let k = 2;
    // Output: 2
    println!("{}", network_delay_time(times, n, k));

    let times = vec![vec![1, 2, 1]];
    let n = 2;
    let k = 2;
    // Output: -1
    println!("{}", network_delay_time(times, n, k));
}

pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let (n, k) = (n as usize, k as usize);
    let adj_list = convert_graph(times, n + 1);
    println!("adj_list={:?}", adj_list);
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut bh = BinaryHeap::new();
    //    weight, node
    bh.push((Reverse(0), k));
    let mut dist = vec![i32::MAX; n + 1];
    let mut visited = vec![false; n + 1];
    dist[k] = 0;

    while let Some(curr_node) = bh.pop() {
        let (curr_node, Reverse(curr_weight)) = (curr_node.1, curr_node.0);
        visited[curr_node] = true;
        for neighbor in adj_list[curr_node as usize].iter() {
            let (neighbor_node, neighbor_weight) = (neighbor.to, neighbor.weight);
            if dist[curr_node] != i32::MAX
                && dist[neighbor_node] > dist[curr_node] + neighbor_weight
            {
                dist[neighbor_node] = dist[curr_node] + neighbor_weight;
            }
            if !visited[neighbor_node] {
                bh.push((Reverse(curr_weight + neighbor_weight), neighbor_node));
            }
        }
        println!("bh={:?}", bh);
    }
    println!("dist={:?}", dist);
    let f = *dist.iter().skip(1).max().unwrap();
    if f == i32::MAX {
        -1
    } else {
        f
    }
}

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    weight: i32,
}

fn convert_graph(times: Vec<Vec<i32>>, n: usize) -> Vec<Vec<Edge>> {
    let mut rezult = vec![vec![]; n];
    for edge in times {
        let from = edge[0] as usize;
        let to = edge[1] as usize;
        let weight = edge[2];

        rezult[from].push(Edge { to, weight });
    }
    rezult
}
