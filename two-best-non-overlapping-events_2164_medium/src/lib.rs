// https://leetcode.com/problems/two-best-non-overlapping-events/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        // // O(n^2) is trivial
        // let mut result = {
        //     let mut max = events[0][2];
        //     for event in &events {
        //         max = max.max(event[2]);
        //     }
        //     max
        // };
        // for i in 0..events.len() {
        //     let cur_event = &events[i];
        //     for j in 1..events.len() {
        //         let other_event = &events[j];
        //         if cur_event[1] >= other_event[0] && cur_event[0] <= other_event[1] {
        //             continue;
        //         }
        //         result = result.max(cur_event[2] + other_event[2]);
        //     }
        // }
        // result
        // // but of course TLE

        // priority queue (min heap)
        let mut pq: std::collections::BinaryHeap<Reverse<(i32, i32)>> =
            std::collections::BinaryHeap::new();
        use std::cmp::Reverse;
        events.sort_unstable();
        let mut result = 0;
        let mut max_val = 0;
        for event in events {
            while !pq.is_empty() && pq.peek().unwrap().0 .0 < event[0] {
                let Reverse(cur) = pq.pop().unwrap();
                max_val = max_val.max(cur.1);
            }
            result = result.max(event[2] + max_val);
            pq.push(Reverse((event[1], event[2])));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let events = vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]];
        let output = 4;
        let result = Solution::max_two_events(events);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let events = vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]];
        let output = 5;
        let result = Solution::max_two_events(events);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let events = vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]];
        let output = 8;
        let result = Solution::max_two_events(events);
        assert_eq!(result, output);
    }
}
