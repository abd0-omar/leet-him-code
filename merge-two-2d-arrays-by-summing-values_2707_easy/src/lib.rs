// https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // it's better to be solved using two pointers
        let mut hm = std::collections::BTreeMap::new();
        for num in nums1.iter().chain(nums2.iter()) {
            let id = num[0];
            let val = num[1];
            *hm.entry(id).or_insert(0) += val;
        }
        hm.into_iter().map(|(id, val)| vec![id, val]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums1 = vec![vec![1, 2], vec![2, 3], vec![4, 5]];
        let nums2 = vec![vec![1, 4], vec![3, 2], vec![4, 1]];
        let output = vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]];
        let result = Solution::merge_arrays(nums1, nums2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums1 = vec![vec![2, 4], vec![3, 6], vec![5, 5]];
        let nums2 = vec![vec![1, 3], vec![4, 3]];
        let output = vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]];
        let result = Solution::merge_arrays(nums1, nums2);
        assert_eq!(result, output);
    }
}
