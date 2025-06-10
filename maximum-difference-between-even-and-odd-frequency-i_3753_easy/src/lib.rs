// https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut freq = [None; 26];
        for c in s.chars() {
            freq[(c as u8 - b'a') as usize] = match freq[(c as u8 - b'a') as usize] {
                Some(count) => Some(count + 1),
                None => Some(1),
            };
        }
        let mut max_odd = None;
        let mut min_even = None;
        for i in 0..26 {
            if let Some(count) = freq[i] {
                if count % 2 == 0 {
                    if let Some(min) = min_even {
                        if min > count {
                            min_even = Some(count);
                        }
                    } else {
                        min_even = Some(count);
                    }
                } else {
                    if let Some(max) = max_odd {
                        if max < count {
                            max_odd = Some(count);
                        }
                    } else {
                        max_odd = Some(count);
                    }
                }
            }
        }
        max_odd.unwrap() - min_even.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "aaaaabbc".to_string();
        let output = 3;
        let result = Solution::max_difference(s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "abcabcab".to_string();
        let output = 1;
        let result = Solution::max_difference(s);
        assert_eq!(result, output);
    }
}
