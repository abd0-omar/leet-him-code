pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut adj_list = vec![vec![]; n];

    for road in roads {
        let from = road[0] as usize;
        let to = road[1] as usize;
        let weight = road[2];

        adj_list[from].push((to, weight));
        adj_list[to].push((from, weight));
    }

    let mut pq = std::collections::BinaryHeap::new();
    use std::cmp::Reverse;
    pq.push((Reverse(0), 0));
    let mut distance = vec![i64::MAX; n];
    distance[0] = 0;

    while let Some((Reverse(_), cur_node)) = pq.pop() {
        for &(neighbor, neighbor_weight) in adj_list[cur_node].iter() {
            let new_distance = if distance[cur_node] != i64::MAX {
                distance[cur_node] + neighbor_weight as i64
            } else {
                neighbor_weight as i64
            };
            if new_distance < distance[neighbor] {
                distance[neighbor] = new_distance;
                pq.push((Reverse(new_distance as usize), neighbor));
            }
        }
    }

    dbg!(&distance);
    let mut pq = std::collections::BinaryHeap::new();
    let mut count = vec![0; n];
    count[0] = 1;

    for (i, dist_i) in distance.iter().enumerate().take(n) {
        pq.push((Reverse(dist_i), i));
    }

    dbg!(&pq);

    while let Some((Reverse(_), cur_node)) = pq.pop() {
        dbg!(&count);
        if cur_node == n - 1 {
            return count[cur_node];
        }
        for (neighbor, weight) in &adj_list[cur_node] {
            if distance[*neighbor] == distance[cur_node] + *weight as i64 {
                count[*neighbor] = (count[*neighbor] + count[cur_node]) % 1_000_000_007;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 7;
        let roads = vec![
            vec![0, 6, 7],
            vec![0, 1, 2],
            vec![1, 2, 3],
            vec![1, 3, 3],
            vec![6, 3, 3],
            vec![3, 5, 1],
            vec![6, 5, 1],
            vec![2, 5, 1],
            vec![0, 4, 5],
            vec![4, 6, 2],
        ];
        let output = 4;
        let result = count_paths(n, roads);
        assert_eq!(result, output);
    }
}
