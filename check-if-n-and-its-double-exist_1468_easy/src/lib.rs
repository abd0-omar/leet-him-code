// https://leetcode.com/problems/check-if-n-and-its-double-exist/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_if_exist(mut arr: Vec<i32>) -> bool {
        // either hashmap O(n) but memory O(n)
        // or
        // binary search O(nlogn) and memory O(1)
        // speed wins
        // let mut hs = std::collections::HashSet::new();
        // for element in arr {
        //     if hs.contains(&(element * 2)) {
        //         return true;
        //     }
        //     if element % 2 == 0 && hs.contains(&(element / 2)) {
        //         return true;
        //     }
        //     hs.insert(element);
        // }
        // false
        arr.sort_unstable();
        for (idx, element) in arr.iter().enumerate() {
            if let Ok(ans) = arr.binary_search(&(element * 2)) {
                if ans != idx {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let arr = vec![10, 2, 5, 3];
        let output = true;
        let result = Solution::check_if_exist(arr);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let arr = vec![3, 1, 7, 11];
        let output = false;
        let result = Solution::check_if_exist(arr);
        assert_eq!(result, output);
    }
}
