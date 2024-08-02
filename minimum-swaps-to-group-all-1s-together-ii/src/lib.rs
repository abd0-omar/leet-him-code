pub fn min_swaps(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let count_ones = nums.iter().filter(|&x| x == &1).count();

    let mut l = 0;

    let mut cur_window_ones = 0;
    let mut max_window_ones = 0;

    for r in 0..n * 2 {
        if nums[r % n] == 1 {
            cur_window_ones += 1;
        }

        if r - l + 1 > count_ones {
            cur_window_ones -= nums[l % n];
            l += 1;
        }

        max_window_ones = max_window_ones.max(cur_window_ones);
    }
    count_ones as i32 - max_window_ones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![0, 1, 1, 1, 0, 0, 1, 1, 0];
        let output = 2;
        let result = min_swaps(nums);
        assert_eq!(result, output);
    }
}
