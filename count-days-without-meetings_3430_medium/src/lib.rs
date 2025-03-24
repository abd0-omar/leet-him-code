// https://leetcode.com/problems/count-days-without-meetings/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        // // interval problems always start with sorting
        // // we could also utilize a diff array to help us prevent recomputing
        // // ranges, sorting would help in this matter anyway but let's try
        // // meetings.sort_unstable();
        // let days = days as usize;

        // // for meeting in &meetings {
        // //     let start = meeting[0];
        // //     let end = meeting[1];
        // // }

        // // "+ 2" because we are 1 based "+ 1" and we need extra one for
        // // diff_array "+ 1"
        // let mut diff_array = vec![0; days + 2];
        // for meeting in &meetings {
        //     let start = meeting[0] as usize;
        //     let end = meeting[1] as usize;
        //     // dbg!(&diff_array);
        //     diff_array[start] += 1;
        //     // dbg!(&diff_array);
        //     diff_array[end + 1] -= 1;
        //     // dbg!(&diff_array);
        // }

        // // prefix_sum the diff_array
        // for i in 1..days + 2 {
        //     diff_array[i] += diff_array[i - 1];
        // }
        // // dbg!(&diff_array);
        // let mut result = 0;
        // for diff in diff_array.into_iter().skip(1).take(days) {
        //     if diff <= 0 {
        //         result += 1;
        //     }
        //     // dbg!(&diff);
        // }
        // result
        // it got TLE because days was 10^9
        // sorting and merging it is
        meetings.sort_unstable();
        let mut merged = vec![meetings[0].clone()];
        for meeting in meetings.into_iter().skip(1) {
            let last = merged.last_mut().unwrap();
            dbg!(&meeting[0]);
            dbg!(&last[1]);
            if meeting[0] <= last[1] + 1 {
                last[1] = last[1].max(meeting[1])
            } else {
                merged.push(meeting);
            }
        }
        dbg!(&merged);
        let meeting_days = merged
            .into_iter()
            .map(|meeting| meeting[1] - meeting[0] + 1)
            .sum::<i32>();
        days - meeting_days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let days = 10;
        let meetings = vec![vec![5, 7], vec![1, 3], vec![9, 10]];
        let output = 2;
        let result = Solution::count_days(days, meetings);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let days = 5;
        let meetings = vec![vec![2, 4], vec![1, 3]];
        let output = 1;
        let result = Solution::count_days(days, meetings);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let days = 6;
        let meetings = vec![vec![1, 6]];
        let output = 0;
        let result = Solution::count_days(days, meetings);
        assert_eq!(result, output);
    }
}
