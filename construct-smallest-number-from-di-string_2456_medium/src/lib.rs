// https://leetcode.com/problems/construct-smallest-number-from-di-string/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut result = Vec::new();
        for i in 1..10u8 {
            let mut used = [false; 10];
            used[i as usize] = true;
            backtrack(0, &mut used, &pattern.as_bytes(), vec![i], &mut result);
        }
        // dbg!(&result);
        result
            .into_iter()
            .map(|num| char::from_digit(num as u32, 10).unwrap())
            .collect()
    }
}

fn backtrack(
    idx: usize,
    used: &mut [bool],
    pattern: &[u8],
    mut cur: Vec<u8>,
    result: &mut Vec<u8>,
) {
    if !result.is_empty() {
        return;
    }

    if idx == pattern.len() {
        *result = cur;
        return;
    }

    let cur_pattern = pattern[idx];
    for next_num in 1..10u8 {
        if used[next_num as usize] {
            continue;
        }

        match cur_pattern {
            b'I' => {
                if next_num <= *cur.last().unwrap() {
                    continue;
                }
            }
            b'D' => {
                if next_num >= *cur.last().unwrap() {
                    continue;
                }
            }
            _ => unreachable!(),
        }

        cur.push(next_num);
        used[next_num as usize] = true;
        backtrack(idx + 1, used, pattern, cur.clone(), result);
        used[next_num as usize] = false;
        cur.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let pattern = "IIIDIDDD".to_string();
        let output = "123549876".to_string();
        let result = Solution::smallest_number(pattern);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let pattern = "DDD".to_string();
        let output = "4321".to_string();
        let result = Solution::smallest_number(pattern);
        assert_eq!(result, output);
    }
}
