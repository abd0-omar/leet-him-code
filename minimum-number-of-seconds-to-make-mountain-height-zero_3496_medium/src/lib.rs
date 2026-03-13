// https://leetcode.com/problems/minimum-number-of-seconds-to-make-mountain-height-zero/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut l = 0;
        let mut r = i64::MAX;
        let mut ans = r;

        while l <= r {
            let mid = l + (r - l) / 2;

            if possible(mountain_height, &worker_times, mid) {
                ans = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }

        ans
    }
}

fn possible(mountain_height: i32, worker_times: &[i32], mid: i64) -> bool {
    let mut total_reduced = 0i64;
    let target = mountain_height as i64;

    for &worker_time in worker_times {
        let wt = worker_time as i64;
        let mut count = 0;
        let mut next_task_cost = wt;
        let mut time_spent = 0;

        while time_spent + next_task_cost <= mid {
            time_spent += next_task_cost;
            count += 1;
            next_task_cost = wt * (count + 1);

            if total_reduced + count >= target {
                return true;
            }
        }
        total_reduced += count;
    }

    total_reduced >= target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let mountain_height = 4;
        let worker_times = vec![2, 1, 1];
        let output = 3;
        let result = Solution::min_number_of_seconds(mountain_height, worker_times);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let mountain_height = 10;
        let worker_times = vec![3, 2, 2, 4];
        let output = 12;
        let result = Solution::min_number_of_seconds(mountain_height, worker_times);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let mountain_height = 5;
        let worker_times = vec![1];
        let output = 15;
        let result = Solution::min_number_of_seconds(mountain_height, worker_times);
        assert_eq!(result, output);
    }
}
