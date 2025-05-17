// https://leetcode.com/problems/sort-colors/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // quick-sort
        let n = nums.len();
        if n > 0 {
            quick_sort(nums, 0, n - 1);
        }
    }
}

fn quick_sort(nums: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot = partition(nums, low, high);
        if pivot > 0 {
            quick_sort(nums, low, pivot - 1);
        }
        quick_sort(nums, pivot + 1, high);
    }
}

fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = nums[high];
    let mut i = low;

    for j in low..high {
        if nums[j] < pivot {
            nums.swap(i, j);
            i += 1;
        }
    }
    nums.swap(i, high);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let output = vec![0, 0, 1, 1, 2, 2];
        let _result = Solution::sort_colors(&mut nums);
        assert_eq!(nums, output);
    }

    #[test]
    fn it_works1() {
        let mut nums = vec![2, 0, 1];
        let output = vec![0, 1, 2];
        let _result = Solution::sort_colors(&mut nums);
        assert_eq!(nums, output);
    }
}
