// https://leetcode.com/problems/minimize-xor/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        // Make result as similar as possible to num1
        let cnt1 = num1.count_ones() as i32;
        let cnt2 = num2.count_ones() as i32;

        // Convert num1 to a fixed-size bit vector (31 bits)
        let bits1 = {
            // really cool trick
            let binary_string = format!("{:031b}", num1); // Pad with leading zeros to 31 bits
            binary_string.chars().collect::<Vec<char>>()
        };

        let mut result_bits = vec!['0'; 31];

        if cnt1 >= cnt2 {
            // If num1 has more or equal set bits, set the MSBs of result_bits to match bits1
            let mut cnt = cnt2;
            for i in 0..31 {
                if cnt == 0 {
                    break;
                }
                if bits1[i] == '1' {
                    result_bits[i] = '1';
                    cnt -= 1;
                }
            }
        } else {
            // If num1 has fewer set bits, set the MSBs of result_bits to match bits1 first
            let mut cnt = cnt1;
            for i in 0..31 {
                if cnt == 0 {
                    break;
                }
                if bits1[i] == '1' {
                    result_bits[i] = '1';
                    cnt -= 1;
                }
            }

            // Then, set the remaining 1s in the LSBs of result_bits
            let mut remaining = cnt2 - cnt1;
            for i in (0..31).rev() {
                if remaining == 0 {
                    break;
                }
                if result_bits[i] == '0' {
                    result_bits[i] = '1';
                    remaining -= 1;
                }
            }
        }

        // Convert the result_bits vector back to an integer
        i32::from_str_radix(&result_bits.iter().collect::<String>(), 2).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let num1 = 3;
        let num2 = 5;
        let output = 3;
        let result = Solution::minimize_xor(num1, num2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let num1 = 1;
        let num2 = 12;
        let output = 3;
        let result = Solution::minimize_xor(num1, num2);
        assert_eq!(result, output);
    }
}
