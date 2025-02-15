// https://leetcode.com/problems/find-the-punishment-number-of-an-integer/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let mut result = 0;

        for i in 1..=n {
            let i_squared = (i * i).to_string();
            let mut partitions = Vec::new();
            partition(i, &i_squared, Vec::new(), &mut partitions);

            for number_segment in partitions {
                let possible_sum: i32 = number_segment
                    .iter()
                    .map(|part| part.parse::<i32>().unwrap())
                    .sum();
                if possible_sum == i {
                    result += i * i;
                    break;
                }
            }
        }
        result
    }
}

fn partition(i: i32, i_squared: &str, mut cur: Vec<String>, result: &mut Vec<Vec<String>>) {
    if i_squared.is_empty() {
        result.push(cur.clone());
        return;
    }
    for j in 1..=i_squared.len() {
        let part = &i_squared[0..j];
        cur.push(part.to_string());
        partition(i, &i_squared[j..], cur.clone(), result);
        cur.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 10;
        let output = 182;
        let result = Solution::punishment_number(n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 37;
        let output = 1478;
        let result = Solution::punishment_number(n);
        assert_eq!(result, output);
    }
}
