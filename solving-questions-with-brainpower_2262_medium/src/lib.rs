// https://leetcode.com/problems/solving-questions-with-brainpower/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        // take it or leave it pattern
        // second day of eid لييت كود اداني العيدية خلاص
        // مسالة ميديم في اول الشهر، بس سهلة الصراحة can't complain
        let mut memo = std::collections::HashMap::new();
        dp(&questions, 0, &mut memo)
    }
}

fn dp(questions: &[Vec<i32>], idx: usize, memo: &mut std::collections::HashMap<usize, i64>) -> i64 {
    if idx >= questions.len() {
        return 0;
    }

    if let Some(ret) = memo.get(&idx) {
        return *ret;
    }

    let take = {
        let cur_value = questions[idx][0] as i64;
        let cur_to_skip = questions[idx][1] as usize + idx + 1;
        cur_value + dp(questions, cur_to_skip, memo)
    };
    let leave = dp(questions, idx + 1, memo);

    let result = take.max(leave);
    memo.insert(idx, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let questions = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
        let output = 5;
        let result = Solution::most_points(questions);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let questions = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
        let output = 7;
        let result = Solution::most_points(questions);
        assert_eq!(result, output);
    }
}
