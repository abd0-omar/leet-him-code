// https://leetcode.com/problems/make-lexicographically-smallest-array-by-swapping-elements/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();
        let mut groups: Vec<std::collections::VecDeque<i32>> = vec![];
        let mut num_to_group = std::collections::HashMap::new();

        for num in sorted_nums {
            if groups.is_empty() || (num - groups.last().unwrap().back().unwrap()).abs() > limit {
                groups.push(std::collections::VecDeque::new());
            }
            let group_idx = groups.len() - 1;
            groups[group_idx].push_back(num);
            num_to_group.insert(num, group_idx);
        }

        let mut result = Vec::new();
        for num in nums {
            let group_idx = num_to_group[&num];
            result.push(groups[group_idx].pop_front().unwrap());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 5, 3, 9, 8];
        let limit = 2;
        let output = vec![1, 3, 5, 8, 9];
        let result = Solution::lexicographically_smallest_array(nums, limit);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 7, 6, 18, 2, 1];
        let limit = 3;
        let output = vec![1, 6, 7, 18, 1, 2];
        let result = Solution::lexicographically_smallest_array(nums, limit);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![1, 7, 28, 19, 10];
        let limit = 3;
        let output = vec![1, 7, 28, 19, 10];
        let result = Solution::lexicographically_smallest_array(nums, limit);
        assert_eq!(result, output);
    }
}
