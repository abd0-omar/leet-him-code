fn main() {
    // println!("Hello, world!");
    // let points = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
    // Output: 20
    // println!("{}", min_cost_connect_points(points));

    // let points = vec![vec![0, 0], vec![1, 1], vec![1, 0], vec![-1, 1]];
    // println!("{}", min_cost_connect_points(points));
}

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    weight: i32,
}

fn change_to_adj_list_undirected(points: Vec<Vec<i32>>) -> Vec<Vec<Edge>> {
    let n = points.len();
    let mut rezult = vec![vec![]; n];

    // calc manhatan dist for each node
    for i in 0..n {
        for j in i + 1..n {
            let manhatan_dist =
                (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
            let edge = Edge {
                to: j,
                weight: manhatan_dist,
            };
            rezult[i].push(edge);
            rezult[j].push(Edge {
                to: i,
                weight: manhatan_dist,
            });
        }
    }

    println!("rezult={:?}", rezult);
    rezult
}

pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let adj_list = change_to_adj_list_undirected(points);
    let n = adj_list.len();
    println!("n={:?}", n);
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut pq = BinaryHeap::new();
    //    weight, node
    pq.push((Reverse(0), 0));

    let mut dist = vec![i32::MAX; n];
    let mut visited = vec![false; n];
    dist[0] = 0;
    let mut mst_cost = 0;

    while let Some(curr_node) = pq.pop() {
        println!("curr_node={:?}", curr_node);
        let (curr_node, Reverse(curr_weight)) = (curr_node.1, curr_node.0);
        if visited[curr_node] {
            continue;
        }

        println!("curr_node={:?}", curr_node);
        mst_cost += curr_weight;

        for neighbor in adj_list[curr_node].iter() {
            let (neighbor_node, neighbor_weight) = (neighbor.to, neighbor.weight);

            println!("neighbor_node={:?}", neighbor_node);
            if dist[neighbor_node] > neighbor_weight {
                dist[neighbor_node] = neighbor_weight;
            }

            println!("dist={:?}", dist);
            println!("neighbor_weight={:?}", neighbor_weight);
            pq.push((Reverse(dist[neighbor_node]), neighbor_node));
        }
        visited[curr_node] = true;
    }

    println!("mst_cost={:?}", mst_cost);
    dist.iter().sum()
}
