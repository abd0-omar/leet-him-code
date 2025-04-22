// https://leetcode.com/problems/count-the-number-of-ideal-arrays/
#[allow(dead_code)]
struct Solution;

const MOD: u64 = 10_u64.pow(9) + 7;

fn pow(x: u64, n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let y = pow(x, n / 2);
    let y = (y * y) % MOD;
    if n % 2 == 0 { y } else { (y * x) % MOD }
}

#[allow(dead_code)]
impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        let n = n as u64;
        let m = max_value as u64;
        let l = (m as f64).sqrt() as u64;
        let primes: Vec<_> = (2..=l)
            .fold(vec![true; m as usize + 1], |mut ps, p| {
                for q in (p * p..=m).step_by(p as usize) {
                    ps[q as usize] = false;
                }
                ps
            })
            .into_iter()
            .enumerate()
            .skip(2)
            .filter_map(|(i, p)| if p { Some(i as u64) } else { None })
            .collect();
        1 + (2..=m)
            .map(|mut v| {
                let mut result = 1;
                let mut ps = primes.iter();
                while v > 1 {
                    let p = ps.next().unwrap();
                    let mut e = 0;
                    while v % p == 0 {
                        e += 1;
                        v /= p;
                    }
                    if e > 0 {
                        result = (1..=e).fold(result, |result, i| {
                            (((result * (n - 1 + i)) % MOD) * pow(i, MOD - 2)) % MOD
                        });
                    }
                }
                result
            })
            .fold(0, |s, v| (s + v) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 2;
        let max_value = 5;
        let output = 10;
        let result = Solution::ideal_arrays(n, max_value);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 5;
        let max_value = 3;
        let output = 11;
        let result = Solution::ideal_arrays(n, max_value);
        assert_eq!(result, output);
    }
}
