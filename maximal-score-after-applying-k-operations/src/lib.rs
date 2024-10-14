pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
    // straight up a heap problem,
    // or if you don't want a heap you can sort by insertion sort, as it'll be already mostly
    // sorted
    let mut heap = std::collections::BinaryHeap::new();
    for num in nums {
        heap.push(num);
    }

    let mut score = 0i64;
    for _ in 0..k {
        let max_cur_element = heap.pop().unwrap();
        score += max_cur_element as i64;
        let division_operation = (max_cur_element as f64 / 3.0).ceil() as i32;
        heap.push(division_operation);
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 10, 3, 3, 3];
        let k = 3;
        let output = 17;
        let result = max_kelements(nums, k);
        assert_eq!(result, output);
    }
}
