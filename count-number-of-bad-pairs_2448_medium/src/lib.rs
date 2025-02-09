// https://leetcode.com/problems/count-number-of-bad-pairs/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        // if num - index == num - index (other number) then it's not a "bad
        // pair"
        //                                  ˅
        //         let nums = vec![4, 1, 3, 3];
        // num - index
        // 3 - 3 == 0
        // search for any prev 0 in the hashamp
        // then that's a "good pair"
        let mut prev_calc = std::collections::HashMap::new();
        let mut good_pairs = 0;
        let n = nums.len();
        for (idx, num) in nums.into_iter().enumerate() {
            let calc = num - idx as i32;
            if let Some(&prev) = prev_calc.get(&calc) {
                good_pairs += prev;
            }
            *prev_calc.entry(calc).or_insert(0i64) += 1;
        }
        // dbg!(&good_pairs);
        // [x, x] total pairs 1
        // diff 2
        // [x, x, x] total pairs 3
        // diff 3
        // [x, x, x, x] total pairs 6
        // diff 4
        // [x, x, x, x, x] total pairs 10
        // diff 5
        // [x, x, x, x, x, x] total pairs 15
        // n (n - 1) / 2
        // can be done recursively
        // if n == 0 return 0
        // return n - 1 + calc_pairs(n - 1)
        let total_pairs = n * (n - 1) / 2;
        total_pairs as i64 - good_pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![4, 1, 3, 3];
        let output = 5;
        let result = Solution::count_bad_pairs(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 2, 3, 4, 5];
        let output = 0;
        let result = Solution::count_bad_pairs(nums);
        assert_eq!(result, output);
    }
}
