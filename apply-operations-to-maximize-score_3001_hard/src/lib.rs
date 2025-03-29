// https://leetcode.com/problems/apply-operations-to-maximize-score/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        const MOD: i64 = 1_000_000_007;
        let mut result = 1;
        let mut prime_scores = vec![0; n];

        for (i, &num) in nums.iter().enumerate() {
            let mut score = 0;
            let mut num = num;
            let limit = (num as f64).sqrt() as i32 + 1;
            for f in 2..=limit {
                if num % f == 0 {
                    score += 1;
                    while num % f == 0 {
                        num /= f;
                    }
                }
            }
            if num > 1 {
                score += 1;
            }
            prime_scores[i] = score;
        }

        let mut left_bound = vec![-1; n];
        let mut right_bound = vec![n as i32; n];

        // Compute left bounds (strictly greater)
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..n {
            while let Some(&last) = stack.last() {
                if prime_scores[last] < prime_scores[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            left_bound[i] = stack.last().map(|&idx| idx as i32).unwrap_or(-1);
            stack.push(i);
        }

        // Compute right bounds (strictly greater)
        stack.clear();
        for i in (0..n).rev() {
            while let Some(&last) = stack.last() {
                if prime_scores[last] <= prime_scores[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            right_bound[i] = stack.last().map(|&idx| idx as i32).unwrap_or(n as i32);
            stack.push(i);
        }

        use std::collections::BinaryHeap;

        let mut max_heap: BinaryHeap<_> = nums
            .iter()
            .enumerate()
            .map(|(idx, &num)| (num, idx))
            .collect();

        let mut k = k as i64;
        while k > 0 {
            let (num, idx) = max_heap.pop().unwrap();

            // Fix left and right segment calculations
            let left_count = idx as i64 - left_bound[idx] as i64;
            let right_count = right_bound[idx] as i64 - idx as i64;

            let count = left_count * right_count;
            let operations = count.min(k);

            result = (result * Self::mod_pow(num as i64, operations, MOD)) % MOD;

            k -= operations;
        }

        result as i32
    }

    fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
        let mut result = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % modulus;
            }
            base = (base * base) % modulus;
            exp /= 2;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![8, 3, 9, 3, 8];
        let k = 2;
        let output = 81;
        let result = Solution::maximum_score(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![19, 12, 14, 6, 10, 18];
        let k = 3;
        let output = 4788;
        let result = Solution::maximum_score(nums, k);
        assert_eq!(result, output);
    }
}
