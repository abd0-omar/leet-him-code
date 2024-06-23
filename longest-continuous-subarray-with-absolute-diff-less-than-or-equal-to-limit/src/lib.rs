pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    // [10, 1, 2, 4, 7, 2]
    // highest no. - lowest no. must be <= limit
    // input is too big so probably brute force won't work
    // [10, 1, 2, 4, 7, 2]
    //  i   j
    //  i      j
    //  i         j
    // subarray_len -> j - i + 1
    // then check with another nested loop k to get the highest and the lowest
    // so overall O(n^3)
    // try prefix_sum
    // [10, 1, 2, 4, 7, 2]
    // [0, 10, 11, 13, 17, 24, 26]
    // limit -> 5
    // I don't see it working
    // [10, 1, 2, 4, 7, 2]
    // hm prefix_sum (later)
    // prefix_max
    // [10, 10, 10, 10, 10, 10]
    // [10, 7, 7, 7, 7, 2]
    // prefix_min
    // [10, 1, 1, 1, 1, 1]
    // [1, 1, 2, 2, 2, 2]
    // it could work
    // [10, 7, 7, 7, 7, 2]
    // [1, 1, 2, 2, 2, 2]
    // let's try hm prefix_sum (later)
    //
    //  0   1  2  3  4  5
    // [10, 1, 2, 4, 7, 2]
    // [10, 1, 2, 4, 7, 2]
    //        ji
    //      j  i
    //   j     i
    // when iterating hold the max_so_far and the min_so_far
    // stop when the number is condition > limit and get the len as a potential answer
    // let n = nums.len();
    // let mut longest_subarray = 0;
    // for i in 0..n {
    //     let mut max_so_far = nums[i];
    //     let mut min_so_far = nums[i];
    //     for j in (0..=i).rev() {
    //         max_so_far = max_so_far.max(nums[j]);
    //         min_so_far = min_so_far.min(nums[j]);
    //         if max_so_far - min_so_far > limit {
    //             continue;
    //         }
    //         // dbg!(i);
    //         // dbg!(j);
    //         // println!("---");
    //         longest_subarray = longest_subarray.max(i as i32 - j as i32 + 1);
    //     }
    // }

    // longest_subarray
    // TLE
    // two pointers
    // let n = nums.len();
    // let mut longest_subarray = 0;
    // let mut st = 0;
    // let mut max_heap = std::collections::BinaryHeap::new();
    // let mut min_heap = std::collections::BinaryHeap::new();
    // use std::cmp::Reverse;
    // for end in 0..n {
    //     max_heap.push(nums[end]);
    //     min_heap.push(Reverse(nums[end]));
    //     dbg!(end);
    //     dbg!(st);
    //     dbg!(&max_heap);
    //     dbg!(&min_heap);
    //     while max_heap.peek().unwrap() - min_heap.peek().unwrap().0 > limit {
    //         // remove 10 from max_val
    //         if nums[st] > nums[end] {
    //             // pop from max_heap
    //             max_heap.pop();
    //         } else {
    //             min_heap.pop();
    //         }
    //         st += 1;
    //     }
    //     longest_subarray = longest_subarray.max(end - st + 1);
    // }
    // longest_subarray as _
    use std::collections::VecDeque;
    let mut max_deque: VecDeque<i32> = VecDeque::new();
    let mut min_deque: VecDeque<i32> = VecDeque::new();
    let mut left = 0;
    let mut max_length = 0;

    for right in 0..nums.len() {
        // Maintain the maxDeque in decreasing order
        while let Some(&back) = max_deque.back() {
            if back < nums[right] {
                max_deque.pop_back();
            } else {
                break;
            }
        }
        max_deque.push_back(nums[right]);

        // Maintain the minDeque in increasing order
        while let Some(&back) = min_deque.back() {
            if back > nums[right] {
                min_deque.pop_back();
            } else {
                break;
            }
        }
        min_deque.push_back(nums[right]);

        println!("{:?}", &max_deque);
        println!("{:?}", &min_deque);
        // Check if the current window exceeds the limit
        while max_deque.front().unwrap() - min_deque.front().unwrap() > limit {
            // Remove the elements that are out of the current window
            if max_deque.front() == Some(&nums[left]) {
                max_deque.pop_front();
            }
            if min_deque.front() == Some(&nums[left]) {
                min_deque.pop_front();
            }
            left += 1;
        }

        max_length = max_length.max(right - left + 1);
    }

    max_length as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![10, 1, 2, 4, 7, 2];
        let limit = 5;
        let output = 4;
        // Explanation: The subarray [2,4,7,2] is the longest since the maximum absolute diff is |2-7| = 5 <= 5.
        let result = longest_subarray(nums, limit);
        assert_eq!(result, output);
    }
}
