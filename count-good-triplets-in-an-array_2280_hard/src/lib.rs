// https://leetcode.com/problems/count-good-triplets-in-an-array/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
// This solution directly uses the solution of problem #315
impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        // index map
        let mut data = vec![0; n];
        for (i, a) in nums1.iter().enumerate() {
            data[*a as usize] = i;
        }

        // v1: sorting of index of nums2 based on the order nums1
        // therefore, the answer to find the # increasing sub-sequences of length 3
        let (mut v1, mut v2) = (vec![0i32; n], vec![0i32; n]);
        for (i, a) in nums2.iter().enumerate() {
            v1[i] = data[*a as usize] as i32;
            v2[i] = n as i32 - 1 - v1[i];
        }

        // ret1[i]: # of elements bigger than v[i] on the right of i
        // ret2[i]: # of elements smaller than v[i] on the left of i
        let (mut ret1, mut ret2) = (vec![0; n], vec![0; n]);
        Self::count_smaller(&v2, &mut ret1);
        v1.reverse();
        Self::count_smaller(&v1, &mut ret2);
        ret2.reverse();

        let mut ret = 0;
        for i in 1..n - 1 {
            ret += (ret1[i] as i64) * (ret2[i] as i64);
        }

        ret
    }

    // this is the solution to #315:
    // https://leetcode.com/problems/count-of-smaller-numbers-after-self/
    fn count_smaller(nums: &[i32], ret: &mut [i32]) {
        let n = nums.len();
        let mut indexes = (0..n).collect::<Vec<usize>>();

        Self::merge_sort(nums, &mut indexes, ret, 0, n - 1);
    }

    fn merge_sort(nums: &[i32], indexes: &mut [usize], ret: &mut [i32], left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mid = left + (right - left) / 2;
        Self::merge_sort(nums, indexes, ret, left, mid);
        Self::merge_sort(nums, indexes, ret, mid + 1, right);
        Self::merge(nums, indexes, ret, left, right, mid)
    }

    fn merge(
        nums: &[i32],
        indexes: &mut [usize],
        ret: &mut [i32],
        left: usize,
        right: usize,
        mid: usize,
    ) {
        let (mut i, mut j) = (left, mid + 1);
        let mut cnt = 0;
        let mut temp = Vec::with_capacity(nums.len());

        while i <= mid && j <= right {
            if nums[indexes[j]] < nums[indexes[i]] {
                cnt += 1;
                temp.push(indexes[j]);
                j += 1;
            } else {
                temp.push(indexes[i]);
                ret[indexes[i]] += cnt;
                i += 1;
            }
        }

        while i <= mid {
            temp.push(indexes[i]);
            ret[indexes[i]] += cnt;
            i += 1;
        }

        while j <= right {
            temp.push(indexes[j]);
            j += 1;
        }

        for (it, a) in temp.iter().enumerate() {
            indexes[left + it] = *a;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums1 = vec![2, 0, 1, 3];
        let nums2 = vec![0, 1, 2, 3];
        let output = 1;
        let result = Solution::good_triplets(nums1, nums2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums1 = vec![4, 0, 1, 3, 2];
        let nums2 = vec![4, 1, 0, 2, 3];
        let output = 4;
        let result = Solution::good_triplets(nums1, nums2);
        assert_eq!(result, output);
    }
}
