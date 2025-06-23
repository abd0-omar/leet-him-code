#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        let mut digit = [0; 100];
        let mut left = 1;
        let mut count = 0;
        let mut ans = 0i64;
        while count < n {
            let right = left * 10;
            for op in 0..2 {
                // generate i then reverse it and append to i
                for i in left..right {
                    if count == n {
                        break;
                    }
                    let mut combined = i as i64;
                    // 123
                    // 12321 or 123321
                    let mut x = if op == 0 { i / 10 } else { i };
                    while x > 0 {
                        combined = combined * 10 + (x % 10) as i64;
                        x /= 10;
                    }
                    if Self::is_palindrome(combined, k, &mut digit) {
                        count += 1;
                        ans += combined;
                    }
                }
            }
            left = right;
        }
        ans
    }

    fn is_palindrome(mut x: i64, k: i32, digit: &mut [i32]) -> bool {
        let mut length = -1;
        while x > 0 {
            length += 1;
            digit[length as usize] = (x % k as i64) as i32;
            x /= k as i64;
        }
        let mut i = 0;
        let mut j = length;
        while i < j {
            if digit[i as usize] != digit[j as usize] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
