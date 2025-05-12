use std::ops::ControlFlow;

// https://leetcode.com/problems/finding-3-digit-even-numbers/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        use std::collections::HashMap;
        let mut hm = HashMap::with_capacity(digits.len());
        for digit in digits {
            *hm.entry(digit).or_insert(0) += 1;
        }
        for number in 100..1000 {
            let mut temp = number;
            let first_digit = temp % 10;
            temp /= 10;
            let second_digit = temp % 10;
            temp /= 10;
            let third_digit = temp;

            if number % 2 != 0 {
                continue;
            }

            if !hm.contains_key(&first_digit)
                || !hm.contains_key(&second_digit)
                || !hm.contains_key(&third_digit)
            {
                continue;
            }

            if let ControlFlow::Break(_) = adjust_count(&mut hm, first_digit) {
                continue;
            }
            if let ControlFlow::Break(_) = adjust_count(&mut hm, second_digit) {
                *hm.entry(first_digit).or_insert(0) += 1;
                continue;
            }
            if let ControlFlow::Break(_) = adjust_count(&mut hm, third_digit) {
                *hm.entry(first_digit).or_insert(0) += 1;
                *hm.entry(second_digit).or_insert(0) += 1;
                continue;
            }

            result.push(number);

            *hm.entry(first_digit).or_insert(0) += 1;
            *hm.entry(second_digit).or_insert(0) += 1;
            *hm.entry(third_digit).or_insert(0) += 1;
        }
        result
    }
}

fn adjust_count(hm: &mut std::collections::HashMap<i32, i32>, digit: i32) -> ControlFlow<()> {
    if let Some(count) = hm.get_mut(&digit) {
        if *count == 0 {
            return ControlFlow::Break(());
        }
        *count -= 1;
    }
    ControlFlow::Continue(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let digits = vec![2, 1, 3, 0];
        let output = vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320];
        let result = Solution::find_even_numbers(digits);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let digits = vec![2, 2, 8, 8, 2];
        let output = vec![222, 228, 282, 288, 822, 828, 882];
        let result = Solution::find_even_numbers(digits);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let digits = vec![3, 7, 5];
        let output = vec![];
        let result = Solution::find_even_numbers(digits);
        assert_eq!(result, output);
    }
}
