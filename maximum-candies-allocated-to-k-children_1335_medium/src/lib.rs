// https://leetcode.com/problems/maximum-candies-allocated-to-k-children/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let (mut l, mut r) = (1, *candies.iter().max().unwrap());
        let mut ans = 0;
        while l <= r {
            let mid = l + (r - l) / 2;
            dbg!(mid);
            if possible(&candies, mid, k) {
                l = mid + 1;
                ans = mid;
                dbg!("possible");
            } else {
                dbg!("impossible");
                if let (_, true) = mid.overflowing_sub(1) {
                    break;
                }
                r = mid - 1;
            }
        }
        ans
    }
}

fn possible(candies: &[i32], candidate: i32, k: i64) -> bool {
    let mut cur_count = 0;
    for &candy in candies {
        let count = candy as i64 / candidate as i64;
        cur_count += count;
    }
    dbg!(&cur_count);
    cur_count >= k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let candies = vec![5, 8, 6];
        let k = 3;
        let output = 5;
        let result = Solution::maximum_candies(candies, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let candies = vec![2, 5];
        let k = 11;
        let output = 0;
        let result = Solution::maximum_candies(candies, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let candies = vec![4, 7, 5];
        let k = 4;
        let output = 3;
        let result = Solution::maximum_candies(candies, k);
        assert_eq!(result, output);
    }
}
