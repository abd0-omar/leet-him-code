// https://leetcode.com/problems/maximum-average-pass-ratio/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_average_ratio(mut classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        // like the easy christams gift pq question but with a twist
        let mut pq = std::collections::BinaryHeap::new();
        // (ratio, idx)
        // don't forget to update the index value
        for (idx, class) in classes.iter().enumerate() {
            let numerator = class[0];
            let denominator = class[1];
            let cur_ratio = numerator as f64 / denominator as f64;
            let next_ratio = (numerator as f64 + 1.0) / (denominator as f64 + 1.0);
            // I think there is a better "Mathy" way to do it
            // but I'll continue with this to satisfy my old good brain
            let improvement = next_ratio - cur_ratio;
            let wrapper = HeapWrapper { improvement, idx };
            pq.push(wrapper);
        }
        for _ in 0..extra_students {
            let cur = pq.pop().unwrap();
            classes[cur.idx][0] += 1;
            classes[cur.idx][1] += 1;
            let new_cur_ratio = (classes[cur.idx][0] as f64) / (classes[cur.idx][1] as f64);
            let next_new_cur_ratio =
                (classes[cur.idx][0] as f64 + 1.0) / (classes[cur.idx][1] as f64 + 1.0);
            let improvement = next_new_cur_ratio - new_cur_ratio;
            pq.push(HeapWrapper {
                improvement,
                idx: cur.idx,
            });
        }
        dbg!(&classes);
        let mut sum = 0f64;
        let n = classes.len();
        // calculate the average pass ratio
        for class in classes {
            let numerator = class[0] as f64;
            let denominator = class[1] as f64;
            sum += numerator / denominator;
        }
        // sum / n
        let avg = sum / n as f64;
        avg
    }
}

// custom wrapper becauase can't impl Ord by itself for f64 due to Nan
#[derive(PartialEq)]
struct HeapWrapper {
    improvement: f64,
    idx: usize,
}

impl Eq for HeapWrapper {}

impl PartialOrd for HeapWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.improvement.partial_cmp(&other.improvement)
    }
}

impl Ord for HeapWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.improvement.partial_cmp(&other.improvement).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let classes = vec![vec![1, 2], vec![3, 5], vec![2, 2]];
        let extra_students = 2;
        let output = 0.78333;
        let result = Solution::max_average_ratio(classes, extra_students);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let classes = vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]];
        let extra_students = 4;
        let output = 0.53485;
        let result = Solution::max_average_ratio(classes, extra_students);
        assert_eq!(result, output);
    }
}
