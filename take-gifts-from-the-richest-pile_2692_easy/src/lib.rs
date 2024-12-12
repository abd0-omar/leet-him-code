// https://leetcode.com/problems/take-gifts-from-the-richest-pile/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut max_heap = std::collections::BinaryHeap::from(gifts);
        for _ in 0..k {
            let top = max_heap.pop().unwrap();
            let sqrt_top = (top as f64).sqrt() as i32;
            max_heap.push(sqrt_top);
        }
        max_heap.into_iter().map(|x| x as i64).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let gifts = vec![25, 64, 9, 4, 100];
        let k = 4;
        let output = 29;
        let result = Solution::pick_gifts(gifts, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let gifts = vec![1, 1, 1, 1];
        let k = 4;
        let output = 4;
        let result = Solution::pick_gifts(gifts, k);
        assert_eq!(result, output);
    }
}
