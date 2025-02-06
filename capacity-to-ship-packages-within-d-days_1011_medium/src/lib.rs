#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        // binary search on answer (ship capacity)
        let mut l = *weights.iter().max().unwrap();
        let mut r = weights.iter().sum();
        let mut ans = -1;
        while l <= r {
            let mid = l + (r - l) / 2;
            if possible(mid, days, &weights) {
                r = mid - 1;
                ans = mid;
            } else {
                l = mid + 1;
            }
        }
        // possible(3, days, &weights);
        ans
    }
}

fn possible(candidate_capcity: i32, days: i32, weights: &[i32]) -> bool {
    let mut cur_weight = 0;
    let mut dayz = 1;
    for &weight in weights {
        cur_weight += weight;
        if cur_weight > candidate_capcity {
            cur_weight = weight;
            dayz += 1;
        }
    }
    dbg!(&cur_weight);
    dbg!(&candidate_capcity);
    dbg!(&dayz);
    dayz <= days
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let weights = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let days = 5;
        let output = 15;
        let result = Solution::ship_within_days(weights, days);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let weights = vec![3, 2, 2, 4, 1, 4];
        let days = 3;
        let output = 6;
        let result = Solution::ship_within_days(weights, days);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let weights = vec![1, 2, 3, 1, 1];
        let days = 4;
        let output = 3;
        let result = Solution::ship_within_days(weights, days);
        assert_eq!(result, output);
    }
}
