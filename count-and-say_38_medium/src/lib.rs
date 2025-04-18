// https://leetcode.com/problems/count-and-say/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut cur_string = vec!['1'];
        // count, digit
        for x in 1..n {
            let prev = None;
            let i = 0;
            let cur_result = Vec::new();
            let cur_count = 0;
            // 1211
            // i == 0
            /*
            prev = 1
            count = 1
            i = 1

            211
            result = (1, 1)
            count = 0
            prev = 2

            211
            count = 1
            i += 1

            11
            result = (1, 1), (1, 2)
            count = 0
            prev = 1

            11
            count = 1
            i += 1


            */
            let count_pairs = generate_count_pairs(&cur_string, prev, i, cur_result, cur_count);
            // dbg!(x);
            // dbg!(&count_pairs);
            // dbg!(&cur_string);
            cur_string.clear();
            for (count, digit) in count_pairs {
                cur_string.push(char::from_digit(count as u32, 10).unwrap());
                cur_string.push(digit);
            }
            // dbg!("final");
            // dbg!(&cur_string);
        }
        cur_string.into_iter().collect()
    }
}

fn generate_count_pairs(
    cur_string: &Vec<char>,
    mut prev: Option<char>,
    mut i: usize,
    mut cur_result: Vec<(usize, char)>,
    mut cur_count: usize,
) -> Vec<(usize, char)> {
    while i < cur_string.len() {
        if let Some(p) = prev {
            if cur_string[i] == p {
                while i < cur_string.len() && cur_string[i] == p {
                    cur_count += 1;
                    i += 1;
                }
            } else {
                // // count, digit
                cur_result.push((cur_count, p));
                cur_count = 0;
                prev = Some(cur_string[i]);
                // cur_result.push((cur_count, p));
                // prev = Some(cur_string[i]);
                // cur_count = 0;
            }
        } else {
            prev = Some(cur_string[i]);
            cur_count += 1;
            i += 1;
        }
    }
    if let Some(p) = prev {
        cur_result.push((cur_count, p));
    }

    cur_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 4;
        let output = "1211".to_string();
        let result = Solution::count_and_say(n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let f = generate_count_pairs(&vec!['1', '2', '1', '1'], None, 0, Vec::new(), 0);
        dbg!(&f);
    }
}
