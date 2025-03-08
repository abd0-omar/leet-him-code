// https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks = blocks.as_bytes();
        let k = k as usize;
        let mut count_w = blocks[0..k].iter().filter(|&&color| color == b'W').count() as i32;
        let mut result = count_w;
        let mut st = 0;

        for end in k..blocks.len() {
            if blocks[st] == b'W' {
                count_w -= 1;
            }
            if blocks[end] == b'W' {
                count_w += 1;
            }

            result = result.min(count_w);
            st += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let blocks = "WBBWWBBWBW".to_string();
        let k = 7;
        let output = 3;
        let result = Solution::minimum_recolors(blocks, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let blocks = "WBWBBBW".to_string();
        let k = 2;
        let output = 0;
        let result = Solution::minimum_recolors(blocks, k);
        assert_eq!(result, output);
    }
}
