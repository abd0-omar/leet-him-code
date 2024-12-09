// https://leetcode.com/problems/special-array-ii/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        // // brute-force is trivial
        // let mut result = Vec::with_capacity(queries.len());
        // for query in queries {
        //     let from = query[0] as usize;
        //     let to = query[1] as usize;
        //     if !special(&nums, from, to) {
        //         result.push(false);
        //         continue;
        //     }
        //     result.push(true);
        // }
        // result
        // Tle on the last test case

        // prefix-sum (count) the index of special arrays
        let n = nums.len();
        let mut prefix_sum = vec![0; n];
        for i in 1..n {
            if nums[i] % 2 == nums[i - 1] % 2 {
                prefix_sum[i] = prefix_sum[i - 1] + 1;
            } else {
                prefix_sum[i] = prefix_sum[i - 1];
            }
        }
        // dbg!(&prefix_sum);
        let mut result = Vec::with_capacity(n);
        for query in queries {
            let from = query[0] as usize;
            let to = query[1] as usize;
            if prefix_sum[to] - prefix_sum[from] == 0 {
                result.push(true);
            } else {
                result.push(false);
            }
        }
        result
    }
}

fn special(nums: &[i32], from: usize, to: usize) -> bool {
    for i in from + 1..=to {
        // cmp with prev
        if nums[i - 1] % 2 == nums[i] % 2 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![3, 4, 1, 2, 6];
        let queries = vec![vec![0, 4]];
        let output = vec![false];
        let result = Solution::is_array_special(nums, queries);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![4, 3, 1, 6];
        let queries = vec![vec![0, 2], vec![2, 3]];
        let output = vec![false, true];
        let result = Solution::is_array_special(nums, queries);
        assert_eq!(result, output);
    }
}
