// https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        /*
        16 00010000
        17 00010001
        71 01000111
        62 00111110
        12 00001100
        24 00011000
        14 00001110
        [16, 17, 62, 24] have 1 bit in the 3rd index
        <= 10^7 will guarantee to have at most 24 bits
        adding leading 0 bits won't change the number
        so start from the right most bit and it will become "the zero index"
        */
        let binary_cand: Vec<Vec<char>> = candidates
            .into_iter()
            .map(|x| format!("{:b}", x).chars().rev().collect::<Vec<char>>())
            .collect();

        let mut result = 0;
        for i in 0..24 {
            let mut count = 0;
            for b in &binary_cand {
                if i < b.len() {
                    if b[i] == '1' {
                        count += 1;
                    }
                }
            }
            result = result.max(count);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let candidates = vec![16, 17, 71, 62, 12, 24, 14];
        let output = 4;
        let result = Solution::largest_combination(candidates);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let candidates = vec![8, 8];
        let output = 2;
        let result = Solution::largest_combination(candidates);
        assert_eq!(result, output);
    }
}
