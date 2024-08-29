use std::collections::{HashMap, HashSet};

pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    let mut graph_row = HashMap::new();
    let mut graph_col = HashMap::new();

    for stone in &stones {
        let row = stone[0];
        let col = stone[1];

        graph_row.entry(row).or_insert(vec![]).push((row, col));
        graph_col.entry(col).or_insert(vec![]).push((row, col));
    }

    let mut visited = HashSet::new();
    let mut num_connected_components = 0;

    for stone in &stones {
        let row = stone[0];
        let col = stone[1];
        if !visited.contains(&(row, col)) {
            num_connected_components += 1;
            dfs(&graph_row, &graph_col, (row, col), &mut visited);
        }
    }

    dbg!(stones.len());
    dbg!(num_connected_components);

    stones.len() as i32 - num_connected_components
}

fn dfs(
    graph_row: &HashMap<i32, Vec<(i32, i32)>>,
    graph_col: &HashMap<i32, Vec<(i32, i32)>>,
    stone: (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
) {
    if visited.contains(&stone) {
        return;
    }

    visited.insert(stone);

    let (row, col) = stone;

    // Visit all stones in the same row
    if let Some(neighbors) = graph_row.get(&row) {
        for &neighbor in neighbors {
            dfs(graph_row, graph_col, neighbor, visited);
        }
    }

    // Visit all stones in the same column
    if let Some(neighbors) = graph_col.get(&col) {
        for &neighbor in neighbors {
            dfs(graph_row, graph_col, neighbor, visited);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let stones = vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2],
        ];
        let output = 5;
        let result = remove_stones(stones);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let stones = vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]];
        let output = 3;
        let result = remove_stones(stones);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let stones = vec![vec![0, 0]];
        let output = 0;
        let result = remove_stones(stones);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let stones = vec![vec![0, 1], vec![1, 0]];
        let output = 0;
        let result = remove_stones(stones);
        assert_eq!(result, output);
    }
}
