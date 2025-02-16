use std::collections::HashSet;

// https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let mut result = vec![0; (n * 2 - 1) as usize];
        let mut used = HashSet::new();
        backtrack(n, &mut result, &mut used, 0);
        return result;
    }
}

fn backtrack(n: i32, result: &mut Vec<i32>, used: &mut HashSet<i32>, idx: usize) -> bool {
    if idx == result.len() {
        return true;
    }

    if result[idx] != 0 {
        return backtrack(n, result, used, idx + 1);
    }

    for num in (1..=n).rev() {
        if used.contains(&num) {
            continue;
        }

        if num == 1 {
            result[idx] = 1;
            used.insert(num);
            if backtrack(n, result, used, idx + 1) {
                return true;
            }
            used.remove(&num);
            result[idx] = 0;
        } else {
            let second_idx = idx + num as usize;

            if second_idx < result.len() && result[second_idx] == 0 {
                result[idx] = num;
                result[second_idx] = num;
                used.insert(num);

                if backtrack(n, result, used, idx + 1) {
                    return true;
                }

                result[idx] = 0;
                result[second_idx] = 0;
                used.remove(&num);
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 3;
        let output = vec![3, 1, 2, 3, 2];
        let result = Solution::construct_distanced_sequence(n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 5;
        let output = vec![5, 3, 1, 4, 3, 5, 2, 4, 2];
        let result = Solution::construct_distanced_sequence(n);
        assert_eq!(result, output);
    }
}
