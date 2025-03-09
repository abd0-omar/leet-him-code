// https://leetcode.com/problems/alternating-groups-ii/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_alternating_groups(mut colors: Vec<i32>, k: i32) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let k = k as usize;
        // double the vec for easy circular iteration
        colors.extend_from_within(..k - 1);
        let n = colors.len();
        let mut prev = None;
        let mut result = 0;
        while r < n {
            let cur = colors[r];

            // case only one element
            if r - l == 0 {
                r += 1;
                prev = Some(cur);
                continue;
            }

            if r - l + 1 < k {
                // case move only right cuze it's not of len k yet and it's valid
                if prev.unwrap_or(-1) != cur {
                    prev = Some(cur);
                    r += 1;
                } else {
                    // case if invalid teleport left to the right place
                    l = r;
                    // will go back to the first case
                }
            } else {
                // case len k

                // valid
                if prev.unwrap_or(-1) != cur {
                    result += 1;
                    prev = Some(cur);
                    r += 1;
                    l += 1;
                } else {
                    // case if invalid teleport left to the right place
                    l = r;
                    // will go back to the first case
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let colors = vec![0, 1, 0, 1, 0];
        let k = 3;
        let output = 3;
        let result = Solution::number_of_alternating_groups(colors, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let colors = vec![0, 1, 0, 0, 1, 0, 1];
        let k = 6;
        let output = 2;
        let result = Solution::number_of_alternating_groups(colors, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let colors = vec![1, 1, 0, 1];
        let k = 4;
        let output = 0;
        let result = Solution::number_of_alternating_groups(colors, k);
        assert_eq!(result, output);
    }
}
