// https://leetcode.com/problems/total-characters-in-string-after-transformations-ii/
#[allow(dead_code)]
struct Solution;

const MOD: i64 = 1_000_000_007;
const L: usize = 26;

struct Mat {
    a: [[i64; L]; L],
}

impl Mat {
    fn new() -> Self {
        Mat { a: [[0; L]; L] }
    }

    fn copy_from(&mut self, other: &Mat) {
        for i in 0..L {
            for j in 0..L {
                self.a[i][j] = other.a[i][j];
            }
        }
    }

    fn mul(&self, other: &Mat) -> Mat {
        let mut result = Mat::new();
        for i in 0..L {
            for j in 0..L {
                for k in 0..L {
                    result.a[i][j] = (result.a[i][j] + self.a[i][k] * other.a[k][j]) % MOD;
                }
            }
        }
        result
    }
}

fn identity_matrix() -> Mat {
    let mut m = Mat::new();
    for i in 0..L {
        m.a[i][i] = 1;
    }
    m
}

fn quickmul(x: &Mat, mut y: i32) -> Mat {
    let mut ans = identity_matrix();
    let mut cur = Mat::new();
    cur.copy_from(x);
    while y > 0 {
        if y & 1 == 1 {
            ans = ans.mul(&cur);
        }
        cur = cur.mul(&cur);
        y >>= 1;
    }
    ans
}

#[allow(dead_code)]
impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let mut transition_matrix = Mat::new();
        for i in 0..L {
            for j in 1..=nums[i] as usize {
                transition_matrix.a[(i + j) % L][i] = 1;
            }
        }

        let res = quickmul(&transition_matrix, t);
        let mut f = [0; L];
        for ch in s.chars() {
            f[(ch as u8 - b'a') as usize] += 1;
        }
        let mut ans: i64 = 0;
        for i in 0..L {
            for j in 0..L {
                ans = (ans + res.a[i][j] * f[j]) % MOD;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "abcyy".to_string();
        let t = 2;
        let nums = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
        ];
        let output = 7;
        let result = Solution::length_after_transformations(s, t, nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "azbk".to_string();
        let t = 1;
        let nums = vec![
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        ];
        let output = 8;
        let result = Solution::length_after_transformations(s, t, nums);
        assert_eq!(result, output);
    }
}
