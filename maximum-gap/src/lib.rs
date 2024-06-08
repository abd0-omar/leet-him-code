pub fn maximum_gap(nums: Vec<i32>) -> i32 {
    // O(nlogn) time
    // O(n) space
    // count sort won't work because input too big
    // radix or bucket sort can work here
    let mut heap = std::collections::BinaryHeap::new();
    let mut max = 0;
    for num in nums {
        heap.push(num);
    }

    while let Some(cur_max_num) = heap.pop() {
        if let Some(cur_second_max_num) = heap.peek() {
            max = max.max(cur_max_num - cur_second_max_num);
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![3, 6, 9, 1];
        let output = 3;
        // Explanation: The sorted form of the array is [1,3,6,9], either (3,6) or (6,9) has the maximum difference 3.
        let result = maximum_gap(nums);
        assert_eq!(result, output);
    }
}
