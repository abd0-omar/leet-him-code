use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

pub fn nth_ugly_number(n: i32) -> i32 {
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut result = -1;

    pq.push(Reverse(1_i64));
    visited.insert(1_i64);
    dfs(&mut pq, n as i64, 0, &mut visited, &mut result);

    result as _
}

fn dfs(
    pq: &mut BinaryHeap<Reverse<i64>>,
    n: i64,
    count: i64,
    visited: &mut HashSet<i64>,
    result: &mut i64,
) {
    if count == n {
        return;
    }

    if let Some(Reverse(num)) = pq.pop() {
        *result = num;

        let factors = [2, 3, 5];
        for &factor in &factors {
            let new_num = num * factor;
            if visited.insert(new_num) {
                pq.push(Reverse(new_num));
            }
        }

        dfs(pq, n, count + 1, visited, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 10;
        let output = 12;
        // Explanation: [1, 2, 3, 4, 5, 6, 8, 9, 10, 12] is the sequence of the first 10 ugly numbers.
        let result = nth_ugly_number(n);
        assert_eq!(result, output);
    }
}
