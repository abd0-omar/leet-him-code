// https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut l = 1;
        let mut r = *quantities.iter().max().unwrap();
        let mut ans = 0;

        while l <= r {
            let mid = l + (r - l) / 2;
            if Self::possible(&quantities, n, mid) {
                ans = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        ans
    }

    fn possible(quantities: &[i32], n: i32, distribute_on_stores: i32) -> bool {
        let mut y = 0;
        for &q in quantities {
            let division = (q as f32 / distribute_on_stores as f32).ceil() as i32;
            y += division;
            if y > n {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 6;
        let quantities = vec![11, 6];
        let output = 3;
        let result = Solution::minimized_maximum(n, quantities);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 7;
        let quantities = vec![15, 10, 10];
        let output = 5;
        let result = Solution::minimized_maximum(n, quantities);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let n = 1;
        let quantities = vec![100000];
        let output = 100000;
        let result = Solution::minimized_maximum(n, quantities);
        assert_eq!(result, output);
    }
}
