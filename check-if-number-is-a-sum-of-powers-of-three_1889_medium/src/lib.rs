// https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        // the obvious solution that came to my mind is backtracking
        // ofcourse there is a tedious math one, that is easy to implement but
        // hard to come up with

        // surprisingly the backtracking solution worked
        backtrack(n as i64, 0, 0)
    }
}

fn backtrack(n: i64, power: u32, result: i64) -> bool {
    if result == n {
        return true;
    }

    if result > n || power as i64 >= n || power > 20 {
        return false;
    }

    let new_result = result + 3i64.pow(power);
    let pick = backtrack(n, power + 1, new_result);

    let leave = backtrack(n, power + 1, result);
    // other way with loops
    //   for i in power..=20 {
    //         let new_result = result + 3i64.pow(i);
    //         if backtrack(n, i + 1, new_result) {
    //             return true;
    //         }
    //     }
    pick || leave
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 12;
        let output = true;
        let result = Solution::check_powers_of_three(n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 91;
        let output = true;
        let result = Solution::check_powers_of_three(n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let n = 21;
        let output = false;
        let result = Solution::check_powers_of_three(n);
        assert_eq!(result, output);
    }
}
