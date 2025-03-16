// https://leetcode.com/problems/minimum-time-to-repair-cars/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let (mut l, mut r) = (1i64, i64::MAX);
        let mut ans = -1;
        while l <= r {
            let mid = l + (r - l) / 2;
            // we want first T
            if possible(&ranks, cars, mid) {
                ans = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        ans
    }
}

fn possible(ranks: &[i32], cars: i32, candidate_time: i64) -> bool {
    let mut total_cars_fixed = 0;
    // break if cur_cars_fixed == cars
    for &rank in ranks {
        let mut cur_cars_fixed = 0;
        for i in 1..=cars as i64 {
            if rank as i64 * i * i > candidate_time {
                break;
            }
            cur_cars_fixed += 1;
        }
        total_cars_fixed += cur_cars_fixed;
        if total_cars_fixed >= cars {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let ranks = vec![4, 2, 3, 1];
        let cars = 10;
        let output = 16;
        let result = Solution::repair_cars(ranks, cars);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let ranks = vec![5, 1, 8];
        let cars = 6;
        let output = 16;
        let result = Solution::repair_cars(ranks, cars);
        assert_eq!(result, output);
    }
}
