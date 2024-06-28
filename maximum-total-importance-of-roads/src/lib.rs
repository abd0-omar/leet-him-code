pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    // adj_list
    let n = n as usize;
    let mut adj_list = vec![vec![]; n];
    for road in roads.iter() {
        let from = road[0];
        let to = road[1];
        // bidirectional
        adj_list[from as usize].push(to);
        adj_list[to as usize].push(from);
    }
    dbg!(&adj_list);

    // sort based of number of edges they have, aka len of the vec of edges they have
    let mut order = Vec::with_capacity(n);
    for (node, edge) in adj_list.iter().enumerate() {
        order.push((edge.len(), node));
    }
    order.sort_unstable();
    dbg!(&order);

    // assign value for each node based on `order`
    let mut hm = std::collections::HashMap::new();
    for (count, (_, node)) in order.iter().enumerate() {
        hm.insert(node, count + 1);
    }
    dbg!(&hm);

    // iterate on edge list (roads) of adj_list and sum the values from hm
    // maybe adj_list won't work because of duplicates
    // let mut sum = 0;
    // for (node, edges) in adj_list.iter().enumerate() {
    //     for &edge in edges.iter() {
    //         sum += hm[&(node)];
    //         dbg!(sum);
    //         sum += hm[&(edge as usize)];
    //         dbg!(sum);
    //     }
    // }
    // edge list
    let mut sum = 0;
    for road in roads {
        let from = road[0];
        let to = road[1];
        sum += hm[&(from as usize)];
        sum += hm[&(to as usize)];
    }
    sum as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 5;
        let roads = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![0, 2],
            vec![1, 3],
            vec![2, 4],
        ];
        let output = 43;
        let result = maximum_importance(n, roads);
        assert_eq!(result, output);
    }
}
