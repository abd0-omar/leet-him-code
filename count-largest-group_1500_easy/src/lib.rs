// https://leetcode.com/problems/count-largest-group/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        // sum | [number]
        let mut group = std::collections::HashMap::new();
        let mut largest_size = 0;
        for num in 1..=n {
            let sum = sum_num(num);
            let group_nums = group.entry(sum).or_insert(Vec::new());
            group_nums.push(num);
            largest_size = largest_size.max(group_nums.len())
        }
        // dbg!(&group);
        group
            .iter()
            .filter(|(_, g)| g.len() == largest_size)
            .count() as i32
    }
}

fn sum_num(num: i32) -> u32 {
    let num = num.to_string();
    let mut cur_sum = 0;
    for digit in num.chars() {
        let digit = digit.to_digit(10).unwrap();
        cur_sum += digit;
    }
    cur_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 13;
        let output = 4;
        let result = Solution::count_largest_group(n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 2;
        let output = 2;
        let result = Solution::count_largest_group(n);
        assert_eq!(result, output);
    }
}
