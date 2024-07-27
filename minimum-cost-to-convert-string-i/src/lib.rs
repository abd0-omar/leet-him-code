//floyd
// make adj_matrix 26 * 26
// populate the adj_matrix with floyd's algo
// for k
//     for i
//         for j
// if a[i][k] != i64::MAX && a[k][j] != i64::MAX
// if a[i][k] + a[k][j] < a[i][j]
// a[i][j] = a[i][k] + a[k][j]
// dijkstra passed
pub fn minimum_cost(
    source: String,
    target: String,
    original: Vec<char>,
    changed: Vec<char>,
    cost: Vec<i32>,
) -> i64 {
    let mut adjacent_list = vec![vec![]; 26];

    let n = original.len();
    for i in 0..n {
        let from_idx = (original[i] as u8 - b'a') as usize;
        let to = changed[i] as u8;
        let weight = cost[i];
        adjacent_list[from_idx].push((weight, to));
    }
    dbg!(&adjacent_list);

    let mut min_cost_total = 0;

    let mut all_distances = vec![vec![0; 26]; 26];

    for i in 0..26 {
        all_distances[i] = dijkstra(&adjacent_list, i as u8 + b'a');
    }
    dbg!(&all_distances);

    for i in 0..source.len() {
        let source_letter = source.as_bytes()[i];
        let target_letter = target.as_bytes()[i];

        if source_letter != target_letter {
            // dbg!("start");
            // let distance = dijkstra(&adjacent_list, source_letter);
            // if distance[(target_letter - b'a') as usize] != i64::MAX {
            //     min_cost_total += distance[(target_letter - b'a') as usize];
            // } else {
            //     return -1; // Return -1 if no valid path exists
            // }
            if all_distances[(source_letter - b'a') as usize][(target_letter - b'a') as usize]
                != i64::MAX
            {
                min_cost_total +=
                    all_distances[(source_letter - b'a') as usize][(target_letter - b'a') as usize];
            } else {
                return -1;
            }
        }
    }
    min_cost_total
}

fn dijkstra(adjacent_list: &[Vec<(i32, u8)>], source_letter: u8) -> Vec<i64> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut distance = vec![i64::MAX; 26];
    distance[(source_letter - b'a') as usize] = 0;

    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), source_letter));

    while let Some(node) = pq.pop() {
        let cur_weight = node.0 .0;
        let cur_node = node.1;
        let cur_node_idx = (cur_node - b'a') as usize;

        for &(n_weight, n_node) in &adjacent_list[cur_node_idx] {
            let new_distance = cur_weight + n_weight as i64;

            if new_distance < distance[(n_node - b'a') as usize] {
                distance[(n_node - b'a') as usize] = new_distance;
                pq.push((Reverse(new_distance), n_node));
            }
        }
    }

    distance
}

// dfs tle
// pub fn minimum_cost(
//     source: String,
//     target: String,
//     original: Vec<char>,
//     changed: Vec<char>,
//     cost: Vec<i32>,
// ) -> i64 {
//     // looks a graph problem to get to the target letter with minimum cost
//     // map original with (changed, cost) in hashmap for a fast lookup
//     use std::collections::HashMap;
//     // c | [(b, 5), (e, 1)]
//     let mut hm = HashMap::new();

//     let n = original.len();
//     for i in 0..n {
//         hm.entry(original[i])
//             .or_insert(vec![])
//             .push((changed[i], cost[i]));
//     }
//     dbg!(&hm);

//     // for letter in source != target
//     // do dfs till you reach the target letter
//     // initialize min with lowest cost so far
//     // don't stop dfs when you reach target as there might be a smaller target

//     let mut min_cost_total = 0;
//     let mut target_chars = target.chars();
//     for source_letter in source.chars() {
//         let target_letter = target_chars.next().unwrap();
//         if source_letter != target_letter {
//             // dfs works but gets tle
//             // let mut min_cost: Option<i64> = None;
//             // // let mut visited = vec![false; n];
//             // dbg!("start");
//             // use std::collections::HashSet;
//             // let mut visited = HashSet::new();
//             // dfs(
//             //     &hm,
//             //     source_letter,
//             //     target_letter,
//             //     &mut min_cost,
//             //     0,
//             //     &mut visited,
//             // );
//             // if let Some(cost) = min_cost {
//             //     min_cost_total += cost;
//             // } else {
//             //     return -1;
//             // }

//         }
//     }
//     min_cost_total
// }

// // dfs works but gets tle
// // so a better idea to do dijkstra or something but then I would have to make it a graph first
// // c | [(5, b), (1, e), (i64::MAX, a), (i64::MAX, c), ...]
// fn dfs(
//     hm: &std::collections::HashMap<char, Vec<(char, i32)>>,
//     source_letter: char,
//     target_letter: char,
//     min_cost: &mut Option<i64>,
//     cur_cost: i64,
//     visited: &mut std::collections::HashSet<char>,
// ) -> () {
//     dbg!(source_letter);
//     dbg!(cur_cost);
//     if source_letter == target_letter {
//         if let Some(cost) = min_cost {
//             if cur_cost < *cost {
//                 *min_cost = Some(cur_cost);
//             }
//         } else {
//             // first time assigning to min_cost
//             // so it got to be the lowest so far
//             *min_cost = Some(cur_cost);
//         }
//         return;
//     }

//     visited.insert(source_letter);

//     if let Some(neighbors) = hm.get(&source_letter) {
//         for (neighbor_letter, neighbor_cost) in neighbors {
//             if !visited.contains(&neighbor_letter) {
//                 dfs(
//                     hm,
//                     *neighbor_letter,
//                     target_letter,
//                     min_cost,
//                     cur_cost + *neighbor_cost as i64,
//                     visited,
//                 );
//             }
//         }
//     }

//     visited.remove(&source_letter);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let source = "abcd".to_string();
        let target = "acbe".to_string();
        let original = vec!['a', 'b', 'c', 'c', 'e', 'd'];
        let changed = vec!['b', 'c', 'b', 'e', 'b', 'e'];
        let cost = vec![2, 5, 5, 1, 2, 20];
        let output = 28;
        let result = minimum_cost(source, target, original, changed, cost);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let source = "aaaabadaaa".to_string();
        let target = "dbdadddbad".to_string();
        let original = vec!['c', 'a', 'c', 'a', 'a', 'b', 'b', 'b', 'd', 'd', 'c'];
        let changed = vec!['a', 'c', 'b', 'd', 'b', 'c', 'a', 'd', 'c', 'b', 'd'];
        let cost = vec![7, 8, 11, 9, 7, 6, 4, 6, 9, 5, 9];
        let output = 56;
        let result = minimum_cost(source, target, original, changed, cost);
        assert_eq!(result, output);
    }
}
