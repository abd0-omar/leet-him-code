// https://leetcode.com/problems/find-building-where-alice-and-bob-can-meet/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // monotonic stack find next larger element bigger than alice & bob
        let n = heights.len();
        let mut new_queries: Vec<Vec<_>> = vec![vec![]; n];
        let mut result = vec![-1; queries.len()];
        let mut stack = vec![(-1, 0); n];
        for (idx, query) in queries.iter().enumerate() {
            let mut a = query[0] as usize;
            let mut b = query[1] as usize;
            // always make alice behind bob
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            // bob is bigger than alice then the answer is bob's index
            if heights[b] > heights[a] || a == b {
                result[idx] = b as i32;
            } else {
                // the ((heights[a], idx)), look for num higher or equal than heights[a]
                // for query index idx
                //
                // we are sure that a'value is higher than b's value here
                new_queries[b as usize].push((heights[a], idx));
            }
        }

        // dbg!(&new_queries);
        for i in (0..n).rev() {
            for &(a, main_queries_idx) in new_queries[i].iter() {
                // bianary search on the stack that is monotonically decreasing (sorted)
                // for a height that is bigger than a
                let position = binary_search(a, &stack);
                if let Some(pos) = position {
                    result[main_queries_idx as usize] = stack[pos as usize].1 as i32;
                }
            }
            // monotonically decreasing
            while !stack.is_empty() && stack.last().unwrap().0 <= heights[i] {
                stack.pop();
            }
            // heights[i] for comparison, i for result
            stack.push((heights[i], i));
            dbg!(&result);
            dbg!(&stack);
        }
        result
    }
}

fn binary_search(a: i32, stack: &[(i32, usize)]) -> Option<i32> {
    let mut l = 0;
    let mut r = stack.len() - 1;
    let mut ans: Option<i32> = None;
    while l <= r {
        let mid = l + (r - l) / 2;
        if stack[mid].0 > a {
            if let Some(a) = ans {
                ans = Some(a.max(mid as i32));
            } else {
                ans = Some(mid as i32);
            }
            l = mid + 1;
        } else {
            if let (_, true) = mid.overflowing_sub(1) {
                break;
            }
            r = mid - 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let heights = vec![6, 4, 8, 5, 2, 7];
        let queries = vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]];
        let output = vec![2, 5, -1, 5, 2];
        let result = Solution::leftmost_building_queries(heights, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let heights = vec![5, 3, 8, 2, 6, 1, 4, 6];
        let queries = vec![vec![0, 7], vec![3, 5], vec![5, 2], vec![3, 0], vec![1, 6]];
        let output = vec![7, 6, -1, 4, 6];
        let result = Solution::leftmost_building_queries(heights, queries);
        assert_eq!(result, output);
    }
}
