use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn modified_graph_edges(
    n: i32,
    edges: Vec<Vec<i32>>,
    source: i32,
    destination: i32,
    target: i32,
) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut graph = vec![HashMap::<usize, i32>::new(); n];
    let mut can_modify_edges = vec![];

    for edge in &edges {
        let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
        if w == -1 {
            can_modify_edges.push((u, v));
        }
        if w != -1 {
            graph[u].insert(v, w);
        }
        if w != -1 {
            graph[v].insert(u, w);
        }
    }
    dbg!(&graph);

    // Run Dijkstra from the source
    let mut dist = vec![i32::MAX; n];
    dijkstra(&graph, &mut dist, source as usize);

    dbg!(&dist);
    dbg!(&can_modify_edges);

    // Adjust edges with -1 weight to achieve the target distance
    while dist[destination as usize] > target && !can_modify_edges.is_empty() {
        let mut dist_from_dest = vec![i32::MAX; n];
        dijkstra(&graph, &mut dist_from_dest, destination as usize);
        dbg!(&dist_from_dest);

        let (u, v) = can_modify_edges.pop().unwrap();
        // pick the shortest distance from source to the other node destination
        let t = (dist[u] + dist_from_dest[v]).min(dist[v] + dist_from_dest[u]);
        let adjust = if t >= target { 1 } else { target - t };

        graph[u].insert(v, adjust);
        graph[v].insert(u, adjust);

        // dijkstra again after modifing the edges
        dijkstra(&graph, &mut dist, source as usize);
    }

    // If we can't achieve the target distance, return an empty vector
    if dist[destination as usize] != target {
        return vec![];
    }

    // Reconstruct the final edges with the correct weights
    let mut result = vec![];
    for edge in edges {
        let (u, v) = (edge[0] as usize, edge[1] as usize);
        if let Some(w) = graph[u].get(&v) {
            result.push(vec![u as i32, v as i32, *w]);
        } else {
            result.push(vec![u as i32, v as i32, 1_000_000_000]);
        }
    }

    dbg!(&result);
    result
}

fn dijkstra(graph: &Vec<HashMap<usize, i32>>, dist: &mut Vec<i32>, source: usize) {
    for u in 0..graph.len() {
        dist[u] = 1_000_000_000;
    }
    dist[source] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, source)));

    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] {
            continue;
        }

        for p in &graph[u] {
            let (v, w) = (*p.0, *p.1);
            if dist[v] - dist[u] <= w {
                continue;
            }
            dist[v] = w + dist[u];
            pq.push(Reverse((dist[v], v)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 4;
        let edges = vec![vec![1, 0, 4], vec![1, 2, 3], vec![2, 3, 5], vec![0, 3, -1]];
        let source = 0;
        let destination = 2;
        let target = 6;
        let output = vec![vec![1, 0, 4], vec![1, 2, 3], vec![2, 3, 5], vec![0, 3, 1]];
        let result = modified_graph_edges(n, edges, source, destination, target);
        assert_eq!(result, output);
    }
}
