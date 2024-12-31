// https://leetcode.com/problems/minimum-cost-for-tickets/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut memory = vec![None; days.len() + 1];
        dp(&days, &costs, 0, &mut memory)
    }
}

fn dp(days: &[i32], costs: &[i32], i: usize, memory: &mut Vec<Option<i32>>) -> i32 {
    if i == days.len() {
        return 0;
    }

    if let Some(ret) = memory[i] {
        return ret;
    }

    let take_one = dp(days, costs, i + 1, memory) + costs[0];
    let take_seven = {
        let mut j = i;
        loop {
            if j >= days.len() || days[i] + 7 <= days[j] {
                break;
            }
            j += 1;
        }
        dp(days, costs, j, memory) + costs[1]
    };
    let take_thirty = {
        let mut j = i;
        loop {
            if j >= days.len() || days[i] + 30 <= days[j] {
                break;
            }
            j += 1;
        }
        dp(days, costs, j, memory) + costs[2]
    };
    let result = take_seven.min(take_thirty.min(take_one));
    memory[i] = Some(result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let days = vec![1, 4, 6, 7, 8, 20];
        let costs = vec![2, 7, 15];
        let output = 11;
        let result = Solution::mincost_tickets(days, costs);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31];
        let costs = vec![2, 7, 15];
        let output = 17;
        let result = Solution::mincost_tickets(days, costs);
        assert_eq!(result, output);
    }
}
