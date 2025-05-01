// https://leetcode.com/problems/maximum-number-of-tasks-you-can-assign/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_task_assign(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32,
    ) -> i32 {
        tasks.sort_unstable();
        workers.sort_unstable();
        let (n, m) = (tasks.len(), workers.len());
        let mut l = 1;
        let mut r = n.min(m);
        let mut ans = -1;

        while l <= r {
            let mid = l + (r - l) / 2;
            if possible(&tasks, &workers, pills, strength, mid) {
                ans = mid as i32;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        ans
    }
}

fn possible(tasks: &[i32], workers: &[i32], mut pills: i32, strength: i32, mid: usize) -> bool {
    use std::collections::VecDeque;

    let mut q = VecDeque::new();
    for t in 0..mid {
        q.push_back(tasks[t]);
    }

    for i in (0..mid).rev() {
        let w = workers[workers.len() - mid + i];

        if let Some(&task) = q.back() {
            if w >= task {
                q.pop_back();
                continue;
            }
        }

        if pills == 0 {
            return false;
        }

        if let Some(&task) = q.front() {
            if w + strength >= task {
                q.pop_front();
                pills -= 1;
            } else {
                return false;
            }
        } else {
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
        let tasks = vec![3, 2, 1];
        let workers = vec![0, 3, 3];
        let pills = 1;
        let strength = 1;
        let output = 3;
        let result = Solution::max_task_assign(tasks, workers, pills, strength);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let tasks = vec![5, 4];
        let workers = vec![0, 0, 0];
        let pills = 1;
        let strength = 5;
        let output = 1;
        let result = Solution::max_task_assign(tasks, workers, pills, strength);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let tasks = vec![10, 15, 30];
        let workers = vec![0, 10, 10, 10, 10];
        let pills = 3;
        let strength = 10;
        let output = 2;
        let result = Solution::max_task_assign(tasks, workers, pills, strength);
        assert_eq!(result, output);
    }
}
