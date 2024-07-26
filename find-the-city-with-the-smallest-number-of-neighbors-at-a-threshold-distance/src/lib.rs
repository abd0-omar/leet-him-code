pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
    // /*
    // dijkstra algo from every node or floyd warshall
    // dijkstra adjacent list
    // flody adjacent matrix
    // */
    // let mut adjacent_list = vec![vec![]; n as usize];
    // for edge in edges.iter() {
    //     let from = edge[0];
    //     let to = edge[1];
    //     let weight = edge[2];
    //     // push weight first because we would use priority queue
    //     adjacent_list[from as usize].push((weight, to));
    //     adjacent_list[to as usize].push((weight, from));
    // }

    // fn dijkstra(source: i32, n: usize, adjacent_list: &Vec<Vec<(i32, i32)>>) -> Vec<i32> {
    //     use std::cmp::Reverse;
    //     use std::collections::BinaryHeap;
    //     let mut pq = BinaryHeap::new();

    //     pq.push((Reverse(0), source));
    //     let mut distance = vec![i32::MAX; n];

    //     distance[source as usize] = 0;

    //     while !pq.is_empty() {
    //         let size = pq.len();
    //         for _ in 0..size {
    //             let cur_node = pq.pop().unwrap();

    //             for &neighbor in adjacent_list[cur_node.1 as usize].iter() {
    //                 let n_node = neighbor.1;
    //                 let n_weight = neighbor.0;

    //                 /*
    //                       4
    //                   A ----> C
    //                   |        |
    //                 1 |        | 2
    //                   B--------|
    //                 */
    //                 // if distance of `to` bigger than accumlated distance that I calculated
    //                 // then change it

    //                 let new_distance = cur_node.0 .0 + n_weight;

    //                 if distance[n_node as usize] > new_distance {
    //                     distance[n_node as usize] = new_distance;
    //                     pq.push((Reverse(new_distance), n_node));
    //                 }
    //             }
    //         }
    //     }
    //     distance
    // }

    // let mut min_cities_within_threshold = 0;
    // let mut min_cities = i32::MAX;
    // for i in 0..adjacent_list.len() {
    //     let distances = dijkstra(i as i32, n as usize, &adjacent_list);

    //     let count = distances
    //         .iter()
    //         .filter(|&x| x <= &distance_threshold)
    //         .count() as i32;

    //     if min_cities >= count {
    //         min_cities = count;
    //         min_cities_within_threshold = i;
    //     }
    // }

    // min_cities_within_threshold as i32

    // floyd mayweather algorithm
    // first define adjacent matrix

    // let n = n as usize;
    // let mut adjacent_matrix = vec![vec![i32::MAX; n]; n];

    // // shortest dist to me is zero
    // for i in 0..n {
    //     adjacent_matrix[i][i] = 0;
    // }

    // // populate adj_matrix with the given weight
    // for edge in edges {
    //     let from = edge[0] as usize;
    //     let to = edge[1] as usize;
    //     let weight = edge[2];
    //     adjacent_matrix[from][to] = weight;
    //     adjacent_matrix[to][from] = weight;
    // }

    // dbg!(&adjacent_matrix);

    // // floyd algo
    // // (k) (i)m (j)
    // // relax = adj_matrix[i][k] + adj_matrix[k][j]
    // // if relax < adj_matrix[i][j]
    // // adj_matrix[i][j] = relax

    // for k in 0..n {
    //     for i in 0..n {
    //         for j in 0..n {
    //             if adjacent_matrix[i][k] != i32::MAX && adjacent_matrix[k][j] != i32::MAX {
    //                 let relax = adjacent_matrix[i][k] + adjacent_matrix[k][j];
    //                 if relax < adjacent_matrix[i][j] {
    //                     adjacent_matrix[i][j] = relax;
    //                 }
    //             }
    //         }
    //     }
    // }

    // let mut min_city = i32::MAX;
    // let mut min_city_idx = 0;
    // for i in 0..n {
    //     let count = adjacent_matrix[i]
    //         .iter()
    //         .filter(|&x| x <= &distance_threshold)
    //         .count() as i32;

    //     if count <= min_city {
    //         min_city = count;
    //         min_city_idx = i;
    //     }
    // }

    // min_city_idx as _

    // bellman ford shortest path from source
    // useful when
    // you want to compute all shortest distances from single source
    // edges have -ve weight
    // works on edge list directly, so no need to conver to adj(list/matrix) cuz most problems give you an edge list

    fn el7g_bellman_ford(n: usize, edges: &Vec<Vec<i32>>, source: usize) -> Vec<i32> {
        let mut distance = vec![i32::MAX; n];

        distance[source] = 0;

        // three loops
        // one iterator
        // one to
        // one neighbors
        //
        // then do your normal relaxation

        // relax edges n-1 times
        for _ in 0..n - 1 {
            for edge in edges.iter() {
                let from = edge[0] as usize;
                let to = edge[1] as usize;
                let weight = edge[2];

                if distance[from] != i32::MAX {
                    let relax = distance[from] + weight;

                    if relax < distance[to] {
                        distance[to] = relax;
                    }
                }

                // cuz it's bidirectional edges in the graph
                if distance[to] != i32::MAX {
                    let relax = distance[to] + weight;

                    if relax < distance[from] {
                        distance[from] = relax;
                    }
                }
            }
        }
        distance
    }

    let mut min_city = i32::MAX;
    let mut min_city_idx = 0;
    let n = n as usize;
    for i in 0..n {
        let distance = el7g_bellman_ford(n, &edges, i);

        let count = distance
            .iter()
            .filter(|&x| x <= &distance_threshold)
            .count() as i32;

        if count <= min_city {
            min_city_idx = i;
            min_city = count;
        }
    }

    min_city_idx as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 4;
        let edges = vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]];
        let distance_threshold = 4;
        let output = 3;
        let result = find_the_city(n, edges, distance_threshold);
        assert_eq!(result, output);
    }
}
