pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
    // generate all pairs in O(n^2)
    // get the kths smallest with a pq
    //
    // maybe sorting, idk why
    // use std::collections::BinaryHeap;
    // let n = nums.len();
    // let mut distances = BinaryHeap::new();
    //
    // for i in 0..n {
    //     for j in i + 1..n {
    //         distances.push((nums[i] - nums[j]).abs());
    //     }
    // }
    // let sort_vec = distances.into_sorted_vec();
    //
    // sort_vec[k as usize - 1]
    // tle

    // sorting + binary search
    // TT ,FF
    // 1, 1, 3
    // k = 1
    // T, F, F, F
    // most right T
    // the solution is between 0, and the max element
    // when we have our mid count how many pairs that are less than or equal to k
    // 0, 2, 2
    // <= 2 (mid)
    nums.sort();
    let mut l = 0;
    let mut r = nums[nums.len() - 1] - nums[0];
    let mut ans = -1;

    while l <= r {
        let mid = l + (r - l) / 2;
        if count_pairs_with_distance_less_or_equal(&nums, mid) < k {
            l = mid + 1;
        } else {
            r = mid - 1;
            ans = mid;
        }
    }

    ans
}
fn count_pairs_with_distance_less_or_equal(nums: &Vec<i32>, dist: i32) -> i32 {
    // two pointers
    let mut count = 0;
    let mut j = 0;
    for i in 0..nums.len() {
        while j < nums.len() && nums[j] - nums[i] <= dist {
            j += 1;
        }
        count += (j - i - 1) as i32;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 3, 1];
        let k = 1;
        let output = 0;
        // Explanation: Here are all the pairs:
        // (1,3) -> 2
        // (1,1) -> 0
        // (3,1) -> 2
        // Then the 1st smallest distance pair is (1,1), and its distance is 0.
        let result = smallest_distance_pair(nums, k);
        assert_eq!(result, output);
    }
}
