pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
    /*
    dijkstra algo from every node or floyd warshall
    dijkstra adjacent list
    flody adjacent matrix
    */
    let mut adjacent_list = vec![vec![]; n as usize];
    for edge in edges.iter() {
        let from = edge[0];
        let to = edge[1];
        let weight = edge[2];
        // push weight first because we would use priority queue
        adjacent_list[from as usize].push((weight, to));
        adjacent_list[to as usize].push((weight, from));
    }

    fn dijkstra(source: i32, n: usize, adjacent_list: &Vec<Vec<(i32, i32)>>) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut pq = BinaryHeap::new();

        pq.push((Reverse(0), source));
        let mut distance = vec![i32::MAX; n];

        distance[source as usize] = 0;
        let mut lvl = 0;

        while !pq.is_empty() {
            let size = pq.len();
            for _ in 0..size {
                let cur_node = pq.pop().unwrap();

                for &neighbor in adjacent_list[cur_node.1 as usize].iter() {
                    let n_node = neighbor.1;
                    let n_weight = neighbor.0;

                    /*
                          4
                      A ----> C
                      |        |
                    1 |        | 2
                      B--------|
                    */

                    // if distance of `to` bigger than accumlated distance that I calculated
                    // then change it

                    let new_distance = cur_node.0 .0 + n_weight;

                    if distance[n_node as usize] > new_distance {
                        distance[n_node as usize] = new_distance;
                        pq.push((Reverse(new_distance), n_node));
                    }
                }
            }
            lvl += 1;
        }
        distance
    }

    let mut min_cities_within_threshold = 0;
    let mut min_cities = i32::MAX;
    for i in 0..adjacent_list.len() {
        let distances = dijkstra(i as i32, n as usize, &adjacent_list);

        let count = distances
            .iter()
            .filter(|&x| x <= &distance_threshold)
            .count() as i32;

        if min_cities >= count {
            min_cities = count;
            min_cities_within_threshold = i;
        }
    }

    min_cities_within_threshold as i32
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
