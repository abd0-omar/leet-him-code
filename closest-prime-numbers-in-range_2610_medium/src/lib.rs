// https://leetcode.com/problems/closest-prime-numbers-in-range/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let primes = get_primes(left, right);
        let mut result = vec![-1, -1];
        if primes.len() < 2 {
            return result;
        }
        let mut min_diff = i32::MAX;
        for i in 0..primes.len() - 1 {
            let diff = primes[i + 1] - primes[i];
            if diff < min_diff {
                min_diff = diff;
                result[0] = primes[i];
                result[1] = primes[i + 1];
            }
            if diff == 2 {
                break;
            }
        }
        result
    }
}

// Sieve
fn get_primes(left: i32, right: i32) -> Vec<i32> {
    let mut is_prime = vec![true; (right + 1) as usize];
    is_prime[0] = false;
    if right >= 1 {
        is_prime[1] = false;
    }
    for i in 2..=((right as f64).sqrt() as i32) {
        if is_prime[i as usize] {
            let mut j = i * i;
            while j <= right {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }
    let mut primes = Vec::new();
    for i in left..=right {
        if is_prime[i as usize] {
            primes.push(i);
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let left = 10;
        let right = 19;
        let output = vec![11, 13];
        let result = Solution::closest_primes(left, right);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let left = 4;
        let right = 6;
        let output = vec![-1, -1];
        let result = Solution::closest_primes(left, right);
        assert_eq!(result, output);
    }
}
